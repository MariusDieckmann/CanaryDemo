FROM rustlang/rust:nightly-alpine AS builder


RUN apk update
RUN apk add build-base
RUN apk add openssl-dev
RUN rustup target add x86_64-unknown-linux-musl --toolchain=nightly

WORKDIR /app_build

ENV RUSTFLAGS="-C target-feature=+crt-static"
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine
RUN apk update

WORKDIR /app
COPY --from=builder /app_build/target/x86_64-unknown-linux-musl/release/color-frontend .
COPY templates .
COPY static .
CMD ["/app/color-frontend"]