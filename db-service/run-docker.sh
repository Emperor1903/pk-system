#!/bin/bash

docker container stop db-service
docker container rm db-service
docker build -t db-service .
docker run -d \
       -p 8080:8080 \
       -it \
       --name db-service \
       db-service
