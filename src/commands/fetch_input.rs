use super::super::config;
use super::super::error;

use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opt {
  year: u16,
  day: u8,

  /// By default, input file will not be downloaded if the
  /// destination file already exists. Pass this to force
  /// overwriting of the existing file.
  #[structopt(short)]
  force: bool,
}

pub fn run(opt: Opt) -> super::super::Result {
  match config::get_auth_token()? {
    Some(auth_token) => match config::get_input_dir()? {
      Some(input_dir) => fetch_input_file(auth_token, input_dir, opt),
      None => Err(error::Error::MissingConfig("input-dir".into())),
    },
    None => Err(error::Error::MissingConfig("auth-token".into())),
  }
}

fn fetch_input_file(auth_token: String, input_dir: PathBuf, opt: Opt) -> super::super::Result {
  let mut file_path = input_dir.clone();
  file_path.push(opt.year.to_string());
  file_path.push(format!("{}.txt", opt.day));

  let mut file = prepare_file(file_path, opt.force)?;
  let input_text = super::super::http::fetch_input(auth_token, opt.year, opt.day)?;
  file.write_all(input_text.as_bytes())?;

  println!("Fetching input for year {} day {}", opt.year, opt.day);
  Ok(())
}

fn prepare_file(path: PathBuf, force: bool) -> super::super::Result<File> {
  create_dir_all(path.parent().unwrap())?;

  let mut options = OpenOptions::new();
  options.write(true);
  if force {
    options.create(true).truncate(true);
  } else {
    options.create_new(true);
  }

  let file = options.open(&path)?;

  Ok(file)
}
