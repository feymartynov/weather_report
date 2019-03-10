mod reducer;

use std::error::Error;

use futures::{future, Future};
use serde::Serialize;

use super::geocoder::GEOCODER;
use super::provider::{Provider, PROVIDERS};
use reducer::Reducer;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Report {
    temperature: f32,
}

impl Report {
    pub fn new(temperature: f32) -> Self {
        Self { temperature }
    }
}

/// Asks each provider for a report on given number of days in a specific location.
/// Returns a vector with averaged reports for each day.
/// The location is being geocoded by geocoder provider at first.
pub fn report(location: &str, days: usize) -> Result<Vec<Report>, Box<dyn Error>> {
    let (lat, lon) = GEOCODER.with(|g| g.borrow().geocode(location))?;
    let mut reducer = Reducer::new(days);

    PROVIDERS.with(|providers| {
        let mut futures = Vec::with_capacity(providers.borrow().len());

        // Call each provider asynchronously.
        for provider in providers.borrow().iter() {
            futures.push(get_provider_reports_async(&provider, lat, lon, days));
        }

        // Synchronize providers and ignore errors.
        for provider_reports in futures.iter_mut().filter_map(|future| future.wait().ok()) {
            reducer.push_provider_reports(&provider_reports);
        }
    });

    Ok(reducer.reduce_reports())
}

/// Wraps provider report call into future.
fn get_provider_reports_async(
    provider: &Box<dyn Provider>,
    lat: f32,
    lon: f32,
    days: usize,
) -> impl Future<Item = Vec<Report>, Error = Box<dyn 'static + Error>> {
    match provider.get_reports(lat, lon, days) {
        Ok(reports) => future::ok(reports),
        Err(error) => {
            println!(
                "Error fetching from provider {}: {}",
                provider.name(),
                error
            );
            future::err(error)
        }
    }
}
