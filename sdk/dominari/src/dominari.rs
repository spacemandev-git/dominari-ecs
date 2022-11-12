use anchor_client::solana_sdk::instruction::Instruction;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::{Program, solana_sdk::signature::Keypair};
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::anchor_lang::system_program::ID as system_program;

use std::rc::Rc;

pub struct Dominari {
    pub program: Program,
}

impl Dominari {
    pub fn new(rpc:&str, wss: &str, mut keypair: Option<Keypair>) -> Self {
        if keypair.is_none() {
            keypair = Some(Keypair::new());
        }

        let payer = Rc::new(keypair.unwrap());
        let program = anchor_client::Client::new_with_options(anchor_client::Cluster::Custom(rpc.to_string(), wss.to_string()), payer, CommitmentConfig::confirmed()).program(dominarisystems::id());
        
        return Dominari {
            program,
        };
    }

    pub fn get_system_signer(&self) -> Pubkey {
        Pubkey::find_program_address(&[b"System_Signer"], &self.program.id()).0
    }

    /* 
    pub fn initalize_map(&self, payer: Pubkey, instance:u64, max_x: u8, max_y:u8) -> Result<Vec<Instruction>, anchor_client::ClientError> {
        let system_signer = Pubkey::find_program_address(&[
            b"System_Signer",
        ], &self.program.id()).0;

        
        self.program
                .request()
                .accounts(dominarisystems::accounts::SystemInitMap {
                    payer,
                    system_program,
                    system_signer,

                })
                .args(dominarisystems::instruction::SystemInitalizeMap {
                    max_x,
                    max_y,
                })
                .instructions()
       
    }
    
    */
}