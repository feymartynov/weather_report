#[macro_use]
extern crate lazy_static;

extern crate reqwest;
extern crate weather_report;

#[macro_use]
mod support;

// Integration test modules
mod generic_api_test;
mod forecast_test;
