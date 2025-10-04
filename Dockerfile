FROM --platform=$BUILDPLATFORM rust:latest as cross

ARG TARGETARCH

WORKDIR /usr/src/seed-otp
COPY hack/platform.sh .

RUN ./platform.sh
RUN rustup component add rustfmt
RUN rustup target add "$(cat /.platform)"

RUN apt-get update -y && apt-get install -y unzip $(cat /.compiler) && mkdir -p /out

COPY Cargo.toml .
COPY Cargo.lock .
COPY .cargo/config.toml .cargo/config.toml
COPY src src
COPY wordlists wordlists

RUN --mount=type=tmpfs,destination=/usr/local/cargo/registry/ \
  --mount=type=tmpfs,destination=/usr/local/cargo/git/ \
  cargo build --locked --release --target "$(cat /.platform)"
RUN cp target/$(cat /.platform)/release/seed-otp /out

FROM --platform=$TARGETPLATFORM gcr.io/distroless/cc
COPY --from=cross /out/seed-otp /
ENTRYPOINT ["/seed-otp"]
