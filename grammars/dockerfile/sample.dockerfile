# syntax=docker/dockerfile:1
ARG VERSION=1.98
FROM rust:${VERSION}-slim AS build
WORKDIR /src
ENV CARGO_TERM_COLOR=always \
    RUSTFLAGS="-C target-cpu=native"
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release && \
    strip target/release/app

FROM debian:bookworm-slim
LABEL org.opencontainers.image.title="sample"
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /src/target/release/app /usr/local/bin/app
EXPOSE 8080/tcp
USER 1000:1000
HEALTHCHECK --interval=30s CMD ["app", "--health"]
ENTRYPOINT ["app"]
CMD ["--port", "8080"]
