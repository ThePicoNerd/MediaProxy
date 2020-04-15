FROM ekidd/rust-musl-builder AS builder
RUN sudo apt-get update -y
RUN USER=rust:rust cargo init . --name mediaproxy
COPY --chown=rust:rust Cargo.* ./
RUN cargo build --release
RUN rm -r src target/x86_64-unknown-linux-musl/release/deps/mediaproxy*
COPY --chown=rust:rust ./src ./src
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/mediaproxy /usr/local/bin/
ENV ADDRESS 0.0.0.0:80
CMD /usr/local/bin/mediaproxy