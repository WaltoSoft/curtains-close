use clap::Parser;
use gtk4::prelude::*;
use gtk4::{glib, Application};
use curtains_close::options::Options;
use curtains_close::css_provider;
use curtains_close::application::app_main;

fn main() -> glib::ExitCode {
    let options = Options::parse();

    let app = Application::builder()
      .application_id("com.waltosoft.curtains-close")
      .build();
  
    let add_css_provider_options_clone = options.clone();
    app.connect_startup(move |_| {
      if let Err(e) = css_provider::add_css_provider(&add_css_provider_options_clone) {
        eprintln!("Error loading CSS provider: {:?}", e);
        std::process::exit(1);
      }
    });
  
    let app_main_options_clone = options.clone();
    app.connect_activate(move |app| {
      if let Err(e) = app_main(&app_main_options_clone, app) {
        eprintln!("Error occurred running application: {:?}", e);
        app.quit();
      } 
    });
  
    app.run_with_args(&[] as &[&str])
  
}