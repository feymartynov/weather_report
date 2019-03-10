# Weather Report

A sample weather API aggregator in Rust.

Implemented providers:
* Yandex Weather
* OpenWeatherMap

Also Yandex Maps Geocoding API is being used for geocoding.

## API

* `GET /forecasts?location=CityName` – get forecasts for the next 5 days.
* `GET /forecasts/YYYY-MM-DD?location=CityName` – get a single forecast for a specific date.

### Example curl

```bash
curl "http://localhost:3000/forecasts/2019-03-11?location=%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B0"
```

## Building and running locally

```bash
cargo build
cargo run
```

The server starts at http://localhost:3000

## Testing

```bash
cargo test
```

## Deploying to Kubernetes

```bash
make all
```

This builds a Docker image and deploys it to Kubernetes.
