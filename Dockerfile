FROM rust:1.70.0 as builder

WORKDIR /usr/src/redizone
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

LABEL \
    name = "redizone" \
    org.opencontainers.image.description = "Redis compatible server convert longitude and latitude to timezone name(s)." \
    org.opencontainers.image.source = "https://github.com/ringsaturn/redizone"

COPY --from=builder /usr/local/cargo/bin/redizone /usr/local/bin/redizone

CMD ["redizone"]
