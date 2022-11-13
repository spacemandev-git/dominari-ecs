use anchor_lang::{prelude::*, InstructionData};
use anchor_lang::system_program::ID as system_program;
use solana_client_wasm::{WasmClient, solana_sdk::instruction::Instruction};

pub struct World {
    pub client: WasmClient,
}

impl World {
    pub fn new(rpc: &str) -> Self {
        return World {
            client: WasmClient::new(rpc)
        }
    }

    pub fn initialize(&self, payer: Pubkey) -> Vec<Instruction> {
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &dominariworld::id()).0;

        let accounts = dominariworld::accounts::Initialize {
            payer,
            system_program,
            world_config,
        };

        let data = &dominariworld::instruction::Initalize {
            universe: ecs::id(),
        }.data();

        println!("{:?}", data.len());

        vec![Instruction {
            program_id: dominariworld::id(),
            accounts: accounts.to_account_metas(None),
            data: data.clone(),
        }]
    }
}

/*
 
       let data = ([0_u8,0_u8,0_u8,0_u8,0_u8,0_u8,0_u8,0_u8], dominariworld::instruction::Initalize {
            universe: ecs::id(),
        }.try_to_vec().unwrap().as_slice()).try_to_vec().unwrap();

        println!("{:?}, {:#}", data, data.len());
        
        vec![Instruction {
            program_id: dominariworld::id(),
            accounts: accounts.to_account_metas(None),
            data,
        }]
 */