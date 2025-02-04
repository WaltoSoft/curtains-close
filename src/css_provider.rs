use dirs::home_dir;
use std::{io, panic};
use std::path::Path;
use thiserror::{self, Error};
use gtk4:: {gdk, style_context_add_provider_for_display, CssProvider};
use gdk::Display;

use crate::content_loader::{self, load_content_from_script};
use crate::options::Options;

#[derive(Error, Debug)]
pub enum LoadCSSProviderError {
    #[error("IO error")]
    Io(#[from] io::Error),
    #[error("Error loading CSS provider")]
    CssProviderError(String),
    #[error("Error occurred while loading CSS Content")]
    ContentLoaderError(#[from] content_loader::ContentLoaderError),
    #[error("Error loading default display")]
    Display,
    #[error("No content or path")]
    ContentOrPathNotFound
}

fn set_css_provider(css_content: &str) -> Result<(), LoadCSSProviderError> {
  let provider = CssProvider::new();
  let default_display = Display::default();

  if let Some(display) = default_display {
    panic::catch_unwind(|| {
      provider.load_from_data(css_content);

      style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION
      );
  
    })
    .map_err(|err| {
      let panic_info = if let Some(s) = err.downcast_ref::<&str>() {
          s.to_string()
      } else if let Some(s) = err.downcast_ref::<String>() {
          s.clone()
      } else {
          "Unknown panic".to_string()
      };
      LoadCSSProviderError::CssProviderError(panic_info)
    })?;
  } else {
    return Err(LoadCSSProviderError::Display);
  }

  return Ok(());
}

pub fn add_css_provider(options: &Options) -> Result<(), LoadCSSProviderError> {
  if let Some(css_content) = &options.css_content {
    set_css_provider(&css_content)?;
  }

  if let Some(css_loader_path) = &options.css_loader_path {
    let css_content = load_content_from_script(&css_loader_path)?;
    set_css_provider(&css_content)?;
  }

  if let Some(css_path) = &options.css_path {
    let css_content = std::fs::read_to_string(css_path)?;
    set_css_provider(&css_content)?;
  }

  if let Some(home_dir) = home_dir() {
    let curtains_config_loaderpath= home_dir.join(".config/curtains/close/style.sh");
    if curtains_config_loaderpath.exists() {
      let css_content = load_content_from_script(&curtains_config_loaderpath)?;
      set_css_provider(&css_content)?;
      return Ok(());
    }

    let curtains_config_path = home_dir.join(".config/curtains/close/style.css");
    if curtains_config_path.exists() {
      let css_content = std::fs::read_to_string(curtains_config_path)?;
      set_css_provider(&css_content)?;
      return Ok(());
    }
  }

  let usr_local_path = Path::new("/usr/local/etc/curtains-close/style.css");
  if usr_local_path.exists() {
    let css_content = std::fs::read_to_string(usr_local_path)?;
    set_css_provider(&css_content)?;
    return Ok(());
  }

  let etc_path = Path::new("/etc/curtains-close/style.css");
  if etc_path.exists() {
    let css_content = std::fs::read_to_string(etc_path)?;
    set_css_provider(&css_content)?;
    return Ok(());
  }    

  Ok(())
}