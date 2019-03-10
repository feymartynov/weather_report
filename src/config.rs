use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

use serde::Deserialize;

const CONFIG_PATH: &str = "config.json";

thread_local! {
    // App configuration parsed from JSON file
    pub static CONFIG: Rc<RefCell<Config>> = {
        let file = File::open(CONFIG_PATH).expect("Couldn't open config file");
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader).expect("Couldn't parse config file");
        Rc::new(RefCell::new(config))
    }
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: Server,
    pub providers: Vec<Provider>,
    pub geocoder: Geocoder,
}

#[derive(Clone, Deserialize)]
pub struct Server {
    pub address: String,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "name")]
pub enum Provider {
    Yandex { api_key: String },
    OpenWeatherMap { api_key: String },
}

#[derive(Clone, Deserialize)]
pub struct Geocoder {
    pub api_key: String,
}
