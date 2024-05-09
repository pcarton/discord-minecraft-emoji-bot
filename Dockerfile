FROM rust:1.78.0-buster as build

WORKDIR /app

COPY src/ /app/src/
COPY Cargo.** /app/

RUN RUSTFLAGS="--deny warnings" cargo build -r

FROM gcr.io/distroless/cc@sha256:e1065a1d58800a7294f74e67c32ec4146d09d6cbe471c1fa7ed456b2d2bf06e0

COPY --from=build /app/target/release/discord-minecraft-emoji-bot /

ENTRYPOINT ["/discord-minecraft-emoji-bot"]
