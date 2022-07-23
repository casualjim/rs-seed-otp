FROM rust:latest as cross

WORKDIR /usr/src/seed-otp

RUN rustup component add rustfmt
RUN rustup target add x86_64-unknown-linux-gnu
RUN apt-get update -y && apt-get install -y unzip

COPY Cargo.toml .
COPY Cargo.lock .

COPY src src
COPY wordlists wordlists

RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=cross /usr/src/seed-otp/target/release/seed-otp /
ENTRYPOINT ["./seed-otp"]