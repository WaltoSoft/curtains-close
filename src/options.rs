use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
#[command(author, version, disable_version_flag = true, about, long_about = None)]
pub struct Options {
  #[arg(short = 'v', long = "version", action = ArgAction::Version)]
  pub version: Option<bool>,

  #[arg(short = 'c', long = "css-path")]
  pub css_path: Option<PathBuf>,

  #[arg(short = 'C', long = "css-content")]
  pub css_content: Option<String>,

  #[arg(short = 'l', long = "css-loader-path")]
  pub css_loader_path: Option<PathBuf>,

  #[arg(short = 's', long = "settings-path")]
  pub settings_path: Option<PathBuf>,

  #[arg(short = 'S', long = "settings-content")]
  pub settings_content: Option<String>,

  #[arg(short = 'L', long = "settings-loader-path")]
  pub settings_loader_path: Option<PathBuf>,

  #[arg(short = 'b', long = "buttons")]
  pub buttons: Option<String>,
  
  #[arg(short = 'n', long = "buttons-per-row")]
  pub buttons_per_row: Option<u32>,

  #[arg(short = 'x', long = "column-spacing")]
  pub column_spacing: Option<u32>,

  #[arg(short = 'y', long = "row-spacing")]
  pub row_spacing: Option<u32>,

  #[arg(short = 'd', long = "delay-before-closing")]
  pub delay_before_closing: Option<u32>,
}