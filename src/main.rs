mod cli;
mod models;

use models::configuration::config_dict::Config;

fn main() {
    let options_path = "files/k8_clusters.json";

    let cfg: Config = Config::new(options_path);

    cli::start(cfg);
}
