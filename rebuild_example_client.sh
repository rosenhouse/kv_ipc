#!/bin/bash

set -e -u -o pipefail

cd "$(dirname "$0")"

# Mac
# cc -I ./include -L ./target/release -lkv_ipc -o ./target/release/example_client ./src/bin/example_client.c

cc -I ./include -L ./target/release -o ./target/release/example_client ./src/bin/example_client.c -lkv_ipc -lpthread -ldl -lm
