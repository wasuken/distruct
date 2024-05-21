use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    pub url_list: Vec<String>,
}

pub fn load_config(config_file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // 設定ファイルを読み込む
    let mut file = File::open(config_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // JSONデータをパース
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}
