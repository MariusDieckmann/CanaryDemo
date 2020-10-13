FROM rustlang/rust:nightly-slim AS builder

RUN rustup target add x86_64-unknown-linux-musl --toolchain=nightly

WORKDIR /app_build

ENV RUSTFLAGS="-C target-feature=+crt-static"
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
WORKDIR /app
COPY --from=builder /app_build/target/x86_64-unknown-linux-musl/release/color-backend .
CMD ["/app/color-backend"]