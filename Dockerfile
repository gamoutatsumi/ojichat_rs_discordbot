FROM ekidd/rust-musl-builder:stable AS builder

RUN git clone https://github.com/gamoutatsumi/ojichat_rs_discordbot.git .

RUN cargo build --release

FROM scratch

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/ojichat-rs-disbot /

ENTRYPOINT ["/ojichat-rs-disbot"]
