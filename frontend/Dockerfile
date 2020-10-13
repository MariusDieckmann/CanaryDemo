FROM rustlang/rust:nightly-alpine AS builder


RUN apk update
RUN apk add build-base
RUN apk add openssl-dev

WORKDIR /app_build

COPY . .
RUN cargo build

FROM ubuntu
RUN apt-get -y update
RUN apt-get -y upgrade

WORKDIR /app
COPY --from=builder /app_build/target/debug/color-frontend .
COPY templates .
COPY static .
CMD ["/app/color-frontend"]