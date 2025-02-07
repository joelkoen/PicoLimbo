FROM rust:1.84-alpine AS build

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/app
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release --locked && \
    cp /usr/src/app/target/release/server /usr/bin/server

FROM alpine

WORKDIR /usr/src/app

COPY ./data /usr/src/app/data
COPY --from=build /usr/bin/server /usr/bin/server

ENV DATA_DIR=/usr/src/app/data
ENV PORT=25565

CMD ["server", "-a", "0.0.0.0:25565"]
