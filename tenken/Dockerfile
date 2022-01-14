FROM rust:1.57-alpine
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN apk add --no-cache musl-dev

WORKDIR /bin/tenken

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo fetch

COPY ./src ./src
RUN cargo build --release

EXPOSE 8080
CMD cargo run --release
