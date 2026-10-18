# Le site dans une image : docker build -t docs . && docker run --rm -p 8080:8080 docs
FROM rust:1-slim-bookworm AS build
RUN apt-get update && apt-get install -y --no-install-recommends git ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY . .
RUN cargo build --release --locked && cp target/release/docs /server

FROM debian:bookworm-slim
RUN useradd --system --uid 10001 docs
WORKDIR /app
COPY --from=build /server /usr/local/bin/docs
COPY --from=build /src/public ./public
USER docs
ENV PORT=8080
EXPOSE 8080
CMD ["docs"]
