FROM rust:1.72.1-buster as build

WORKDIR /app

COPY src/* /app/src/
COPY Cargo.** /app/

RUN RUSTFLAGS="--deny warnings" cargo build -r

FROM gcr.io/distroless/cc@sha256:396891e37c26c8ea032aef368c806f64c950d19cc578fdab2b0093710a036895

COPY --from=build /app/target/release/discord-minecraft-emoji-bot /

ENTRYPOINT ["/discord-minecraft-emoji-bot"]
