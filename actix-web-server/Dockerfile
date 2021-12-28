## ref: https://dev.to/sergeyzenchenko/actix-web-in-docker-how-to-build-small-and-secure-images-2mjd
FROM rust:1.57.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/api-service
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/api-service /usr/local/bin/api-service

CMD ["api-service"]
