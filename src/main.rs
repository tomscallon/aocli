mod commands;
mod config;
mod error;

pub type Result<T = ()> = std::result::Result<T, error::Error>;

fn main() {
  match commands::run() {
    Ok(_) => {}
    Err(error) => println!("Failed to run: {:?}", error),
  }
}
