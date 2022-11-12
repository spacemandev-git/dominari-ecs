#!/usr/bin/env bash 

# Deploy the programs
./scripts/deploy.bash

# Initalize the programs
cargo run register

# Setup 8x8 Map
cargo run map 1 8 8
