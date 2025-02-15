# syntax=docker/dockerfile:1.7-labs
FROM rust:1.84-alpine AS build

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/app
COPY --parents ./Cargo.toml ./Cargo.lock ./pico_wake ./pico_limbo ./libraries ./macros ./

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release --locked --package pico_wake --bin pico_wake && \
    cp /usr/src/app/target/release/pico_wake /usr/bin/pico_wake

FROM alpine

WORKDIR /usr/src/app

COPY data/generated /usr/src/app/data
COPY --from=build /usr/bin/pico_wake /usr/bin/pico_wake

ENV DATA_DIR=/usr/src/app/data

CMD ["pico_wake", "-a", "0.0.0.0:25565"]
