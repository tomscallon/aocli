use confy::ConfyError;

#[derive(Debug)]
pub enum Error {
  ConfyError(ConfyError),
  UnknownConfig(String),
}

impl From<ConfyError> for Error {
  fn from(confy_error: ConfyError) -> Error {
    Error::ConfyError(confy_error)
  }
}
