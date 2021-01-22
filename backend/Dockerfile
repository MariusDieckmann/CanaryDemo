FROM rustlang/rust:nightly-slim AS builder

WORKDIR /app_build

RUN rustup target add x86_64-unknown-linux-musl

COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release


FROM scratch

WORKDIR /app
COPY --from=builder /app_build/target/x86_64-unknown-linux-musl/release/color-backend .
WORKDIR /

CMD ["/app/color-backend"]