FROM rust:1.93.1 AS build

WORKDIR /app

COPY src/ /app/src/
COPY Cargo.** /app/

RUN RUSTFLAGS="--deny warnings" cargo build -r

FROM gcr.io/distroless/cc@sha256:8c1a496b055d36c222e95ebd5c53bdd1fee447689a97ad889febca8380d578ec

COPY --from=build /app/target/release/discord-minecraft-emoji-bot /

ENTRYPOINT ["/discord-minecraft-emoji-bot"]
