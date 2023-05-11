use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs;
use std::collections::HashMap;
use std::str;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    path: String,
    file_limit: u32,
    get_url: String,
    payload_title: String
}

fn main() {
    let f = std::fs::File::open("config.yml").expect("Could not open file: config.yml");
    let config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    //println!("{:?}", config);

    let paths = fs::read_dir(config.path.clone()).unwrap();
    let file_count = paths.count();
    //println!("Found Files: {}", file_count);

    if file_count <= config.file_limit as usize {
        println!("File count is less or equal than limit. Exiting..");
        return;
    }

    
    let mut body = String::from("");
    body.push_str("Es wurden ");
    body.push_str(&(file_count.to_string()));
    body.push_str(" Dateien im Ordner ");
    body.push_str(&config.path);
    body.push_str(" gefunden. Das Limit ist: ");
    body.push_str(&(config.file_limit.to_string()));

    let body_str: &str = &body;
    let title: &str = &config.payload_title.as_str();

    let mut map = HashMap::new();
    map.insert("@type", "MessageCard");
    map.insert("@context", "<http://schema.org/extensions>");
    map.insert("summary", "Meine erste Alert-Summary!");
    map.insert("themeColor", "D70000");
    map.insert("title", title);
    map.insert("text", body_str);

    let _res = rest(config.get_url, map);

    return;
}

#[tokio::main]
async fn rest(url: String, map: HashMap<&str, &str>) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let _res = client.post(url)
        .json(&map)
        .send()
        .await?;
        Ok(())
}
