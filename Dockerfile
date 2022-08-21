FROM rust:buster as build

WORKDIR /app

COPY src/* /app/src/
COPY Cargo.** /app/

RUN cargo build -r

ENTRYPOINT /app/target/release/discord-minecraft-emoji-bot
