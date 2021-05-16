FROM rust:1.31

WORKDIR /usr/src/db-service

COPY . .

RUN cargo build --release
RUN cargo install --path .

CMD ["./db-service"]
