use clap::Parser;

mod app;
mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[warn(clippy::large_enum_variant)]
enum Margareth {
  Config(config::Args),
  App(Box<app::Args>),
}

fn main() {
  let margareth = Margareth::parse();
  let result = match margareth {
    Margareth::Config(config) => config::run(config),
    Margareth::App(app) => app::run(*app),
  };

  if let Err(err) = result {
    println!("error: {}", err);
  }
}
