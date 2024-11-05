FROM rust:1.82-slim-bookworm AS build_container

ARG build_arg

WORKDIR /space_world/

RUN apt update
RUN apt upgrade -y
RUN apt install -y pkg-config libpq-dev libssl-dev


FROM build_container

COPY ./src/ ./src/
COPY ./Cargo.toml ./
COPY ./migrations/ ./migrations/

CMD cargo run $build_arg
