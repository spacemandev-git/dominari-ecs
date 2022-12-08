#!/usr/bin/env bash 

set -e

# Deploy the programs
./scripts/deploy.bash

# Initalize the programs (init world, components, action bundle)
cargo run initialize

# Register blueprints
cargo run blueprints blueprints/features
cargo run blueprints blueprints/units
cargo run blueprints blueprints/mods

# Setup 8x8 Map
cargo run setup_game configs/2player.toml 1
# cargo run game 1