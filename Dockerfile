FROM rust:buster as build

WORKDIR /app

COPY src/* /app/src/
COPY Cargo.** /app/

RUN cargo build -r

FROM gcr.io/distroless/cc

COPY --from=build /app/target/release/discord-minecraft-emoji-bot /

ENTRYPOINT /discord-minecraft-emoji-bot
