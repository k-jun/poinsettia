FROM rust:1.46 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.toml
# COPY Cargo.lock Cargo.lock
RUN mkdir -p src/bin
RUN echo "fn main(){}" > src/bin/main.rs
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl --bin main

COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl --bin server
RUN strip ./target/x86_64-unknown-linux-musl/release/server

# for quick check
# CMD ["./target/x86_64-unknown-linux-musl/release/server", "-h", "0.0.0.0"]


# for release build
FROM alpine:3.11
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server /bin/poinsettia
CMD ["/bin/poinsettia", "-h", "0.0.0.0"]
