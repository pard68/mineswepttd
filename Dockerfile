FROM rust as builder
WORKDIR /src
COPY . .
RUN cargo build --release

FROM debian:buster-slim as runner
COPY --from=builder /src/target/release/mineswepttd /usr/local/bin/mineswepttd
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
ENTRYPOINT ["mineswepttd"]
