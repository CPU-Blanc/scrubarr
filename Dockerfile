FROM rust:1.82.0-alpine AS builder
WORKDIR /usr/src/scrubarr
COPY . .
RUN cargo add openssl --features vendored
RUN apk add make musl-dev perl
RUN cargo install --path ./client

FROM alpine
RUN apk update && rm -rf /var/cache/apk/*
COPY --from=builder /usr/local/cargo/bin/scrubarr /usr/local/bin/scrubarr
CMD ["scrubarr"]