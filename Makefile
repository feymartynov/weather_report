build:
	docker build -t weather_report:latest .

deploy:
	kubectl apply -f k8s.yml

all: build deploy
.PHONY: all
