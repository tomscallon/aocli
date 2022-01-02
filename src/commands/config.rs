use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opt {
  name: String,
  value: Option<String>,
}

pub fn run(opt: Opt) {
  println!("Ran `config {:?} {:?}`", opt.name, opt.value)
}
