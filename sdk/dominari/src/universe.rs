use anchor_client::Program;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::{signature::Keypair};
use std::rc::Rc;

pub struct Universe {
    pub program: Program
}

impl Universe {
    pub fn new(rpc:&str, wss: &str, mut keypair: Option<Keypair>) -> Self {
        if keypair.is_none() {
            keypair = Some(Keypair::new());
        }

        let payer = Rc::new(keypair.unwrap());
        let program = anchor_client::Client::new_with_options(anchor_client::Cluster::Custom(rpc.to_string(), wss.to_string()), payer, CommitmentConfig::confirmed()).program(ecs::id());
        
        return Universe {
            program,
        };
    }
}
