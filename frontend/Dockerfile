FROM rustlang/rust:nightly AS builder

RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get install build-essential zlib1g-dev -y

WORKDIR /app_build

COPY src src
COPY static static
COPY templates templates
COPY Cargo.toml .

RUN cargo build --release

FROM ubuntu
RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get -y install libssl-dev

WORKDIR /app
COPY --from=builder /app_build/target/release/color-frontend .
COPY templates templates
COPY static static
CMD ["/app/color-frontend"]