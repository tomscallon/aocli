use structopt::StructOpt;

mod config;
mod fetch_input;

#[derive(StructOpt)]
#[structopt(about = "Command-line interface for Advent of Code")]
pub enum Opt {
  Config(config::Opt),
  FetchInput(fetch_input::Opt),
}

pub fn run() -> super::Result {
  match Opt::from_args() {
    Opt::Config(opt) => config::run(opt),
    Opt::FetchInput(opt) => fetch_input::run(opt),
  }
}
