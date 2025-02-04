use dirs;
use std::{fs, io, path::Path};
use serde::Deserialize;
use thiserror::Error;

use crate::{content_loader::{load_content_from_script, ContentLoaderError}, options::Options};

#[derive(Clone, Debug, Deserialize)]
struct RawSettings {
  pub buttons_per_row: Option<u32>,
  pub column_spacing: Option<u32>,
  pub row_spacing: Option<u32>,
  pub delay_before_closing: Option<u32>,
  pub buttons: Option<Vec<ButtonInfo>>
}

#[derive(Clone, Debug)]
pub struct Settings {
  pub buttons_per_row: u32,
  pub column_spacing: u32,
  pub row_spacing: u32,
  pub delay_before_closing: u32,
  pub buttons: Vec<ButtonInfo>
}


#[derive(Clone, Debug, Deserialize)]
pub struct ButtonInfo {
  pub id: String,
  pub command: String,
  pub icon: String,
  pub text: String,
  pub keybind: char,
}

#[derive(Error, Debug)]
pub enum LoadSettingsError {
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),
    #[error("Settings content loader error")]
    ContentLoaderError(#[from] ContentLoaderError),
    #[error("No content or path")]
    ContentOrPathNotFound
}

fn from_json(json_str: &str, options: &Options) -> Result<Settings, LoadSettingsError> {
  let raw_settings = serde_json::from_str(json_str).map_err(LoadSettingsError::from)?;
  let settings = override_settings(raw_settings, options)?;
  return Ok(settings);
}

fn from_file(file_path: &Path, options: &Options) -> Result<Settings, LoadSettingsError> {
  let data = fs::read_to_string(file_path)?;
  return Ok(from_json(&data, &options)?);
}

fn override_settings(raw_settings: RawSettings, options: &Options) -> Result<Settings, LoadSettingsError> {
  let mut button_info: Option<Vec<ButtonInfo>> = None;
  
  if let Some(raw_buttons) = &options.buttons {
    button_info = Some(serde_json::from_str::<Vec<ButtonInfo>>(raw_buttons.as_str()).map_err(LoadSettingsError::from)?);
  } 

  Ok(
    Settings {
      buttons_per_row: options.buttons_per_row.unwrap_or_else(|| raw_settings.buttons_per_row.unwrap_or(3)),
      column_spacing: options.column_spacing.unwrap_or_else(|| raw_settings.column_spacing.unwrap_or(5)),
      row_spacing: options.row_spacing.unwrap_or_else(|| raw_settings.row_spacing.unwrap_or(5)),
      delay_before_closing: options.delay_before_closing.unwrap_or_else(|| raw_settings.delay_before_closing.unwrap_or(0)),
      buttons: button_info.unwrap_or_else(|| raw_settings.buttons.unwrap_or_else(Vec::new))
    }
  )
}

impl Settings {
  pub fn load_settings(options: &Options) -> Result<Settings, LoadSettingsError> {
    if let Some(content) = &options.settings_content {
      return from_json(&content, &options);
    } 
    else if let Some(settings_loader_path) = &options.settings_loader_path {
      let settings_content = load_content_from_script(&settings_loader_path)?;
      return from_json(&settings_content, &options);
    } 
    else if let Some(path) = &options.settings_path {
      return from_file(&path, &options);
    }

    if let Some(home_dir) = dirs::home_dir() {
      let curtains_config_loaderpath = home_dir.join(".config/curtains/close/settings.sh");
      if curtains_config_loaderpath.exists() {
        let settings_content = load_content_from_script(&curtains_config_loaderpath)?;
        return from_json(&settings_content, &options);
      }

      let curtains_config_path = home_dir.join(".config/curtains/close/settings.json");
      if curtains_config_path.exists() {
        return from_file(&curtains_config_path, &options);
      }
    }

    let usr_local_path = Path::new("/usr/local/etc/curtains-close/settings.json");
    if usr_local_path.exists() {
      return from_file(usr_local_path, &options);
    }

    let etc_path = Path::new("/etc/curtains-close/settings.json");
    if etc_path.exists() {
      return from_file(etc_path, &options);
    }    

    Err(LoadSettingsError::ContentOrPathNotFound)
  }
}