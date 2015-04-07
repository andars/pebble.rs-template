#!/bin/bash

set -x
set -u
set -e
set -k

cargo build -p pebble --target arm-none-eabi --release

cd target/arm-none-eabi/release/deps
ar xv *.a
cd -

pebble build
