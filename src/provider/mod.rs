mod open_weather_map;
mod yandex;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use super::config;
use super::reporter::Report;
use open_weather_map::OpenWeatherMap;
use yandex::Yandex;

fn build_provider(provider_config: &config::Provider) -> Box<dyn Provider> {
    match provider_config {
        config::Provider::Yandex { api_key } => Box::new(Yandex::new(api_key)),
        config::Provider::OpenWeatherMap { api_key } => Box::new(OpenWeatherMap::new(api_key)),
    }
}

thread_local! {
    // Weather forecast providers reference.
    pub static PROVIDERS: Rc<RefCell<Vec<Box<dyn Provider>>>> = {
        config::CONFIG.with(|config| {
            let providers = config.borrow().providers.iter().map(|p| build_provider(p)).collect();
            Rc::new(RefCell::new(providers))
        })
    };
}

pub trait Provider {
    fn name(&self) -> String;
    fn get_reports(&self, lat: f32, lon: f32, days: usize) -> Result<Vec<Report>, Box<dyn Error>>;
}
