FROM rust:latest AS builder

WORKDIR /usr/src

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

# Install the required libraries
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/target/release/based-ten /usr/local/bin/based-ten

COPY .env /usr/local/bin/.env

WORKDIR /usr/local/bin

ENTRYPOINT [ "/usr/local/bin/based-ten" ]

EXPOSE 5000