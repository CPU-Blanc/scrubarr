FROM rust:1.93.1-alpine AS builder
WORKDIR /usr/src/scrubarr
COPY . .
RUN cargo add openssl --features vendored
RUN apk add make musl-dev perl
RUN cargo install --path .

FROM alpine
RUN apk update && rm -rf /var/cache/apk/*
COPY --from=builder /usr/local/cargo/bin/scrubarr /usr/local/bin/scrubarr
ENV X_SCRUBARR_CONFIG="/config/settings.json"
VOLUME /config
CMD ["scrubarr"]