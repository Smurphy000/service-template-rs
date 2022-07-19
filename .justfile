alias d := dockerize

dockerize:
    docker build -t server .
    docker run -p 50051:50051 server

server:
    RUST_LOG=server cargo run --bin server
    
client:
    RUST_LOG=client cargo run --bin client
