use anchor_lang::{prelude::*, InstructionData};
use anchor_lang::system_program::ID as system_program;
use dominariworld::account::WorldConfig;
use solana_client_wasm::{WasmClient, solana_sdk::instruction::Instruction};
use crate::util::fetch_account;

pub struct World {
    pub client: WasmClient,
    pub pubkey: Pubkey,
}

impl World {
    pub fn new(rpc: &str, world: Pubkey) -> Self {
        return World {
            client: WasmClient::new(rpc),
            pubkey: world,
        }
    }

    pub fn get_default_program_id() -> Pubkey {
        dominariworld::id()
    }

    pub fn initialize(&self, payer: Pubkey) -> Vec<Instruction> {
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &self.pubkey).0;

        let accounts = dominariworld::accounts::Initialize {
            payer,
            system_program,
            world_config,
        };

        vec![Instruction {
            program_id: self.pubkey,
            accounts: accounts.to_account_metas(None),
            data: dominariworld::instruction::Initalize {
                universe: ecs::id(),
            }.data(),
        }]
    }

    pub fn register_component(&self, schema: &String, payer:Pubkey) -> Vec<Instruction> {
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &self.pubkey).0;

        let component = Pubkey::find_program_address(&[
            schema.as_bytes().as_ref(),
        ], &self.pubkey).0;

        vec![Instruction {
            program_id: self.pubkey,
            accounts: dominariworld::accounts::RegisterComponent {
                payer,
                system_program,
                component,
                world_config,
            }.to_account_metas(None),
            data: dominariworld::instruction::RegisterComponent {
                schema: schema.clone(),
            }.data()
        }]
    }

    pub async fn instance_world(&self, payer:Pubkey) -> Vec<Instruction> {
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &self.pubkey).0;

        let world_config_acc:dominariworld::account::WorldConfig = self.get_world_config().await.1;

        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            &self.pubkey.as_ref(),
            (world_config_acc.instances + 1_u64).to_be_bytes().as_ref()
        ], &world_config_acc.universe).0;

        let instance_authority = Pubkey::find_program_address(&[
            b"Instance_Authority".as_ref(),
            world_instance.to_bytes().as_ref()
        ], &self.pubkey).0;

        vec![Instruction {
            program_id: self.pubkey,
            accounts: dominariworld::accounts::InstanceWorld {
                payer,
                system_program,
                world_config, // Should be a CPI Signer
                world_instance,
                instance_authority,
                universe: ecs::id(),
            }.to_account_metas(Some(true)), // This will CPI into Universe program, so some of these accounts are signers
            data: dominariworld::instruction::InstanceWorld {}.data()
        }]
    } 
    
    pub async fn register_system(&self, system:Pubkey, instance:u64, payer:Pubkey) -> Vec<Instruction> {
        let world_config_acc:dominariworld::account::WorldConfig = self.get_world_config().await.1;

        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            &self.pubkey.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &world_config_acc.universe).0;

        let instance_authority = Pubkey::find_program_address(&[
            b"Instance_Authority".as_ref(),
            world_instance.to_bytes().as_ref()
        ], &self.pubkey).0;

        let system_registration = Pubkey::find_program_address(&[
            b"System_Registration",
            world_instance.to_bytes().as_ref(),
            system.to_bytes().as_ref()
        ], &self.pubkey).0;

        vec![Instruction {
            program_id: self.pubkey,
            accounts: dominariworld::accounts::RegisterSystem {
                payer,
                system_program,
                world_instance,
                instance_authority,
                system_registration,
                system,
            }.to_account_metas(None),
            data: dominariworld::instruction::RegisterSystem {}.data()
        }]
    }    

    pub async fn add_components_to_system_registration(&self, components: Vec<Pubkey>, system:Pubkey, instance:u64, payer: Pubkey) -> Vec<Instruction> {
        let world_config_acc:dominariworld::account::WorldConfig = self.get_world_config().await.1;

        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            &self.pubkey.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &world_config_acc.universe).0;

        let instance_authority = Pubkey::find_program_address(&[
            b"Instance_Authority".as_ref(),
            world_instance.to_bytes().as_ref()
        ], &self.pubkey).0;

        let system_registration = Pubkey::find_program_address(&[
            b"System_Registration",
            world_instance.to_bytes().as_ref(),
            system.to_bytes().as_ref()
        ], &self.pubkey).0;

        vec![Instruction {
            program_id: self.pubkey,
            accounts: dominariworld::accounts::AddComponentsToSystemRegistration {
                payer,
                system_program,
                world_instance,
                instance_authority,
                system_registration,
                system
            }.to_account_metas(None),
            data: dominariworld::instruction::AddComponentsToSystemRegistration {
                components,
            }.data()
        }]
    }


    /*******************************************************************Account Fetch */
    pub async fn get_world_config(&self) -> (Pubkey, dominariworld::account::WorldConfig) {
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &self.pubkey).0;
        let world_config_acc: WorldConfig = fetch_account(&self.client, &world_config).await.unwrap();
        return (world_config, world_config_acc);
    }

}