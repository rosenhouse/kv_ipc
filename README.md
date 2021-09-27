# kv_ipc

Exploring a couple things in Rust:
- C program uses a Rust library
- Unix domain datagram sockets for interprocess communication

## Components
- `./src/bin/example_client.c`: a C program that calls the Rust library
- `./include/libkv_ipc_client.h`: the C header for the Rust library
- `./src/lib.rs`: Rust library
- `./src/bin/example_server.rs`: a Rust program that receives messages from the C program

## Building and testing
Build the components
```sh
cargo install cbindgen
cargo build
./regenerate_c_header.sh
./rebuild_example_client.sh
```

Launch the server, listening on a socket
```
./target/debug/example_server /tmp/my-socket
```

Send a message from the C program to that socket
```
./target/debug/example_client /tmp/my-socket
```

