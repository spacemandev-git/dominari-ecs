
use anchor_client::Program;
use anchor_client::anchor_lang::system_program::ID as system_program;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::instruction::Instruction;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::{signature::Keypair};
use dominarisystems::accounts;
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

    pub fn register_component(&self, schema:String, payer:Pubkey) -> Result<Vec<Instruction>, anchor_client::ClientError>{
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &self.program.id()).0;

        let component = Pubkey::find_program_address(&[
            schema.as_bytes().as_ref(),
        ], &self.program.id()).0;

        self.program
            .request()
            .accounts(dominariworld::accounts::RegisterComponent {
                payer,
                system_program,
                component,
                world_config
            })
            .args (dominariworld::instruction::RegisterComponent {
                schema,
            })
            .instructions()          
    }
    
    pub fn instance_world(&self, payer:Pubkey) -> Result<Vec<Instruction>, anchor_client::ClientError> {
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &self.program.id()).0;

        let world_config_acc:dominariworld::account::WorldConfig = self.get_world_config();

        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            &self.program.id().as_ref(),
            (world_config_acc.instances + 1_u64).to_be_bytes().as_ref()
        ], &world_config_acc.universe).0;

        let instance_authority = Pubkey::find_program_address(&[
            b"Instance_Authority".as_ref(),
            world_instance.to_bytes().as_ref()
        ], &self.program.id()).0;

        self.program
                .request()
                .accounts(dominariworld::accounts::InstanceWorld {
                    payer,
                    system_program,
                    world_config,
                    world_instance,
                    instance_authority,
                    universe: world_config_acc.universe,
                })
                .args(dominariworld::instruction::InstanceWorld {})
                .instructions()
    } 

    pub fn get_world_config(&self) -> dominariworld::account::WorldConfig {
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &self.program.id()).0;
        let world_config_acc:dominariworld::account::WorldConfig = self.program.account(world_config).unwrap();
        return world_config_acc;
    }

    pub fn register_system_for_component(&self, schema:String, system:Pubkey, instance:u64, payer:Pubkey) -> Result<Vec<Instruction>, anchor_client::ClientError> {
        let world_config = Pubkey::find_program_address(&[
            b"world_signer".as_ref(),
        ], &self.program.id()).0;

        let world_config_acc:dominariworld::account::WorldConfig = self.get_world_config();

        let world_instance = Pubkey::find_program_address(&[
            b"World".as_ref(),
            &self.program.id().as_ref(),
            instance.to_be_bytes().as_ref()
        ], &world_config_acc.universe).0;

        let instance_authority = Pubkey::find_program_address(&[
            b"Instance_Authority".as_ref(),
            world_instance.to_bytes().as_ref()
        ], &self.program.id()).0;

        let component = Pubkey::find_program_address(&[
            schema.as_bytes().as_ref(),
        ], &self.program.id()).0;

        let system_registration = Pubkey::find_program_address(&[
            b"System_Registration",
            component.to_bytes().as_ref(),
            world_instance.to_bytes().as_ref(),
            system.to_bytes().as_ref()
        ], &self.program.id()).0;

        self.program
                .request()
                .accounts(dominariworld::accounts::RegisterSystem {
                    payer,
                    system_program,
                    world_instance,
                    component,
                    instance_authority,
                    system_registration,
                    system,
                }) 
                .args(dominariworld::instruction::RegisterSystemForComponent {})
                .instructions()
    }

}

pub struct ComponentSchema {
    pub url: String
}

impl ComponentSchema {
    pub fn get_schemas() -> Vec<ComponentSchema> {
        vec![
            ComponentSchema { url: "metadata.json".to_string() },
            ComponentSchema { url: "mapmeta.json".to_string() },
            ComponentSchema { url: "location.json".to_string() },
            ComponentSchema { url: "feature.json".to_string() },
            ComponentSchema { url: "owner.json".to_string() },
            ComponentSchema { url: "value.json".to_string() },
            ComponentSchema { url: "occupant.json".to_string() },
            ComponentSchema { url: "player_stats.json".to_string() },
            ComponentSchema { url: "last_used_slot.json".to_string() },
            ComponentSchema { url: "rank.json".to_string() },
            ComponentSchema { url: "range.json".to_string() },
            ComponentSchema { url: "drop_table.json".to_string() },
            ComponentSchema { url: "uses.json".to_string() },
            ComponentSchema { url: "healing_power.json".to_string() },
            ComponentSchema { url: "health.json".to_string() },
            ComponentSchema { url: "damage.json".to_string() },
            ComponentSchema { url: "troop_class.json".to_string() },

        ]
    }
}