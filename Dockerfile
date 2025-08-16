FROM rust:1.89.0 as build

WORKDIR /app

COPY src/ /app/src/
COPY Cargo.** /app/

RUN RUSTFLAGS="--deny warnings" cargo build -r

FROM gcr.io/distroless/cc@sha256:00cc20b928afcc8296b72525fa68f39ab332f758c4f2a9e8d90845d3e06f1dc4

COPY --from=build /app/target/release/discord-minecraft-emoji-bot /

ENTRYPOINT ["/discord-minecraft-emoji-bot"]
