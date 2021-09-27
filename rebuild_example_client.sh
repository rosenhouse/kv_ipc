#!/bin/bash

set -e -u -o pipefail

cd "$(dirname "$0")"

# Mac
# cc -I ./include -L ./target/debug -lkv_ipc -o ./target/debug/example_client ./src/bin/example_client.c

cc -I ./include -L ./target/debug -o ./target/debug/example_client ./src/bin/example_client.c -lkv_ipc -lpthread -ldl -lm
