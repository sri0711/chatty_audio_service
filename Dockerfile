# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.91.0
ARG APP_NAME=chatty_audio

FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.3.0 AS xx

FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
ARG TARGETPLATFORM
WORKDIR /app

# Copy cross compilation helpers
COPY --from=xx / /

# Install host and target build dependencies
RUN apk add --no-cache \
    clang lld build-base musl-dev pkgconfig \
    openssl-dev libstdc++ git file

# FFmpeg & yt-dlp if needed
RUN apk add --no-cache ffmpeg yt-dlp

# Install target-specific musl libraries (needed for xx cross builds)
RUN xx-apk add --no-cache musl-dev gcc openssl-dev

# Set environment variables so openssl-sys finds OpenSSL
ENV OPENSSL_DIR=/usr \
    OPENSSL_INCLUDE_DIR=/usr/include \
    OPENSSL_LIB_DIR=/usr/lib \
    OPENSSL_STATIC=1

# Build app
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/,id=rust-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    xx-cargo build --locked --release --target-dir ./target && \
    cp ./target/$(xx-cargo --print-target-triple)/release/$APP_NAME /bin/server && \
    xx-verify /bin/server

# ------------------- Runtime -------------------
FROM alpine:3.18 AS final

ARG UID=10001
RUN adduser -D -u $UID appuser

COPY --from=build /bin/server /bin/server
RUN apk add --no-cache openssl ffmpeg yt-dlp

USER appuser
EXPOSE 3000
CMD ["/bin/server"]
