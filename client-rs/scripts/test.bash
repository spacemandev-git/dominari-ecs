#!/usr/bin/env bash 

# Deploy the programs
./scripts/deploy.bash

# Initalize the programs (init world, components, action bundle)
cargo run initialize

# Register blueprints
cargo run blueprints blueprints/features
cargo run blueprints blueprints/units
cargo run blueprints blueprints/mods


# Instance the world
cargo run instance

# Setup 8x8 Map
cargo run map maps/01.toml