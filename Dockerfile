FROM rust:1.79.0-bullseye AS builder
WORKDIR /builder

COPY . .

RUN --mount=type=secret,id=SECRET_LOG \
	SECRET_LOG=$(cat /run/secrets/SECRET_LOG) && \
	echo $SECRET_LOG

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /builder/target/release/test_logger /

CMD ["./test_logger"]