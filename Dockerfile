FROM rust:alpine AS build

WORKDIR /app
COPY . /app
RUN apk add --no-cache musl-dev && \
    cargo build --release


FROM gcr.io/distroless/static-debian13

WORKDIR /app

ENV RUST_LOG=info

COPY --from=build /app/target/release/update-proxy /app/update-proxy

ENTRYPOINT ["/app/update-proxy"]
CMD []