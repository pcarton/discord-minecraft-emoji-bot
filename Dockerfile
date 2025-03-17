FROM rust:1.85.0 as build

WORKDIR /app

COPY src/ /app/src/
COPY Cargo.** /app/

RUN RUSTFLAGS="--deny warnings" cargo build -r

FROM gcr.io/distroless/cc@sha256:85dac24dd2f03e841d986d5ed967385d3a721dcd9dbd21b602ddd82437f364c9

COPY --from=build /app/target/release/discord-minecraft-emoji-bot /

ENTRYPOINT ["/discord-minecraft-emoji-bot"]
