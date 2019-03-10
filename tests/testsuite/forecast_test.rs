use chrono::{Duration, Local};
use serde::Deserialize;

use crate::support::shared_responses::ErrorResponse;
use crate::support::test_server_client::CLIENT;

/// It should return a list of reports for the next 5 days.
#[test]
fn week_forecast() {
    let url = "/forecasts?location=%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B0";
    let response = CLIENT.get_json::<ForecastIndexResponse>(url, 200);
    assert_eq!(response.reports.len(), 5);
}

/// It should return a single averaged report for the given day.
#[test]
fn date_forecast() {
    let date = Local::now() + Duration::days(2);

    let url = format!(
        "/forecasts/{}?location=%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B0",
        date.format("%Y-%m-%d").to_string()
    );

    CLIENT.get_json::<ForecastShowResponse>(&url, 200);
}

/// It should return an error on invalid date format.
#[test]
fn bad_date() {
    let url = "/forecasts/123-45?location=%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B0";
    let response = CLIENT.get_json::<ErrorResponse>(url, 422);
    assert_eq!(response.error, "Invalid date format");
}

/// It should return an error on missing location parameter.
#[test]
fn missing_location() {
    let response = CLIENT.get_json::<ErrorResponse>("/forecasts", 422);
    assert_eq!(response.error, "Missing required params");
}

/// It should return an error when unable to geocode the location.
#[test]
fn bad_location() {
    let url = "/forecasts?location=qweqweqweqweqwqwqeqeqweqweqweqeqw";
    let response = CLIENT.get_json::<ErrorResponse>(url, 422);
    assert_eq!(response.error, "Failed to geocode location");
}

/// Structs for parsing expected responses.

#[derive(Debug, Deserialize)]
struct ForecastIndexResponse {
    reports: Vec<Report>,
}

#[derive(Debug, Deserialize)]
struct ForecastShowResponse {
    report: Report,
}

#[derive(Debug, Deserialize)]
struct Report {
    temperature: f32,
}
