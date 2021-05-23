#!/bin/bash

docker container stop base-service
docker container rm base-service
docker build -t base-service .
