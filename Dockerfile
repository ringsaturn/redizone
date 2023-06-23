FROM rust:1.70.0 as builder

WORKDIR /usr/src/redizone
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

COPY --from=builder /usr/local/cargo/bin/redizone /usr/local/bin/redizone

CMD ["redizone"]
