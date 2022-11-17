use anchor_lang::prelude::*;

pub mod account;
pub mod context;
pub mod constant;
pub mod error;
pub mod event;
pub mod component;
pub mod state;

//use account::*;
use context::*;
use constant::*;
//use error::*;
//use event::*;
use component::*;
use state::*;

use ecs::state::SerializedComponent;

declare_id!("3YdayPtujByJ1g1DWEUh7vpg78gZL49FWyD5rDGyof9T");

#[program]
pub mod dominarisystems {

    use super::*;

    /**
     * Sets the owner of the System, used to register Blueprints and such.
     */
    pub fn initialize(ctx: Context<Initialize>, component_keys: RelevantComponentKeys) -> Result<()> {
        ctx.accounts.system_signer.authority = ctx.accounts.payer.key();
        ctx.accounts.system_signer.components = component_keys;
        Ok(())
    }

    pub fn register_blueprint(ctx:Context<RegisterBlueprint>, components: Vec<SerializedComponent>, entity_name: String) -> Result<()> {
        ctx.accounts.blueprint.entity_name = entity_name;
        ctx.accounts.blueprint.components = components;
        Ok(())
    }

    pub fn system_initalize_map(ctx:Context<SystemInitMap>, entity_id:u64, max_x: u8, max_y: u8) -> Result<()> {
        let system_signer_seeds:&[&[u8]] = &[
            b"System_Signer",
            &[*ctx.bumps.get("system_signer").unwrap()]
        ];
        let signer_seeds = &[system_signer_seeds];

        let mint_entity_ctx = CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            dominariworld::cpi::accounts::MintEntity{
                entity: ctx.accounts.map_entity.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                world_instance: ctx.accounts.world_instance.to_account_info(),
                world_config: ctx.accounts.world_config.to_account_info(),
                system: ctx.accounts.system_signer.to_account_info(),
                system_registration: ctx.accounts.system_registration.to_account_info(),
                universe: ctx.accounts.universe.to_account_info(),
            },
            signer_seeds
        );

        let mut components: Vec<SerializedComponent> = vec![];
        // Map has Metadata and MapMeta Components
        let metadata_component = ComponentMetadata {
            name: format!("Map ({:#})", ctx.accounts.world_instance.instance),
            entity_type: "Map".to_string(),
            world_instance: ctx.accounts.world_instance.key(),
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.metadata.key(), 
            world: ctx.accounts.world_instance.key(), 
            max_size: STRING_MAX_SIZE + STRING_MAX_SIZE + 32, 
            data:  metadata_component
        });

        let mapmeta_component = ComponentMapMeta {
            max_x,
            max_y,
            play_phase: false
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.mapmeta.key(), 
            world: ctx.accounts.world_instance.key(), 
            max_size: 1 + 1 + 1, 
            data: mapmeta_component 
        });

        // Mint Map Entity
        dominariworld::cpi::mint_entity(mint_entity_ctx, entity_id, components)?;

        ctx.accounts.instance_index.map = ctx.accounts.map_entity.key();
        Ok(())
    }

    /*
     * Security Concern: There's no way to check if the Tile already exists
     * So it's up the caller of the program to make sure duplicates don't get created
     
    pub fn system_init_tile(ctx:Context<SystemInitTile>, entity_id:u64, x:u8, y:u8) -> Result<()> {
        let system_signer_seeds:&[&[u8]] = &[
            b"System_Signer",
            &[*ctx.bumps.get("system_signer").unwrap()]
        ];
        let signer_seeds = &[system_signer_seeds];

        // Mint Map Entity
        let mint_ctx = CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            dominariworld::cpi::accounts::MintEntity{
                entity: ctx.accounts.tile_entity.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                world_instance: ctx.accounts.world_instance.to_account_info(),
                world_config: ctx.accounts.world_config.to_account_info(),
                system: ctx.accounts.system_signer.to_account_info(),
                system_registration: ctx.accounts.system_registration.to_account_info(),
                universe: ctx.accounts.universe.to_account_info(),
            },
            signer_seeds
        );
        dominariworld::cpi::mint_entity(mint_ctx, entity_id)?;

        let mut components: Vec<SerializedComponent> = vec![];

        // Tile has Metadata, Location, Feature, Owner, Occupant Components
        let metadata_component = ComponentMetadata {
            name: format!("Tile ({:#}, {:#})", x, y),
            entity_type: "Tile".to_string(),
            world_instance: ctx.accounts.world_instance.key(),
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.metadata.key(), 
            world: ctx.accounts.world_instance.key(), 
            max_size: STRING_MAX_SIZE + STRING_MAX_SIZE + 32, 
            data: metadata_component
        });        
        
        let location_component = ComponentLocation {
            x,
            y
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.location.key(), 
            world: ctx.accounts.world_instance.key(), 
            max_size: STRING_MAX_SIZE + STRING_MAX_SIZE + 32, 
            data: location_component
        });

        let feature_component = ComponentFeature {
            feature_id: Default::default() // Empty pubkey
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.feature.key(), 
            world: ctx.accounts.world_instance.key(), 
            max_size: 32, 
            data: feature_component
        });

        let occupant_component = ComponentOccupant {
            occupant_id: Default::default() //empty pubkey
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.occupant.key(), 
            world: ctx.accounts.world_instance.key(), 
            max_size: 32, 
            data: occupant_component
        });

        let owner_component = ComponentOwner {
            player: Default::default(),
            owner: Default::default() //empty pubkey
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.occupant.key(), 
            world: ctx.accounts.world_instance.key(), 
            max_size: 32 + 32, 
            data: owner_component
        });

        // CPI into the World Program to Request Entity update
        let accounts = dominariworld::cpi::accounts::AddComponents {
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            world_config: ctx.accounts.world_config.to_account_info(),
            entity: ctx.accounts.tile_entity.to_account_info(),
            system: ctx.accounts.system_signer.to_account_info(),
            system_registration: ctx.accounts.system_registration.to_account_info(),
            universe: ctx.accounts.universe.to_account_info()
        };

        dominariworld::cpi::req_add_component(CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(), 
            accounts, 
            signer_seeds
        ), components)?;

        ctx.accounts.instance_index.tiles.push(ctx.accounts.tile_entity.key());
        
        Ok(())
    }
    */
    /*
     * Players don't have blueprints cause they are largely unique
     
    pub fn system_register_player(ctx:Context<SystemRegisterPlayer>, entity_id:u64, player_name:String) -> Result<()> {
        let system_signer_seeds:&[&[u8]] = &[
            b"System_Signer",
            &[*ctx.bumps.get("system_signer").unwrap()]
        ];
        let signer_seeds = &[system_signer_seeds];

        // Mint Map Entity
        let mint_ctx = CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            dominariworld::cpi::accounts::MintEntity{
                entity: ctx.accounts.player_entity.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                world_instance: ctx.accounts.world_instance.to_account_info(),
                world_config: ctx.accounts.world_config.to_account_info(),
                system: ctx.accounts.system_signer.to_account_info(),
                system_registration: ctx.accounts.system_registration.to_account_info(),
                universe: ctx.accounts.universe.to_account_info(),
            },
            signer_seeds
        );


        let mut components: Vec<SerializedComponent> = vec![];

        // Add Player Metadata
        let metadata_component = ComponentMetadata {
            name: player_name,
            entity_type: "Player".to_string(), 
            world_instance: ctx.accounts.world_instance.key()
        }.try_to_vec().unwrap();
        components.push(SerializedComponent {
            component_key: ctx.accounts.system_signer.components.metadata.key(),
            world: ctx.accounts.world_instance.world.key(),
            max_size: STRING_MAX_SIZE + STRING_MAX_SIZE + 32,
            data: metadata_component
        });

        // Add Player Stats
        let player_stats = ComponentPlayerStats {
            score: 0_u64,
            kills: 0_u64,
            cards: vec![ctx.accounts.starting_card_blueprint.key()]
        }.try_to_vec().unwrap();
        components.push(SerializedComponent {
            component_key: ctx.accounts.system_signer.components.player_stats.key(),
            world: ctx.accounts.world_instance.world.key(),
            max_size: 8 + 8 + 32,
            data: player_stats
        });
        // CPI into the World Program to Request Entity update
        let accounts = dominariworld::cpi::accounts::AddComponents {
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            world_config: ctx.accounts.world_config.to_account_info(),
            entity: ctx.accounts.player_entity.to_account_info(),
            system: ctx.accounts.system_signer.to_account_info(),
            system_registration: ctx.accounts.system_registration.to_account_info(),
            universe: ctx.accounts.universe.to_account_info()
        };
        
        dominariworld::cpi::req_add_component(CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(), 
            accounts, 
            signer_seeds
        ), components)?;

        ctx.accounts.instance_index.map = ctx.accounts.player_entity.key();

        Ok(())
    }
    */

}

   
/* ## Systems
-> RegisterBlueprint()
    -> Blueprint is a Collection of Preset Components
-> RegisterPlayer()
-> InitMap()
-> InitTile()
-> BuyTile()
-> BuildFeature()
-> SwitchPhases()
-> MoveUnit()
-> UseFeature() *** (Will be quite a few systems)
-> PlayCard()
-> AttackUnit() */
 

/*
        // Can only be instatiated in an empty Entity
        if player.components.len() > 0 {
            return err!(DominariError::InvalidEntity)
        }

        // Player had Metadata and Stats Components
        let new_components = ctx.accounts.player_blueprint.components.clone();
        
        for mut component in new_components {
            if component.component_key.key() == ctx.accounts.system_signer.components.metadata.key() {
                // Setup Player Metadata
                let mut metadata = ComponentMetadata::try_from_slice(component.data.as_slice()).unwrap();
                metadata.entity_type = "Player".to_string();
                metadata.name = player_name.clone();
                metadata.world_instance = ctx.accounts.world_instance.key();
                component.data = metadata.try_to_vec().unwrap(); //serializes back into Vec<u8>
                player.components.push(component);
            } else if component.component_key.key() == ctx.accounts.system_signer.components.player_stats.key() {
                // Setup Player Stats


            }
        }

*/