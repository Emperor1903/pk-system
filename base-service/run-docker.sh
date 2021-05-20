#!/bin/bash

docker run -d \
       -p 8080:8080 \
       -it \
       --name base-service \
       base-service
