use std::io;

use confy::ConfyError;

#[derive(Debug)]
pub enum Error {
  ConfyError(ConfyError),
  IoError(io::Error),
  MissingConfig(String),
  UnknownConfig(String),
}

impl From<ConfyError> for Error {
  fn from(confy_error: ConfyError) -> Error {
    Error::ConfyError(confy_error)
  }
}

impl From<io::Error> for Error {
  fn from(io_error: io::Error) -> Error {
    Error::IoError(io_error)
  }
}
