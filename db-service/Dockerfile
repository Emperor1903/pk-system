FROM rust:1.52.1

WORKDIR /usr/src/db-service

COPY . .

RUN cargo install --path .

CMD ["db-service"]
