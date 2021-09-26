# ipc-pair

Exploring a couple things in Rust:
- C program using a Rust library
- IPC in Rust

## Components
- `./example_client` is a C program that calls the Rust library
- `./include/libkv_ipc_client.h` is the C header for the Rust library
- `./src/ipc_client.rs` is the source for the Rust library, providing IPC

## Building the example client
```sh
cargo build
./regenerate_c_header.sh
cc -I ./include -L ./target/debug -lkv_ipc_client -o ./target/debug/example_client ./example_client/example_client.c
./target/debug/example_client
```

