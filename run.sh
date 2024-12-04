#!/bin/bash
cargo build --release
mv target/release/rust_server ./
nohup ./rust_server > output.log 2>&1 &