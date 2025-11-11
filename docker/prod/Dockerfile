FROM rust:1.91.0-slim-bullseye AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:13-slim AS prod

ARG USER=afrodite-auth-service

WORKDIR /home/app

COPY --from=builder /app/target/release/afrodite-auth-service /usr/local/bin/afrodite-auth-service

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    $USER \
    && chown -R $USER:$USER /home/$USER \
    && touch .env

USER $USER

CMD ["/usr/local/bin/afrodite-auth-service"]