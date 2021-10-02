# 開発環境
FROM rust:1.55 as develop-stage
WORKDIR /app
RUN apt-get update && apt-get install libmariadb-dev
RUN cargo install cargo-watch
# RUN cargo install diesel_cli --no-default-features --features mysql
COPY . .

# ビルド環境
FROM develop-stage as build-stage
RUN cargo build --release

# 本番環境
FROM rust:1.55
COPY --from=build-stage /app/target/release/backend .
EXPOSE 5000
CMD ["/usr/local/bin/backend"]
