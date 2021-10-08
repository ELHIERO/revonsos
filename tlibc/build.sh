#!/bin/bash
set -e
set -x

# capture all output to a file
# script -e .script_output

THIS_DIR="$(dirname "$(readlink -f "$0")")"
REVONSOS_BASE_DIR=$THIS_DIR/..
REVONSOS_CARGO_PATH="$REVONSOS_BASE_DIR/tools/revonsos_cargo"
REVONSOS_DEPS_DIR="$REVONSOS_BASE_DIR/build/deps"

export RUST_BACKTRACE=1

### Note: the "revonsos_cargo" tool must be installed locally instead of invoked via `cargo run` 
cargo install --force --path=$REVONSOS_CARGO_PATH --root=$REVONSOS_CARGO_PATH

### Do a full clean build every time at this point
cargo clean

### Use revonsos_cargo to build this cargo package (tlibc) 
### with an automatic configuration that builds it to depend against pre-built RevonsOs crates.
$REVONSOS_CARGO_PATH/bin/revonsos_cargo  --input $REVONSOS_DEPS_DIR  build

### Create a library archive (.a) file from all of the tlibc crate object files.
### Note: it's better to do a partial link, using `ld -r` below.
ar -rcs ./target/x86_64-revonsos/release/libtlibc.a ./target/x86_64-revonsos/release/deps/*.o 

### Create a partially-linked object (.o) file from all of the tlibc crate object files. 
ld -r -o  ./target/x86_64-revonsos/release/tlibc.o ./target/x86_64-revonsos/release/deps/*.o

### Attempt to statically link everything together in a way we can overwrite the relocations later. 
# reset
# ld --emit-relocs -o  ./target/x86_64-revonsos/release/tlibc_static  \
#     -u main  \
#     ./target/x86_64-revonsos/release/deps/*.o  \
#     $REVONSOS_BASE_DIR/target/x86_64-revonsos/release/libnano_core.a \
#     $REVONSOS_BASE_DIR/target/x86_64-revonsos/release/deps/*.o \


    

