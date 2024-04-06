FROM rust:1.77.1-bullseye as builder
WORKDIR /builder

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /builder/target/release/test_logger /

CMD ["./test_logger"]