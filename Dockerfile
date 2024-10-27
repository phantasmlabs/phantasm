# Build the application
FROM rust:bullseye AS builder
RUN apt update && apt upgrade -y
RUN apt install -y protobuf-compiler libprotobuf-dev
WORKDIR /phantasm
COPY . .
RUN cargo build --release

# Build the application image
FROM debian:bullseye-slim
COPY --from=builder /phantasm/target/release/phantasm /usr/local/bin/phantasm
EXPOSE 2505 2510
ENTRYPOINT ["phantasm"]
