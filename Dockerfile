# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.91.0
ARG APP_NAME=chatty_audio

FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.3.0 AS xx

FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
ARG TARGETPLATFORM
WORKDIR /app
COPY --from=xx / /

RUN apk add --no-cache clang lld build-base musl-dev pkgconfig libstdc++ git file ffmpeg yt-dlp

RUN xx-apk add --no-cache musl-dev gcc

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/,id=rust-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    xx-cargo build --locked --release --target-dir ./target && \
    cp ./target/$(xx-cargo --print-target-triple)/release/$APP_NAME /bin/server && \
    xx-verify /bin/server

FROM alpine:3.18 AS final
ARG UID=10001
RUN adduser -D -u $UID appuser
RUN apk add --no-cache ffmpeg yt-dlp ca-certificates
COPY --from=build /bin/server /bin/server
USER appuser
EXPOSE 3000
CMD ["/bin/server"]
