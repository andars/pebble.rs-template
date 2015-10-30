#!/bin/bash

set -x
set -u
set -e
set -k

cargo build -p pebble --target arm-none-eabi --release

#this is pretty hacky but seems to work
#  i'm sure there is a better way
#  but this allows me to still use pebble's waf system
cd target/arm-none-eabi/release/deps
ar xv *.a
cd -

pebble build
