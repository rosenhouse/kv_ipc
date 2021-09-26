#!/bin/bash

set -e -u -o pipefail

cd "$(dirname "$0")"
cbindgen --config cbindgen.toml --output ./include/libkv_ipc_client.h
