FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/app/target/release/based-ten /usr/local/bin/based-ten

ENTRYPOINT [ "/usr/local/bin/based-ten" ]

EXPOSE 8080