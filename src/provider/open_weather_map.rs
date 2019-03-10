use std::error::Error;

use reqwest::Client;
use serde::Deserialize;

use super::super::provider::Provider;
use super::super::reporter::Report;

pub struct OpenWeatherMap {
    api_key: String,
    client: Client,
}

impl OpenWeatherMap {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: String::from(api_key),
            client: Client::new(),
        }
    }

    /// Makes a call to OpenWeatherMap weather forecast API and returns the response.
    fn get_forecast(
        &self,
        lat: f32,
        lon: f32,
        days: usize,
    ) -> Result<ForecastResponse, Box<dyn Error>> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&cnt={}&units=metric&appid={}",
            lat, lon, days, self.api_key
        );

        let mut response = self.client.get(url.as_str()).send()?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json()?),
            status => bail!("Unexpected HTTP status {}", status.as_u16()),
        }
    }
}

impl Provider for OpenWeatherMap {
    fn name(&self) -> String {
        String::from("OpenWeatherMap")
    }

    /// Gets weather forecasts from OpenWeatherMap, parses the response and builds reports.
    fn get_reports(&self, lat: f32, lon: f32, days: usize) -> Result<Vec<Report>, Box<dyn Error>> {
        let reports = self
            .get_forecast(lat, lon, days)?
            .list
            .iter()
            .map(|forecast| Report::new(forecast.main.temp))
            .collect();

        Ok(reports)
    }
}

/*
Structs for parsing OpenWeatherMap API response with serde.
Example response (simplified):

{
    "list": [
        {
            "main": {
                "temp": -3.5
            }
        }
    ]
}
*/

#[derive(Debug, Deserialize)]
struct ForecastResponse {
    list: Vec<Forecast>,
}

#[derive(Debug, Deserialize)]
struct Forecast {
    main: Main,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f32,
}
