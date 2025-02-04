use gdk4::Key;
use glib::Propagation;
use gtk4::{
    gdk::{Display, Monitor},
    prelude::*,
    {Align, Application, ApplicationWindow, EventControllerKey, GestureClick, Grid, PropagationPhase},
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use thiserror::Error;

use crate::{
    command_button::CommandButtonParamsError,
    options::Options,
    settings::{self, Settings},
    CommandButton,
};

#[derive(Error, Debug)]
pub enum ApplicationError {
  #[error("Error occurred loading application settings")]
  LoadSettingsError(#[from] settings::LoadSettingsError),

  #[error("Error occurred loading buttons")]
  LoadButtonsError(#[from] CommandButtonParamsError),
}

fn mouse_clicked(gesture: &GestureClick, app: &Application, _: i32, x: f64, y: f64, ) {
  if let Some(widget) = gesture.widget() {
    if widget.is::<ApplicationWindow>() {
      if let Some(target) = widget.pick(x, y, gtk4::PickFlags::DEFAULT) {
        if target.is::<ApplicationWindow>() {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            app.quit();
        }
      }
    }
  }
}

fn key_pressed(controller: &EventControllerKey, keyval: gtk4::gdk::Key) -> bool {
  let Some(widget) = controller.widget() else { return false };
  let Some(window) = widget.downcast_ref::<ApplicationWindow>() else { return false };
  let Some(app) = window.application() else { return false };

  if keyval == Key::Escape {
    app.quit();
    return true;
  }

  let buttons = get_command_buttons(window);

  for button in buttons {
    if let Some(keychar) = keyval.name() {
      if button.keybind().to_string() == keychar {
        button.clicked();
        return true;
      }
    }
  }

  false
}

fn get_command_buttons(window: &ApplicationWindow) -> Vec<CommandButton> {
  let mut buttons = Vec::<CommandButton>::new();
  let mut current = window.first_child().unwrap().first_child();

  while let Some(widget) = current {
      if let Some(button) = widget.downcast_ref::<CommandButton>() {
          buttons.push(button.clone());
      }
      current = widget.next_sibling();
  }

  buttons
}

fn init_new_window(app: &Application, monitor: Option<Monitor>, enable_keyboard: bool) -> ApplicationWindow {
  let new_window= ApplicationWindow::builder()
    .application(app)
    .title("curtains-close")
    .build();
  
  new_window.init_layer_shell();
  new_window.set_layer(Layer::Overlay);
  new_window.set_namespace("curtains-close");
  new_window.set_exclusive_zone(-1);
  new_window.set_anchor(Edge::Left, true);
  new_window.set_anchor(Edge::Right, true);
  new_window.set_anchor(Edge::Top, true);
  new_window.set_anchor(Edge::Bottom, true);

  if let Some(mon) = &monitor {
    new_window.set_monitor(mon);
  }

  if enable_keyboard {
    new_window.set_keyboard_mode(KeyboardMode::Exclusive);
  } else {
    new_window.set_keyboard_mode(KeyboardMode::None);
  }

  let gesture = GestureClick::new();
  let mouse_click_app_clone = app.clone();

  gesture.set_propagation_phase(PropagationPhase::Bubble);
  gesture.set_button(gtk4::gdk::ffi::GDK_BUTTON_PRIMARY as u32);
  gesture.connect_pressed(move |gesture, n_press , x, y |{
    mouse_clicked(&gesture, &mouse_click_app_clone, n_press, x, y);
  });

  new_window.add_controller(gesture);

  let key_controller = EventControllerKey::new();

  key_controller.connect_key_pressed(move |controller, keyval, _keycode, _state | {
    if key_pressed(&controller, keyval) {
      return Propagation::Stop;
    }

    Propagation::Proceed
  });

  new_window.add_controller(key_controller);
  
  new_window
}

fn load_buttons(settings: &Settings, window: &ApplicationWindow) -> Result<(), ApplicationError> {
  let content_grid = Grid::builder()
    .column_spacing(settings.column_spacing as i32)
    .row_spacing(settings.row_spacing as i32)
    .build();

  content_grid.set_halign(Align::Center);
  content_grid.set_valign(Align::Center);

  let mut current_column = 0;
  let mut current_row = 0;

  let app = window.application().unwrap();

  for button_info in settings.buttons.iter() {
    let button_info_clone = button_info.clone();
    
    let button = CommandButton::with_params(
     &app,
     button_info_clone.id, 
     button_info_clone.icon, 
     button_info_clone.text, 
     button_info_clone.keybind.to_string(),
     button_info_clone.command, 
     true, 
     settings.delay_before_closing
    )?;

    content_grid.attach(&button, current_column, current_row, 1, 1);
    current_column += 1;

    if current_column >= settings.buttons_per_row as i32 {
      current_column = 0;
      current_row += 1;
    }
  }

  window.set_child(Some(&content_grid));

  return Ok(())
}

fn get_monitors() -> Vec<Monitor> {
  let display = Display::default().unwrap();
  let monitors = display.monitors();
  let mut monitor_list = Vec::<Monitor>::new();

  for index in 0..monitors.n_items() {
    if let Some(obj) = monitors.item(index) {
      if let Some(mon_ref) = obj.downcast_ref::<Monitor>() {
        let monitor = mon_ref.clone();
        monitor_list.push(monitor);
      }
    }
  }

  monitor_list
}

fn load_windows_on_monitors(focused_window: &ApplicationWindow) {
  let app = focused_window.application().unwrap();
  let display= Display::default().unwrap();
  let surface = focused_window.surface().unwrap();
  let focused_monitor = display.monitor_at_surface(&surface).unwrap();
  let monitors = get_monitors();

  for monitor in monitors {
    if monitor != focused_monitor {
      let new_window = init_new_window(&app, Some(monitor), false);
      new_window.present();
    }
  }
}

pub fn app_main(options: &Options, app: &Application) -> Result<(), ApplicationError> {
  let settings= Settings::load_settings(&options)?;
  let focused_window = init_new_window(&app, None, true);

  focused_window.connect_is_active_notify(|window| {
    load_windows_on_monitors(&window);
  });

  load_buttons(&settings, &focused_window)?;
  focused_window.present();  

  Ok(())
}