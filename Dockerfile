FROM rust:1.93.1 AS build

WORKDIR /app

COPY src/ /app/src/
COPY Cargo.** /app/

RUN RUSTFLAGS="--deny warnings" cargo build -r

FROM gcr.io/distroless/cc@sha256:56aaf20ab2523a346a67c8e8f8e8dabe447447d0788b82284d14ad79cd5f93cc

COPY --from=build /app/target/release/discord-minecraft-emoji-bot /

ENTRYPOINT ["/discord-minecraft-emoji-bot"]
