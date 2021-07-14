FROM rust as builder
WORKDIR src
COPY . .
RUN cargo build --release

FROM alpine as runner
COPY --from=builder /src/target/release/mineswepttd /
ENTRYPOINT ["/mineswepttd"]
