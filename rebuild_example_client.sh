#!/bin/bash

set -e -u -o pipefail

cd "$(dirname "$0")"

cc -I ./include -L ./target/debug -lkv_ipc -o ./target/debug/example_client ./src/bin/example_client.c
