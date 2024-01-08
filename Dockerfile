FROM rust:latest as builder
WORKDIR /app
ENV SQLX_OFFLINE true
COPY . .
RUN cargo build --release

FROM rust:latest
WORKDIR /app
COPY --from=builder /app /app
CMD ["cargo","run","--release"]