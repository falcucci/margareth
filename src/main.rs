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
  match args.config {
    Some(config) => match config {
      ConfigView::Config(config) => {
        dbg!(config);
        let file = get_config();
        dbg!(file);
      }
    },
    _ => println!("No config"),
  }
}

fn get_config() -> serde_json::Value {
  let file = std::fs::read_to_string("margareth.json").unwrap();
  file.parse::<serde_json::Value>().unwrap()
}
