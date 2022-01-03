use std::io;

#[derive(Debug)]
pub enum Error {
  ConfyError(confy::ConfyError),
  IoError(io::Error),
  MissingConfig(String),
  ReqwestError(reqwest::Error),
  UnknownConfig(String),
}

impl From<confy::ConfyError> for Error {
  fn from(confy_error: confy::ConfyError) -> Error {
    Error::ConfyError(confy_error)
  }
}

impl From<io::Error> for Error {
  fn from(io_error: io::Error) -> Error {
    Error::IoError(io_error)
  }
}

impl From<reqwest::Error> for Error {
  fn from(reqwest_error: reqwest::Error) -> Error {
    Error::ReqwestError(reqwest_error)
  }
}
