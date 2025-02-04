use std::{io, path::PathBuf, process::Command, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContentLoaderError {
    #[error("Error reading css loader script output")]
    IOError(#[from] io::Error),
    #[error("Error reading css loader script output")]
    StringConversionError(#[from] FromUtf8Error),
    #[error("Error running CSS loader script")]
    ScriptError,
    #[error("Script at path not found.")]
    PathNotFound,
}


pub fn load_content_from_script(script_path: &PathBuf) -> Result<String, ContentLoaderError> {
  if script_path.exists() {
    let cmd= script_path.to_str().unwrap();
    let output = Command::new("bwrap")
      .args(&[
          "--ro-bind", "/", "/",
          "--dev-bind", "/dev", "/dev",   
          "--proc", "/proc",  
          "--tmpfs", "/tmp",  
          "--unshare-all",  
          "--new-session",  
          "--die-with-parent",
          cmd
      ])
      .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error running bwrap: {}", stderr);
        return Err(ContentLoaderError::ScriptError);
    }

    let result = String::from_utf8(output.stdout)?;

    Ok(result)
  } else {
    Err(ContentLoaderError::PathNotFound)
  }
}