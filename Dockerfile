FROM ekidd/rust-musl-builder:nightly-2020-04-10 AS builder

RUN USER=rust cargo new mediaproxy
RUN USER=rust cargo new mediaproxy-router
RUN USER=rust cargo new mediaproxy-lib --lib

COPY --chown=rust:rust Cargo.* ./
COPY --chown=rust:rust mediaproxy/Cargo.* mediaproxy/
COPY --chown=rust:rust mediaproxy-router/Cargo.* mediaproxy-router/
COPY --chown=rust:rust mediaproxy-lib/Cargo.* mediaproxy-lib/

RUN cargo build --release

RUN rm -r target/x86_64-unknown-linux-musl/release/deps/mediaproxy*
RUN rm -r target/x86_64-unknown-linux-musl/release/deps/libmediaproxy*
RUN rm -r mediaproxy/src mediaproxy-router/src mediaproxy-lib/src

COPY --chown=rust:rust mediaproxy mediaproxy
COPY --chown=rust:rust mediaproxy-router mediaproxy-router
COPY --chown=rust:rust mediaproxy-lib mediaproxy-lib

RUN cargo build --release

FROM alpine:latest AS mediaproxy
RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/mediaproxy /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/mediaproxy", "--listen", "0.0.0.0:80"]

FROM alpine:latest AS mediaproxy-router
RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/mediaproxy-router /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/mediaproxy-router", "--listen", "0.0.0.0:80"]