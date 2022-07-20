# service-template-rs

Template repository for creating a microservice using grpc in rust

## Contents

This template repository contains a small set of dependencies to get started with a microservice using grpc and includes logging. This includes

- tonic
- prost
- tokio
- log
- env_logger

## Run the project

This can be run either with or without docker.

Commands can be run using [just](https://github.com/casey/just) or copy the commands from the `.justfile`.

Start the server

> just server

Make a call to the server using the client

> just client

If you want to run the server in a docker container run

> just dockerize

then run the client to communicate with the server running on the container
