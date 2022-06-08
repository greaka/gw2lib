VERSION 0.6
FROM docker.io/rust:slim-bullseye

tools:
  RUN cargo install cargo-nextest

  SAVE IMAGE tools

prefetch-proxy:
  FROM +tools

  COPY Cargo.toml ./
  COPY model/Cargo.toml ./model/
  COPY http/Cargo.toml ./http/
  COPY proxy/Cargo.toml ./proxy/
  RUN cd proxy && cargo fetch

  SAVE IMAGE prefetch-proxy

build-proxy:
  FROM +prefetch-proxy

  COPY --dir proxy ./
  COPY --dir http ./
  COPY --dir model ./
  RUN cd proxy && cargo build --release

  SAVE ARTIFACT proxy/target/release/proxy /proxy

prefetch:
  FROM +tools

  COPY Cargo.toml ./
  COPY model/Cargo.toml ./model/
  COPY http/Cargo.toml ./http/
  RUN cargo fetch

  SAVE IMAGE prefetch

build-tests:
  FROM +prefetch
  
  COPY --dir .config ./
  COPY --dir http ./
  COPY --dir model ./

  RUN cargo nextest archive --archive-file tests.tar.zst

  SAVE ARTIFACT tests.tar.zst /tests.tar.zst

test:
  FROM +tools

  DO +BASE_TESTS

test-all:
  FROM +tools

  DO +BASE_TESTS
  RUN --push cargo nextest run --archive-file tests.tar.zst --run-ignored ignored-only

BASE_TESTS:
  COMMAND
  COPY +build-proxy/proxy /proxy
  COPY +build-tests/tests.tar.zst ./
  RUN --push /proxy &
  RUN --push cargo nextest run --archive-file tests.tar.zst
