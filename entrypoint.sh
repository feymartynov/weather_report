#!/bin/sh

envsubst < config.json.template > config.json
./weather_report
