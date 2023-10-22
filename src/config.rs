#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  #[command(subcommand)]
  view: Option<ConfigView>,
}

#[derive(clap::Subcommand, Debug)]
enum ConfigView {
  View { view: Option<String> },
}

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
  match args.view {
    Some(ConfigView::View { view: _ }) => {
      let config = get_config();
      dbg!(config);
    }
    None => {
      println!("no view");
    }
  }
  Ok(())
}

fn get_config() -> serde_json::Value {
  let file = std::fs::read_to_string("margareth.json").unwrap();
  file.parse::<serde_json::Value>().unwrap()
}
