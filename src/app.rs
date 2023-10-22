use crate::config::get_config;

#[derive(clap::Args, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  create: Option<String>,
  list: Option<String>,
  delete: Option<String>,
  info: Option<String>,
  start: Option<String>,
  stop: Option<String>,
  env_set: Option<String>,
  env_unset: Option<String>,
  secret_set: Option<String>,
  secret_unset: Option<String>,
  logs: Option<String>,
  auto_scale: Option<String>,
  delete_pods: Option<String>,
  set_vhost: Option<String>,
  unset_vhost: Option<String>,
}

pub fn run(_args: Args) -> Result<(), Box<dyn std::error::Error>> {
  let config = get_config();
  dbg!(config);
  Ok(())
}
