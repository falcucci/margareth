use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub config: Option<ConfigView>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigView {
  Config(Config),
}

#[derive(Args, Debug)]
pub struct Config {
  pub view: String,
}


fn main() {
  let args = Cli::parse();
  println!("{:?}", args);
}
