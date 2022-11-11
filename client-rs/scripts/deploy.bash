#!/usr/bin/env bash 

anchor build

# Deploy Universe
solana program deploy target/deploy/ecs.so

# Deploy Dominari World
solana program deploy target/deploy/dominariworld.so

# Deploy Dominari System
solana program deploy target/deploy/dominarisystems.so



#anchor deploy --program-name dominarisystems --program-keypair target/deploy/dominarisystems-keypair.json
