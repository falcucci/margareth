use clap::Parser;

mod app;
mod config;
mod deploy;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Margareth {
  Config(config::Args),
  App(Box<app::Args>),
  Deploy(Box<deploy::Args>),
}

fn main() {
  let margareth = Margareth::parse();
  let result = match margareth {
    Margareth::Config(config) => config::run(config),
    Margareth::App(app) => app::run(*app),
    Margareth::Deploy(deploy) => deploy::run(*deploy),
  };

  if let Err(err) = result {
    println!("error: {}", err);
  }
}
