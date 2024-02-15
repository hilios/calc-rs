FROM rust:alpine3.18 as builder
WORKDIR app
COPY . .
RUN cargo build --release --bin calc-rs

FROM rust:alpine3.18 as runtime
WORKDIR app
COPY --from=builder /app/target/release/calc-rs /usr/local/bin
ENTRYPOINT ["./usr/local/bin/calc-rs"]