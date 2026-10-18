# syntax=docker/dockerfile:1
# Le site dans une image : docker build -t docs . && docker run --rm -p 8080:8080 docs
# Si des dépendances git sont privées : --secret id=repos_token,env=REPOS_TOKEN
FROM rust:1-slim-bookworm AS build
RUN apt-get update && apt-get install -y --no-install-recommends git ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY . .
RUN --mount=type=secret,id=repos_token,required=false \
    if [ -s /run/secrets/repos_token ]; then \
      export REPOS_TOKEN="$(cat /run/secrets/repos_token)" CARGO_NET_GIT_FETCH_WITH_CLI=true; \
      git config --global credential.https://github.com.helper \
        '!f() { test "$1" = get && echo username=x-access-token && echo "password=$REPOS_TOKEN"; }; f'; \
    fi; \
    cargo build --release --locked && cp target/release/docs /server

FROM debian:bookworm-slim
RUN useradd --system --uid 10001 docs
WORKDIR /app
COPY --from=build /server /usr/local/bin/docs
COPY --from=build /src/public ./public
USER docs
ENV PORT=8080
EXPOSE 8080
CMD ["docs"]
