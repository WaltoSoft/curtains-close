use glib::ParamSpec;
use glib::Value;
use glib::subclass::prelude::*;
use gtk4::prelude::*;
use gtk4::subclass::button::ButtonImpl;
use gtk4::subclass::widget::WidgetImpl;
use gtk4::{Application, Box, Button};
use gtk4::{subclass::widget::WidgetImplExt, Align, Label, Orientation};
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::{process::Command, thread, time::Duration};
use thiserror::Error;

glib::wrapper! {
  pub struct CommandButton(ObjectSubclass<imp::CommandButton>)
    @extends Button, gtk4::Widget,
    @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget;
}

#[derive(Error, Debug)]
pub enum CommandButtonParamsError {
  #[error("Error occurred building CommandButton from params")]
  WithParamsError(#[from] std::boxed::Box<dyn std::error::Error>),
}

impl CommandButton {
  pub fn with_params<S>(
    app: &Application, 
    widget_name: S, 
    icon: S, 
    text: S, 
    keybind: S, 
    command: S, 
    terminate_on_click: bool, 
    terminate_delay: u32
  ) -> Result<Self, CommandButtonParamsError>
  where 
    S: Into<String> {
    let command_button: Self = glib::Object::new();

    command_button.set_application(app);
    command_button.set_widget_name(&widget_name.into());
    command_button.set_text(text.into());
    command_button.set_icon(icon.into());
    command_button.set_keybind(keybind.into());
    command_button.set_command(command.into());
    command_button.set_terminate_on_click(terminate_on_click);
    command_button.set_terminate_delay(terminate_delay);

    Ok(command_button)
  }

  pub fn clicked(&self) {
    ButtonImpl::clicked(self.imp())
  }

  pub fn app(&self) -> Application {
    self.property_value("app").get::<Application>().unwrap_or_default()
  }

  pub fn icon(&self) -> String {
    self.property_value("icon").get::<String>().unwrap_or_default()
  }

  pub fn text(&self) -> String {
    self.property_value("text").get::<String>().unwrap_or_default()
  }

  pub fn keybind(&self) -> String {
    self.property_value("keybind").get::<String>().unwrap_or_default()
  }

  pub fn command(&self) -> String {
    self
      .property_value("command")
      .get::<String>()
      .unwrap_or_default()
  }

  pub fn terminate_on_click(&self) -> bool {
    self
      .property_value("terminate-on-click")
      .get::<bool>()
      .unwrap_or_default()
  }

  pub fn terminate_delay(&self) -> u32 {
    self
      .property_value("terminate-delay")
      .get::<u32>()
      .unwrap_or_default()
  }

  pub fn set_application(&self, app: &Application) -> &Self {
    self.set_property("app", app);
    self
  }

  pub fn set_icon(&self, icon: String) -> &Self {
    self.set_property("icon", icon);
    self
  }

  pub fn set_text(&self, text: String) -> &Self {
    self.set_property("text", text);
    self
  }

  pub fn set_keybind(&self, keybind: String) -> &Self {
    self.set_property("keybind", keybind);
    self
  }

  pub fn set_command(&self, command: String) -> &Self {
    self.set_property("command", command);
    self
  }

  pub fn set_terminate_on_click(&self, terminate_on_click: bool) -> &Self {
    self.set_property("terminate_on_click", terminate_on_click);
    self
  }

  pub fn set_terminate_delay(&self, terminate_delay: u32) -> &Self {
    self.set_property("terminate_delay", terminate_delay);
    self
  }

  fn execute_command(&self) {
    let cmd = self.command();
    let mut parts = cmd.split_whitespace();

    if let Some(program) = parts.next() {
      let args: Vec<&str> = parts.collect();

      match Command::new(program).args(args).spawn() {
        Ok(_) => {
          if self.terminate_on_click() {
            self.handle_termination();
          }
        }
        Err(e) => eprintln!("Failed to execute command: {}", e),
      }
    }
  }

  fn handle_termination(&self) {
    let delay = self.terminate_delay();
    if delay > 0 {
        thread::sleep(Duration::from_millis(delay as u64));
    }

    self.app().quit();
  }
}

mod imp {
  use super::*;
  
  #[derive(Default)]
  pub struct CommandButton {
    app: RefCell<Option<Application>>,
    icon: RefCell<Option<String>>,
    text: RefCell<Option<String>>,
    keybind: RefCell<Option<String>>,
    command: RefCell<Option<String>>,
    terminate_on_click: RefCell<bool>,
    terminate_delay: RefCell<u32>,
  }

  #[glib::object_subclass]
  impl ObjectSubclass for CommandButton {
    const NAME: &'static str = "CommandButton";
    type Type = super::CommandButton;
    type ParentType = Button;
  }

  impl ObjectImpl for CommandButton {
    fn properties() -> &'static [ParamSpec] {
      static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
        vec![
          glib::ParamSpecObject::builder::<Application>("app").build(),
          glib::ParamSpecString::builder("icon").build(),
          glib::ParamSpecString::builder("text").build(),
          glib::ParamSpecString::builder("keybind").build(),
          glib::ParamSpecString::builder("command").build(),
          glib::ParamSpecBoolean::builder("terminate-on-click").build(),
          glib::ParamSpecUInt::builder("terminate-delay").build()
        ]
      });
      
      PROPERTIES.as_ref()
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
      match pspec.name() {
        "app" => self.app.borrow().to_value(),
        "icon" => self.icon.borrow().to_value(),
        "text" => self.text.borrow().to_value(),
        "keybind" => self.keybind.borrow().to_value(),
        "command" => self.command.borrow().to_value(),
        "terminate-on-click" => self.terminate_on_click.borrow().to_value(),
        "terminate-delay" => self.terminate_delay.borrow().to_value(),
        _ => unimplemented!(),
      }
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
      match pspec.name() {
        "app" => {
          if let Ok(app) = value.get() {
            self.app.replace(Some(app));
          }
        }
        "icon" => {
          if let Ok(icon) = value.get() {
            self.icon.replace(icon);
          }
        },
        "text" => {
          if let Ok(text) = value.get() {
            self.text.replace(text);
          }
        },
        "keybind" => {
          if let Ok(keybind) = value.get() {
            self.keybind.replace(keybind);
          }
        },
        "command" => {
          if let Ok(command) = value.get() {
            self.command.replace(command);
          }
        },
        "terminate-on-click" => {
          if let Ok(terminate_on_click) = value.get() {
            self.terminate_on_click.replace(terminate_on_click);
          }
        },
        "terminate-delay" => {
          if let Ok(terminate_delay) = value.get() {
            self.terminate_delay.replace(terminate_delay);
          }
        },
        _ => unimplemented!(),
      }
    }
  }

  impl WidgetImpl for CommandButton {
    fn realize(&self) {
      self.parent_realize();

      let button = self.obj();
    
      let vbox = Box::builder()
          .orientation(Orientation::Vertical)
          .spacing(10)
          .halign(Align::Center)
          .valign(Align::Center)
          .build();

      let (icon, text) = {
          let icon = self.icon.borrow();
          let text = self.text.borrow();
          (icon.as_deref().unwrap_or("").to_string(), 
          text.as_deref().unwrap_or("").to_string())
      };

      if ! icon.is_empty() {
          let icon_label = Label::builder()
          .label(&icon)
          .css_classes(["button-icon"])
          .build();

          vbox.append(&icon_label);
      }

      if ! text.is_empty() {
        let text_label = Label::builder()
        .label(&text)
        .css_classes(["button-text"])
        .use_markup(true)
        .build();

        vbox.append(&text_label);
      }

      button.add_css_class("button");
      button.set_child(Some(&vbox));
    }  
  }

  impl ButtonImpl for CommandButton {
    fn clicked(&self) {
      let button = self.obj();
      button.execute_command();
    }
  }
}