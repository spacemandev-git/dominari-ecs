#!/usr/bin/env bash 

cd ../
anchor build

# Deploy Universe
solana program deploy --program-id keypairs/ecs-keypair.json target/deploy/ecs.so

# Deploy Dominari World
solana program deploy --program-id keypairs/dominariworld-keypair.json target/deploy/dominariworld.so

# Deploy Dominari System
solana program deploy --program-id keypairs/dominarisystems-keypair.json target/deploy/dominarisystems.so



#anchor deploy --program-name dominarisystems --program-keypair target/deploy/dominarisystems-keypair.json
