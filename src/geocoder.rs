use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use reqwest::Client;
use serde::Deserialize;
use simple_error::SimpleError;

use super::config::CONFIG;

thread_local! {
    // Service that is used for searching geo coordinates of locations by string.
    pub static GEOCODER: Rc<RefCell<Geocoder>> = {
        CONFIG.with(|config| {
            let geocoder = Geocoder::new(&config.borrow().geocoder.api_key);
            Rc::new(RefCell::new(geocoder))
        })
    };
}

pub struct Geocoder {
    api_key: String,
    client: Client,
}

impl Geocoder {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: String::from(api_key),
            client: Client::new(),
        }
    }

    /// Asks Yandex to geocode `location`, parses the response and returns `(lat, lon)`.
    pub fn geocode(&self, location: &str) -> Result<(f32, f32), Box<dyn Error>> {
        self.get_geocode(location)?
            .response
            .geo_object_collection
            .feature_member
            .first()
            .and_then(|feature_member| {
                // Yandex returns lon first, e.g. "37.12345 55.67890" (lon, lat).
                let mut it = feature_member.geo_object.point.pos.split(' ');
                let lon_parse = it.next().map(|x| x.parse::<f32>()).and_then(|x| x.ok());
                let lat_parse = it.next().map(|x| x.parse::<f32>()).and_then(|x| x.ok());
                lon_parse.and_then(|lon| lat_parse.and_then(|lat| Some((lat, lon))))
            })
            .ok_or_else(|| SimpleError::new("Failed to geocode location").into())
    }

    /// Makes a call to Yandex geocoding API and returns the response.
    fn get_geocode(&self, location: &str) -> Result<GeocodeResponse, Box<dyn Error>> {
        let escaped_location = utf8_percent_encode(location, DEFAULT_ENCODE_SET).to_string();

        let url = format!(
            "https://geocode-maps.yandex.ru/1.x/?geocode={}&kind=locality&apikey={}&format=json",
            escaped_location, self.api_key
        );

        let mut response = self.client.get(url.as_str()).send()?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json()?),
            status => bail!("Unexpected HTTP status {}", status.as_u16()),
        }
    }
}

/*
Structs for parsing Yandex geocoding API response with serde.
Example response (simplified):

{
    "response": {
        "GeoObjectCollection": {
            "featureMember": [
                {
                    "GeoObject": {
                        "Point": {
                            "pos": "37.12345 55.67890"
                        }
                    }
                }
            ]
        }
    }
}
*/

#[derive(Debug, Deserialize)]
struct GeocodeResponse {
    response: GeocodeResponseWrapper,
}

#[derive(Debug, Deserialize)]
struct GeocodeResponseWrapper {
    #[serde(rename = "GeoObjectCollection")]
    geo_object_collection: GeoObjectCollection,
}

#[derive(Debug, Deserialize)]
struct GeoObjectCollection {
    #[serde(rename = "featureMember")]
    feature_member: Vec<FeatureMember>,
}

#[derive(Debug, Deserialize)]
struct FeatureMember {
    #[serde(rename = "GeoObject")]
    geo_object: GeoObject,
}

#[derive(Debug, Deserialize)]
struct GeoObject {
    #[serde(rename = "Point")]
    point: Point,
}

#[derive(Debug, Deserialize)]
struct Point {
    pos: String,
}
