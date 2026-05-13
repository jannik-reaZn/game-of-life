FROM rust:1.94.0 AS builder

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["game-of-life"]