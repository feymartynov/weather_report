use std::error::Error;

use reqwest::Client;
use serde::Deserialize;

use super::super::provider::Provider;
use super::super::reporter::Report;

pub struct Yandex {
    api_key: String,
    client: Client,
}

impl Yandex {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: String::from(api_key),
            client: Client::new(),
        }
    }

    /// Makes a call to Yandex weather forecast API and returns the response.
    fn get_forecast(
        &self,
        lat: f32,
        lon: f32,
        days: usize,
    ) -> Result<ForecastResponse, Box<dyn Error>> {
        let url = format!(
            "https://api.weather.yandex.ru/v1/forecast?lat={}&lon={}&limit={}",
            lat, lon, days
        );

        let mut response = self
            .client
            .get(url.as_str())
            .header("X-Yandex-API-Key", self.api_key.clone())
            .send()?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json()?),
            status => bail!("Unexpected HTTP status {}", status.as_u16()),
        }
    }
}

impl Provider for Yandex {
    fn name(&self) -> String {
        String::from("Yandex")
    }

    /// Gets weather forecasts from Yandex, parses the response and builds reports.
    fn get_reports(&self, lat: f32, lon: f32, days: usize) -> Result<Vec<Report>, Box<dyn Error>> {
        let reports = self
            .get_forecast(lat, lon, days)?
            .forecasts
            .iter()
            .map(|forecast| Report::new(forecast.parts.day.temp_avg as f32))
            .collect();

        Ok(reports)
    }
}

/*
Structs for parsing Yandex weather API response with serde.
Example response (simplified):

{
    "forecasts": [
        {
            "parts": {
                "day": {
                    "temp_avg": -10
                }
            }
        }
    ]
}
*/

#[derive(Debug, Deserialize)]
struct ForecastResponse {
    forecasts: Vec<Forecast>,
}

#[derive(Debug, Deserialize)]
struct Forecast {
    parts: Parts,
}

#[derive(Debug, Deserialize)]
struct Parts {
    day: Day,
}

#[derive(Debug, Deserialize)]
struct Day {
    temp_avg: isize,
}
