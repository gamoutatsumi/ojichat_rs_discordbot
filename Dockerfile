FROM ekidd/rust-musl-builder:stable AS builder

RUN git clone https://github.com/gamoutatsumi/ojichat_rust_discordbot .

RUN cargo build --release

FROM scratch

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/ojichat-disbot /

ENTRYPOINT ["/ojichat-disbot"]
