VERSION 0.6
FROM earthly/dind:ubuntu-24.04-docker-27.3.1-1

tools:
  RUN apt-get update
  RUN apt-get install -y --no-install-recommends build-essential
  RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  ENV PATH="/root/.cargo/bin:${PATH}"
  RUN cargo --color=always install cargo-nextest --locked

  SAVE IMAGE --cache-hint tools

prefetch:
  FROM +tools

  COPY Cargo.toml ./
  COPY model/Cargo.toml ./model/
  COPY http/Cargo.toml ./http/
  COPY keys/Cargo.toml ./keys/
  RUN cargo --color=always fetch

  SAVE IMAGE prefetch

build-tests:
  FROM +prefetch
  
  DO +COPY_SRC
  
  RUN cargo --color=always nextest archive --archive-file tests.tar.zst --features=blocking,redis

  SAVE ARTIFACT tests.tar.zst /tests.tar.zst

test:
  FROM +tools

  DO +BASE_TESTS

  WITH DOCKER --compose integration-compose.yml
    RUN --secret GW2_API_KEY --secret GW2_TESTING_CHAR --no-cache cargo --color=always \
        nextest run --archive-file tests.tar.zst
  END

test-ignored:
  FROM +tools

  DO +BASE_TESTS

  WITH DOCKER --compose integration-compose.yml
    RUN --secret GW2_API_KEY --secret GW2_TESTING_CHAR --no-cache cargo --color=always \
        nextest run --archive-file tests.tar.zst --run-ignored ignored-only
  END

test-all:
  FROM +tools

  DO +BASE_TESTS

  WITH DOCKER --compose integration-compose.yml
    RUN --secret GW2_API_KEY --secret GW2_TESTING_CHAR --no-cache cargo --color=always \
        nextest run --archive-file tests.tar.zst && \
        cargo --color=always nextest run --archive-file tests.tar.zst --run-ignored ignored-only
  END

BASE_TESTS:
  COMMAND

  COPY integration-compose.yml ./
  COPY Cargo.toml ./
  DO +COPY_SRC
  COPY +build-tests/tests.tar.zst ./

COPY_SRC:
  COMMAND
  
  COPY --dir .config ./
  COPY --dir http ./
  COPY --dir model ./
  COPY --dir keys ./
