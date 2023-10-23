use crate::config::get_config;

#[derive(clap::Args, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  create: Option<String>,
  list: Option<String>,
  rollback: Option<String>,
}

pub fn run(_args: Args) -> Result<(), Box<dyn std::error::Error>> {
  let config = get_config();
  dbg!(config);
  Ok(())
}
