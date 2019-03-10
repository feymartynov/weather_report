use std::error::Error;

use chrono::prelude::*;
use iron::mime::Mime;
use iron::prelude::*;
use iron::{status, AfterMiddleware};
use router::{NoRoute, Router};
use serde::Deserialize;
use serde_json::json;
use simple_error::SimpleError;

use super::config::CONFIG;

// *********************
// * Endpoint handlers *
// *********************

#[derive(Debug, Deserialize)]
struct ForecastQueryParams {
    location: String,
}

const FORECASTS_INDEX_DAYS: usize = 5;

/// Returns an array of reports for the next 5 days.
///
/// GET /forecasts
/// Example response: `{"reports": [{"temperature": -12.3}, …]}`.
fn forecasts_index_handler(request: &mut Request) -> IronResult<Response> {
    parse_query_params::<ForecastQueryParams>(request)
        .and_then(|params| super::reporter::report(&params.location, FORECASTS_INDEX_DAYS))
        .and_then(|reports| Ok(json_response(status::Ok, json!({ "reports": reports }))))
        .or_else(|error| Ok(error_response(error)))
}

/// Returns a report for the specified day.
///
/// GET /forecasts/:date
/// `date` has format "YYYY-MM-DD".
/// Example response: `{"report": {"temperature": -12.3}}`.
fn forecasts_show_handler(request: &mut Request) -> IronResult<Response> {
    parse_query_params::<ForecastQueryParams>(request)
        .and_then(|params| parse_date(request).map(|date| (params, date)))
        .and_then(|(params, date)| days_since_today(date).map(|days| (params, days)))
        .and_then(|(params, days)| super::reporter::report(&params.location, days))
        .and_then(|reports| {
            let report = reports.last().unwrap();
            Ok(json_response(status::Ok, json!({ "report": report })))
        })
        .or_else(|error| Ok(error_response(error)))
}

/// Parses `date` URL parameter to NaiveDate. Expected format is "YYYY-MM-DD".
fn parse_date(request: &Request) -> Result<NaiveDate, Box<dyn Error>> {
    let router = request.extensions.get::<Router>().unwrap();
    let date_str = router.find("date").unwrap();

    NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| SimpleError::new("Invalid date format").into())
}

/// Counts the number of days between today and the given date.
/// Returns `Ok(number_of_days)` when the value is valid and is between 0 and 5.
/// Otherwise returns `Err(error)`.
fn days_since_today(date: NaiveDate) -> Result<usize, Box<dyn Error>> {
    let today = Local::now().naive_local().date();
    let days_since_today = date.signed_duration_since(today).num_days();

    if days_since_today < 0 {
        Err(SimpleError::new("Date is in the past").into())
    } else if days_since_today > 5 {
        Err(SimpleError::new("Date is more than 5 days in the future").into())
    } else {
        Ok(days_since_today as usize)
    }
}

// ********************
// * Helper functions *
// ********************

/// Parses query parameters from the `request`'s URL into `T` struct using serde_qs.
/// Returns `Ok(T)` if parameters are present and valid. Otherwise returns `Err(error)`.
fn parse_query_params<'a, T: Deserialize<'a>>(request: &'a Request) -> Result<T, Box<dyn Error>> {
    request
        .url
        .query()
        .ok_or_else(|| SimpleError::new("Missing required params").into())
        .and_then(|query_string| serde_qs::from_str::<T>(query_string).map_err(|e| e.into()))
}

/// Builds 422 UnprocessableEntity error with JSON body containing the error description.
fn error_response(error: Box<dyn Error>) -> Response {
    let body = json!({ "error": error.description() });
    json_response(status::UnprocessableEntity, body)
}

/// Builds HTTP response with JSON `body` and specified HTTP `status` code.
fn json_response(status: status::Status, body: serde_json::Value) -> Response {
    let content_type = "application/json".parse::<Mime>().unwrap();
    Response::with((content_type, status, body.to_string()))
}

struct Custom404;

impl AfterMiddleware for Custom404 {
    /// Returns 404 with JSON error on missing route.
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if err.error.is::<NoRoute>() {
            Ok(json_response(
                status::NotFound,
                json!({"error": "Not found"}),
            ))
        } else {
            Err(err)
        }
    }
}

/// Starts the server.
pub fn start(config: Option<super::config::Server>) -> iron::error::HttpResult<iron::Listening> {
    // Application routes.
    let router = router::router!(
        forecasts_index: get "/forecasts" => forecasts_index_handler,
        forecasts_show: get "/forecasts/:date" => forecasts_show_handler,
    );

    let mut chain = Chain::new(router);
    chain.link_after(Custom404);

    let server_config =
        config.unwrap_or_else(|| CONFIG.with(|config| config.borrow().server.clone()));

    Iron::new(chain).http(server_config.address)
}
