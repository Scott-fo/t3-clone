FROM lukemathwalker/cargo-chef:latest-rust-1.87-alpine AS chef_base

RUN apk add --no-cache \
      build-base \
      pkgconf \
      mariadb-connector-c-dev mariadb-static \
      openssl-dev openssl-libs-static zlib-static

ENV MYSQLCLIENT_STATIC=1 \
    OPENSSL_STATIC=1 \
    PKG_CONFIG_ALL_STATIC=1

FROM chef_base AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef_base AS rust-builder
ARG BIN_NAME=web
WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

RUN cargo install diesel_cli --no-default-features --features "mysql"

# Copy the application source and build it.
COPY . .
RUN cargo build --release --bin ${BIN_NAME}

RUN strip target/release/${BIN_NAME}

FROM oven/bun:latest AS frontend-builder
WORKDIR /frontend

COPY frontend/package.json frontend/bun.lock ./
RUN bun install --frozen-lockfile

COPY frontend ./

RUN --mount=type=secret,id=REPLICACHE_KEY \
    echo "VITE_REPLICACHE_KEY=$(cat /run/secrets/REPLICACHE_KEY)" > .env

RUN bun run build

FROM gcr.io/distroless/static-debian11:nonroot AS final

WORKDIR /app

COPY --from=rust-builder /app/target/release/web ./bin/web
COPY --from=frontend-builder /frontend/build/client ./frontend/build/client

EXPOSE 80
USER nonroot:nonroot

CMD ["/app/bin/web"]
