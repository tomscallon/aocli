use structopt::StructOpt;

pub mod config;

#[derive(StructOpt)]
#[structopt(about = "Command-line interface for Advent of Code")]
pub enum Opt {
  Config(config::Opt),
}

pub fn run() -> super::Result {
  match Opt::from_args() {
    Opt::Config(opt) => config::run(opt),
  }
}
