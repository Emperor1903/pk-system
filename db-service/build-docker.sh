#!/bin/bash

docker container stop db-service
docker container rm db-service
docker build -t db-service .
