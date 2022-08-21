use std::error::Error;
use chrome_native::chrome_native_task;
use chrome_native::{parse_data, Plugin};

mod hello;

#[derive(thiserror::Error, Debug)]
pub enum AuthErrors {
  #[error("Unsupported")]
  Unsupported(),
  #[error("Failed to login")]
  FailedLogin(),
  #[error("Windows Error: {0}")]
  WinError(#[from] windows::core::Error),
  #[error("IO Error: {0}")]
  IOError(#[from] std::io::Error)
}

/// Tasks that will be handled from chrome
#[chrome_native_task]
pub enum Tasks {
    HelloTask(String)
}

#[derive(Plugin)]
#[default_gen(true)]

pub struct MyPlugin {

}

impl Plugin for MyPlugin {
    /// Handle raw chrome messages
    fn handle_command(&self, command: String) -> Result<String, Box<dyn Error>> {
        let task = parse_data::<Tasks>(command.as_str())?;
        match task {
            Tasks::HelloTask(_str) => {
                let result = hello::win_hello()?;
                Ok(format!("success: {}", result))
            },
        }
    }
}
