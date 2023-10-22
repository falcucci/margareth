use clap::{Args, Parser, Subcommand};

mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Margareth {
  Config(config::Args),
}

#[derive(Subcommand, Debug)]
pub enum App {
  Create(Create),
  Remove(Remove),
  List(List),
}

#[derive(Args, Debug)]
pub struct Create {
  pub name: String,
}

#[derive(Args, Debug)]
pub struct Remove {
  pub name: String,
}

#[derive(Args, Debug)]
pub struct List {
  pub name: String,
}

fn main() {
  let margareth = Margareth::parse();
  let result = match margareth {
    Margareth::Config(config) => config::run(config),
  };

  if let Err(err) = result {
    println!("error: {}", err);
  }
}
