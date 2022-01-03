use super::super::config;

use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Opt {
  AuthToken {
    value: Option<String>,
  },
  InputDir {
    #[structopt(parse(from_os_str))]
    value: Option<PathBuf>,
  },
}

pub fn run(opt: Opt) -> super::super::Result {
  match opt {
    Opt::AuthToken { value } => match value {
      Some(auth_token) => config::set_auth_token(auth_token),
      None => print_config("auth_token"),
    },
    Opt::InputDir { value } => match value {
      Some(input_dir) => config::set_input_dir(input_dir),
      None => print_config("input_dir"),
    },
  }
}

fn print_config(name: &str) -> super::super::Result {
  println!("{}", config::get_string(name)?);
  Ok(())
}
