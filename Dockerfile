FROM rust:1.72.0-buster as build

WORKDIR /app

COPY src/* /app/src/
COPY Cargo.** /app/

RUN RUSTFLAGS="--deny warnings" cargo build -r

FROM gcr.io/distroless/cc@sha256:3603adbdee2906dc3b7a18d7c0424a40633231c61dcd82196ae15de1282a5822

COPY --from=build /app/target/release/discord-minecraft-emoji-bot /

ENTRYPOINT ["/discord-minecraft-emoji-bot"]
