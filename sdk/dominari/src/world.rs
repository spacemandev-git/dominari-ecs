
use anchor_client::Program;
use anchor_client::anchor_lang::system_program::ID as system_program;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::instruction::Instruction;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::{signature::Keypair};
use std::rc::Rc;
use std::str::FromStr;


pub struct World {
    pub program: Program,
}

impl World {
    pub fn new(rpc:&str, wss: &str, mut keypair: Option<Keypair>) -> Self {
        if keypair.is_none() {
            keypair = Some(Keypair::new());
        }

        let payer = Rc::new(keypair.unwrap());
        let program = anchor_client::Client::new_with_options(anchor_client::Cluster::Custom(rpc.to_string(), wss.to_string()), payer, CommitmentConfig::confirmed()).program(dominariworld::id());
        
        return World {
            program,
        };
    }


    pub fn initialize(&self, universe_address:&str, payer: Pubkey) -> Result<Vec<Instruction>, anchor_client::ClientError>{
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &self.program.id()).0;
        
        self.program
            .request()
            .accounts(dominariworld::accounts::Initialize {
                payer,
                system_program,
                world_config
            })
            .args(dominariworld::instruction::Initalize {
                universe: Pubkey::from_str(universe_address).unwrap()
            })
            .instructions()
    }
    
}