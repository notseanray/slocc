#!/bin/bash
cargo fmt
cargo build --release
echo "copying into /usr/bin directory"
echo "if you wish for another location, copy ./target/release/slocc to somewhere in the path"
doas cp target/release/slocc /usr/bin/
