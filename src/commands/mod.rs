use structopt::StructOpt;

pub mod config;

#[derive(StructOpt)]
#[structopt(about = "Command-line interface for Advent of Code")]
pub enum Opt {
  Config(config::Opt),
}

pub fn run() {
  use Opt::*;

  match Opt::from_args() {
    Config(opt) => config::run(opt),
  }
}
