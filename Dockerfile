FROM rust:1.82-slim-bookworm AS base

ARG build_arg

RUN apt update
RUN apt upgrade -y
RUN apt install -y pkg-config libpq-dev libssl-dev

RUN cargo install sccache
RUN cargo install cargo-chef
RUN cargo install diesel_cli --no-default-features --features postgres

ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache/


FROM base AS planner

WORKDIR /app/

COPY ./src/ ./src/
COPY ./Cargo.toml ./
COPY ./migrations/ ./migrations/

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json


FROM base AS builder

WORKDIR /app/

COPY --from=planner /app/recipe.json recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

COPY ./src/ ./src/
COPY ./Cargo.toml ./
COPY ./migrations/ ./migrations/

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build $build_arg

CMD cargo run $build_arg
