# Build recipe
FROM rust as plan
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build dependecies
FROM rust as cache
WORKDIR /app
RUN apt update && \
    apt install -y cmake
RUN cargo install cargo-chef
COPY --from=plan /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin server

# Build small image
FROM gcr.io/distroless/cc-debian11
COPY --from=cache /app/target/release/server /app/server
WORKDIR /app
CMD ["./server"]
