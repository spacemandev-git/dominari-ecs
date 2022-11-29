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
use error::*;
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

    pub fn register_blueprint(ctx:Context<RegisterBlueprint>, name:String, components: Vec<SerializedComponent>) -> Result<()> {
        ctx.accounts.blueprint.name = name;
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
            ctx.accounts.world_program.to_account_info(),
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
            max_size: ComponentMetadata::get_max_size(), 
            data:  metadata_component
        });

        let mapmeta_component = ComponentMapMeta {
            max_x,
            max_y,
            play_phase: PlayPhase::Lobby
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.mapmeta.key(), 
            max_size: ComponentMapMeta::get_max_size(), 
            data: mapmeta_component 
        });

        // Mint Map Entity
        dominariworld::cpi::mint_entity(mint_entity_ctx, entity_id, components)?;
        ctx.accounts.instance_index.map = entity_id; //ctx.accounts.map_entity.key();
        Ok(())
    }

    pub fn system_init_tile(ctx:Context<SystemInitTile>, entity_id:u64, x:u8, y:u8, cost:u64) -> Result<()> {
        // Tile can only be instanced by Admin
        // So we can trust in the input

        // Tile has Metadata, Location, Feature, Owner and Cost components
        let mut components: Vec<SerializedComponent> = vec![];
        let metadata = ComponentMetadata {
            name: format!("Tile ({x}, {y})"),
            entity_type: "Tile".to_string(),
            world_instance: ctx.accounts.world_instance.key(),
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.metadata.key(),
            max_size: ComponentMetadata::get_max_size(),
            data: metadata
        });

        let location = ComponentLocation {
            x,
            y,
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.location.key(),
            max_size: ComponentLocation::get_max_size(),
            data: location
        });

        let feature = ComponentFeature {
            feature_id: None,
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.feature.key(),
            max_size: ComponentFeature::get_max_size(),
            data: feature
        });

        let owner = ComponentOwner {
            owner: Some(ctx.accounts.payer.key()),
            player: None,
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.owner.key(),
            max_size: ComponentOwner::get_max_size(),
            data: owner
        });

        let cost_component = ComponentCost {
            lamports: cost,
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.cost.key(),
            max_size: ComponentCost::get_max_size(),
            data: cost_component
        });

        let system_signer_seeds:&[&[u8]] = &[
            b"System_Signer",
            &[*ctx.bumps.get("system_signer").unwrap()]
        ];
        let signer_seeds = &[system_signer_seeds];

        let mint_entity_ctx = CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(),
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

        dominariworld::cpi::mint_entity(mint_entity_ctx, entity_id, components)?;
        ctx.accounts.instance_index.tiles.push(entity_id);
        Ok(())
    }
    
    pub fn system_instance_feature(ctx:Context<SystemInstanceFeature>, entity_id: u64) -> Result<()> {
        // Check to make sure tile can be modified by payer
        let tile_owner_component = ctx.accounts.tile_entity.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.owner.key()).unwrap();
        let tile_owner:ComponentOwner = ComponentOwner::try_from_slice(&tile_owner_component.data.as_slice()).unwrap();
        if tile_owner.owner.unwrap().key() != ctx.accounts.payer.key() {
            return err!(ComponentErrors::InvalidOwner)
        }

        // TODO: Check Blueprint 'cost' component and transfer that fee to build the Feature

        // Create Feature entity
        let mut components = vec![];
        // Feature has Metadata, Location, Owner, Active, and ..Blueprint Components
        let metadata_component = ComponentMetadata {
            name: ctx.accounts.blueprint.name.clone(),
            entity_type: "Feature".to_string(),
            world_instance: ctx.accounts.world_instance.key(),
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.metadata.key(), 
            max_size: ComponentMetadata::get_max_size(), 
            data:  metadata_component
        });
        // Just copy the Tile Location component
        let tile_location = ctx.accounts.tile_entity.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.location.key()).unwrap();
        components.push(tile_location.clone());
        
        let owner = ComponentOwner {
            owner: tile_owner.owner,
            player: None,
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.owner.key(),
            max_size: ComponentOwner::get_max_size(),
            data: owner
        });

        let active = ComponentActive {
            active: true
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.active.key(),
            max_size: ComponentActive::get_max_size(),
            data: active
        });

        components.extend(ctx.accounts.blueprint.components.clone());

        //msg!("System Registration Components: {:?}", ctx.accounts.system_registration.components);
        //msg!("Feature Components: {:?}", components);


        let system_signer_seeds:&[&[u8]] = &[
            b"System_Signer",
            &[*ctx.bumps.get("system_signer").unwrap()]
        ];
        let signer_seeds = &[system_signer_seeds];

        let mint_entity_ctx = CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(),
            dominariworld::cpi::accounts::MintEntity{
                entity: ctx.accounts.feature_entity.to_account_info(),
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

        dominariworld::cpi::mint_entity(mint_entity_ctx, entity_id, components)?;
        ctx.accounts.instance_index.features.push(entity_id);

        // Modify the Tile Entity with the new Feature
        let tile_feature_component = ctx.accounts.tile_entity.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.feature.key()).unwrap();
        let mut tile_feature:ComponentFeature = ComponentFeature::try_from_slice(&tile_feature_component.data.as_slice()).unwrap();
        tile_feature.feature_id = Some(ctx.accounts.feature_entity.key());
        let data = tile_feature.try_to_vec().unwrap();

        //msg!("{}", ctx.accounts.system_signer.components.feature.key());

        let modify_tile_ctx = CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(),
            dominariworld::cpi::accounts::ModifyComponent {
                world_config: ctx.accounts.world_config.to_account_info(),
                entity: ctx.accounts.tile_entity.to_account_info(),
                system: ctx.accounts.system_signer.to_account_info(),
                system_registration: ctx.accounts.system_registration.to_account_info(),
                universe: ctx.accounts.universe.to_account_info(),
            },
            signer_seeds
        );
        dominariworld::cpi::req_modify_component(modify_tile_ctx, vec![ctx.accounts.system_signer.components.feature.key()], vec![data])?;
        Ok(())
    }

    pub fn create_game_instance(ctx:Context<CreateGameInstance>, instance:u64, config: GameConfig) -> Result<()> {
        // Instance the World
        let instance_ctx = CpiContext::new(
            ctx.accounts.world_program.to_account_info(),
            dominariworld::cpi::accounts::InstanceWorld {
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                world_config: ctx.accounts.world_config.to_account_info(),
                world_instance: ctx.accounts.world_instance.to_account_info(),
                universe: ctx.accounts.universe.to_account_info(),
                instance_authority: ctx.accounts.instance_authority.to_account_info()
            }
        );

        dominariworld::cpi::instance_world(instance_ctx, instance)?;
        // Set up Instance Index
        ctx.accounts.instance_index.config = config; 
        Ok(())
    }

     
    pub fn system_init_player(ctx:Context<SystemInitPlayer>, entity_id: u64, name:String, image: String ) -> Result <()> {
        // Optional: Fail if too many players already in the instance
        if ctx.accounts.instance_index.config.max_players == ctx.accounts.instance_index.players.len() as u16 {
            return err!(DominariError::PlayerCountExceeded)
        }

        if name.len() > STRING_MAX_SIZE as usize || image.len() > STRING_MAX_SIZE as usize {
            return err!(ComponentErrors::StringTooLong)
        }

        // Create Player Entity
        // Player has: Metadata and Player Stats
        let mut components = vec![];
        // Feature has Metadata, Location, Owner, Active, and ..Blueprint Components
        let metadata_component = ComponentMetadata {
            name: ctx.accounts.payer.key().to_string(),
            entity_type: "Player".to_string(),
            world_instance: ctx.accounts.world_instance.key(),
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.metadata.key(), 
            max_size: ComponentMetadata::get_max_size(), 
            data:  metadata_component
        });

        let player_stats_component = ComponentPlayerStats {
            name,
            image, 
            key: ctx.accounts.payer.key(),
            score: 0,
            kills: 0,
            // Give them Starting Card
            cards: ctx.accounts.instance_index.config.starting_cards.clone()
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.player_stats.key(), 
            max_size: ComponentPlayerStats::get_max_size(), 
            data:  player_stats_component
        });

        let system_signer_seeds:&[&[u8]] = &[
            b"System_Signer",
            &[*ctx.bumps.get("system_signer").unwrap()]
        ];
        let signer_seeds = &[system_signer_seeds];

        let mint_entity_ctx = CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(),
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

        dominariworld::cpi::mint_entity(mint_entity_ctx, entity_id, components)?;
        
        // Add player entity to instance index
        ctx.accounts.instance_index.players.push(entity_id);

        Ok(())
    }

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