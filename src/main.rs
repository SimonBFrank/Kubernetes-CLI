mod models;

use crate::models::configuration::config_dict::Config;

fn main() {
    let options_path = "files/k8_clusters.json";

    let cfg: Config = Config::new(options_path);

    let pkey: String = String::from("qa");
    let skey: String = String::from("blue");
    let result: String = cfg[pkey][skey].get_file_path();

    println!("{:#?}", result)
}
