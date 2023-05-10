use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs;
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    path: String,
    file_limit: u32,
    get_url: String,
}

fn main() {
    let f = std::fs::File::open("config.yml").expect("Could not open file: config.yml");
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    //println!("{:?}", scrape_config);

    let paths = fs::read_dir(scrape_config.path).unwrap();
    let file_count = paths.count();
    //println!("Found Files: {}", file_count);

    if file_count <= scrape_config.file_limit as usize {
        println!("File count is less or equal than limit. Exiting..");
        return;
    }

    let mut url = scrape_config.get_url.clone();
    url.push_str("&file_count=");
    url.push_str(&file_count.to_string());

    let mut easy = Easy::new();
    easy.get(true).unwrap();
    easy.url(&url).unwrap();
    easy.perform().unwrap();
    println!("File count is greater than limit. Requested..");
}
