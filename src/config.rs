use super::error::Error;

use std::path::PathBuf;

use confy;
use serde::{Deserialize, Serialize};

const CONFY_NAME: &str = "aocli";

#[derive(Deserialize, Serialize)]
struct Config {
  auth_token: Option<String>,
  input_dir: Option<PathBuf>,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      auth_token: None,
      input_dir: None,
    }
  }
}

pub fn get(name: &str) -> super::Result<String> {
  let config: Config = confy::load(CONFY_NAME)?;

  match name {
    "auth_token" => Ok(config.auth_token.unwrap_or("(none)".into())),
    "input_dir" => Ok(
      config
        .input_dir
        .map(|dir| {
          dir
            .into_os_string()
            .into_string()
            .unwrap_or("(can't represent as string)".into())
        })
        .unwrap_or("(none)".into()),
    ),
    _ => Err(Error::UnknownConfig(name.into())),
  }
}

pub fn set_auth_token(auth_token: String) -> super::Result {
  let mut config: Config = confy::load(CONFY_NAME)?;

  config.auth_token = Some(auth_token);
  confy::store(CONFY_NAME, config)?;

  Ok(())
}

pub fn set_input_dir(input_dir: PathBuf) -> super::Result {
  let mut config: Config = confy::load(CONFY_NAME)?;

  config.input_dir = Some(input_dir);
  confy::store(CONFY_NAME, config)?;

  Ok(())
}
