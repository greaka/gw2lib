VERSION 0.6
FROM earthly/dind:ubuntu

tools:
  RUN apt-get update
  RUN apt-get install -y --no-install-recommends build-essential
  RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  ENV PATH="/root/.cargo/bin:${PATH}"
  RUN cargo install cargo-nextest

  SAVE IMAGE --cache-hint tools

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

  COPY --dir proxy/src ./proxy/
  COPY --dir http/src ./http/
  COPY --dir model/src ./model/
  RUN --mount=type=cache,target=proxy/target \
    cd proxy && \
    cargo build --release && \
    mv target/release/proxy ../artifact

  SAVE ARTIFACT artifact /proxy

prefetch:
  FROM +tools

  COPY Cargo.toml ./
  COPY model/Cargo.toml ./model/
  COPY http/Cargo.toml ./http/
  RUN cargo fetch

  SAVE IMAGE prefetch

build-tests:
  FROM +prefetch
  
  DO +COPY_SRC
  
  RUN --mount=type=cache,target=target \
    cargo nextest archive --archive-file tests.tar.zst

  SAVE ARTIFACT tests.tar.zst /tests.tar.zst

docker-proxy:
  FROM gcr.io/distroless/cc-debian11
  
  COPY +build-proxy/proxy /proxy
  
  CMD ["/proxy"]

  SAVE IMAGE --cache-hint gw2api-proxy

test:
  FROM +tools

  DO +BASE_TESTS

  WITH DOCKER --compose integration-compose.yml --load gw2api-proxy=+docker-proxy
    RUN --no-cache cargo nextest run --archive-file tests.tar.zst
  END

test-ignored:
  FROM +tools

  DO +BASE_TESTS

  WITH DOCKER --compose integration-compose.yml --load gw2api-proxy=+docker-proxy
    RUN --no-cache cargo nextest run --archive-file tests.tar.zst --run-ignored ignored-only
  END

test-all:
  FROM +tools

  DO +BASE_TESTS

  WITH DOCKER --compose integration-compose.yml --load gw2api-proxy=+docker-proxy
    RUN --no-cache cargo nextest run --archive-file tests.tar.zst && \
        cargo nextest run --archive-file tests.tar.zst --run-ignored ignored-only
  END

BASE_TESTS:
  COMMAND

  COPY integration-compose.yml ./
  COPY Cargo.toml ./
  DO +COPY_SRC
  COPY +build-proxy/proxy /proxy
  COPY +build-tests/tests.tar.zst ./

COPY_SRC:
  COMMAND
  
  COPY --dir .config ./
  COPY --dir http ./
  COPY --dir model ./
