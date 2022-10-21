#!/usr/bin/env bash
set -e

# run an endless loop of tests
while [ 1 ]; do 
	cargo test --release -- --nocapture
done
