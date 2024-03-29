FROM rust:1.73-slim as base

WORKDIR /analyzer

RUN apt-get update && \
    apt-get install -y musl musl-dev musl-tools && \
    rustup target add x86_64-unknown-linux-musl

COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl && \
    cp target/x86_64-unknown-linux-musl/release/rust-analyzer ./bin/

FROM alpine:3.18.4

WORKDIR /opt/analyzer

COPY --from=base /analyzer/bin/* ./bin/

# Adding the cargo binary to run clippy
COPY --from=base /usr/local/rustup/toolchains/1.73.0-x86_64-unknown-linux-gnu/bin/cargo ./cargo

ENTRYPOINT ["bin/run.sh"]
