use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::*;


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
use event::*;
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
            entity_type: EntityType::Map,
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

        // Tile has Metadata, Location, Feature, Occupant, Owner and Cost components
        let mut components: Vec<SerializedComponent> = vec![];
        let metadata = ComponentMetadata {
            name: format!("Tile ({x}, {y})"),
            entity_type: EntityType::Tile,
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

        let occupant = ComponentOccupant {
            occupant_id: None,
        }.try_to_vec().unwrap();
        components.push(SerializedComponent { 
            component_key: ctx.accounts.system_signer.components.occupant.key(),
            max_size: ComponentOccupant::get_max_size(),
            data: occupant
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
            entity_type: EntityType::Feature,
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
        tile_feature.feature_id = Some(entity_id);
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
            entity_type: EntityType::Player,
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

    /**
     * Can only be called by a player that's in the game
     * Starts the game if players.len() == max_players by setting Map.play_phase to Play
     */
    pub fn change_game_state(ctx:Context<ChangeGameState>, game_state: PlayPhase) -> Result<()> {
        if game_state != PlayPhase::Paused || game_state != PlayPhase::Play {
            return err!(DominariError::InvalidPlayPhase)
        }

        if !ctx.accounts.instance_index.players.contains(&ctx.accounts.player.entity_id) {
            return err!(DominariError::InvalidPlayer)
        }

        let tile_feature_component = ctx.accounts.map.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.mapmeta.key()).unwrap();
        let mut mapmeta = ComponentMapMeta::try_from_slice(&tile_feature_component.data.as_slice()).unwrap();
        mapmeta.play_phase = game_state;
        let data = mapmeta.try_to_vec().unwrap();

        let system_signer_seeds:&[&[u8]] = &[
            b"System_Signer",
            &[*ctx.bumps.get("system_signer").unwrap()]
        ];
        let signer_seeds = &[system_signer_seeds];
        let modify_map_ctx = CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(),
            dominariworld::cpi::accounts::ModifyComponent {
                world_config: ctx.accounts.world_config.to_account_info(),
                entity: ctx.accounts.map.to_account_info(),
                system: ctx.accounts.system_signer.to_account_info(),
                system_registration: ctx.accounts.system_registration.to_account_info(),
                universe: ctx.accounts.universe.to_account_info(),
            },
            signer_seeds
        );
        dominariworld::cpi::req_modify_component(modify_map_ctx, vec![ctx.accounts.system_signer.components.mapmeta.key()], vec![data])?;

        Ok(())
    }

    pub fn spawn_unit(ctx:Context<SpawnUnit>, unit_id: u64) -> Result<()> {

        // Check player belongs to payer
        let player_stats_component = ctx.accounts.player.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.player_stats.key()).unwrap();
        let mut player_stats = ComponentPlayerStats::try_from_slice(&player_stats_component.data.as_slice()).unwrap();
        if player_stats.key.key() != ctx.accounts.payer.key() {
            return err!(ComponentErrors::InvalidOwner)
        }

        // Check that the Tile is Empty
        let tile_occupant_component = ctx.accounts.tile.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.occupant.key()).unwrap();
        let mut tile_occupant = ComponentOccupant::try_from_slice(&tile_occupant_component.data.as_slice()).unwrap();
        if tile_occupant.occupant_id.is_some() {
            return err!(ComponentErrors::TileOccupied)
        }

        // Check the Blueprint is in Player Hand
        let card_idx = player_stats.cards.iter().position(|&card| card.key() == ctx.accounts.unit_blueprint.key());

        if card_idx.is_none() {
            return err!(ComponentErrors::InvalidCard)
        }

        // Modify Player Hand to remove Blueprint
        player_stats.cards.swap_remove(card_idx.unwrap());

        // Create Unit Entity
        let mut components: Vec<SerializedComponent> = vec![];
        // Add Metadata, Owner, Location, Active + Blueprint components
        let metadata_component = ComponentMetadata {
            name: ctx.accounts.unit_blueprint.name.clone(),
            entity_type: EntityType::Unit,
            world_instance: ctx.accounts.world_instance.key()

        }.try_to_vec().unwrap();
        components.push(SerializedComponent {
            component_key: ctx.accounts.system_signer.components.metadata.key(),
            max_size: ComponentMetadata::get_max_size(),
            data: metadata_component
        });
        let owner_component = ComponentOwner {  
            owner: Some(ctx.accounts.payer.key()),
            player: Some(ctx.accounts.player.entity_id)
        }.try_to_vec().unwrap();
        components.push(SerializedComponent {
            component_key: ctx.accounts.system_signer.components.owner.key(),
            max_size: ComponentOwner::get_max_size(),
            data: owner_component
        });
        let active_component = ComponentActive {
            active: true
        }.try_to_vec().unwrap();
        components.push(SerializedComponent{
            component_key: ctx.accounts.system_signer.components.active.key(),
            max_size: ComponentActive::get_max_size(),
            data: active_component
        });

        // Clone the Tile's location component to the Unit
        components.push(
            ctx.accounts.tile.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.location.key()).unwrap().clone()
        );
        
        components.extend(ctx.accounts.unit_blueprint.components.clone());
        
        let system_signer_seeds:&[&[u8]] = &[
            b"System_Signer",
            &[*ctx.bumps.get("system_signer").unwrap()]
        ];
        let signer_seeds = &[system_signer_seeds];

        let mint_entity_ctx = CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(),
            dominariworld::cpi::accounts::MintEntity{
                entity: ctx.accounts.unit.to_account_info(),
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

        dominariworld::cpi::mint_entity(mint_entity_ctx, unit_id, components)?;
        // Add the new Unit Entity to Instance index
        ctx.accounts.instance_index.units.push(unit_id);

        // Modify Tile to point to Unit Entity
        tile_occupant.occupant_id = Some(unit_id);
        let data = tile_occupant.try_to_vec().unwrap();
        let modify_tile_ctx = CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(),
            dominariworld::cpi::accounts::ModifyComponent {
                world_config: ctx.accounts.world_config.to_account_info(),
                entity: ctx.accounts.tile.to_account_info(),
                system: ctx.accounts.system_signer.to_account_info(),
                system_registration: ctx.accounts.system_registration.to_account_info(),
                universe: ctx.accounts.universe.to_account_info(),
            },
            signer_seeds
        );
        dominariworld::cpi::req_modify_component(modify_tile_ctx, vec![ctx.accounts.system_signer.components.feature.key()], vec![data])?;

        emit!(NewUnitSpawned {
            instance: ctx.accounts.world_instance.instance,
            tile: ctx.accounts.tile.entity_id,
            player: ctx.accounts.player.entity_id,
            unit: unit_id
        });

        Ok(())
    }


    pub fn move_unit(ctx:Context<MoveUnit>) -> Result<()> {
        // From.Occupant must be Unit
        let from_occupant_component = ctx.accounts.from.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.occupant.key()).unwrap();
        let mut from_occupant = ComponentOccupant::try_from_slice(&from_occupant_component.data.as_slice()).unwrap();
        if from_occupant.occupant_id.unwrap() != ctx.accounts.unit.entity_id {
            return err!(ComponentErrors::InvalidUnit)
        }
        
        // Unit must be active
        let active_component = ctx.accounts.unit.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.active.key()).unwrap();
        let active = ComponentActive::try_from_slice(&active_component.data.as_slice()).unwrap();
        if active.active == false {
            return err!(ComponentErrors::UnitDead)
        }

        // To.Occupant must be Empty
        let to_occupant_component = ctx.accounts.to.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.occupant.key()).unwrap();
        let mut to_occupant = ComponentOccupant::try_from_slice(&to_occupant_component.data.as_slice()).unwrap();
        if to_occupant.occupant_id.is_some() {
            return err!(ComponentErrors::TileOccupied)
        }

        // Unit must be Owned by Player        
        let unit_owner_component = ctx.accounts.unit.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.owner.key()).unwrap();
        let unit_owner = ComponentOwner::try_from_slice(&unit_owner_component.data.as_slice()).unwrap();
        if unit_owner.owner.unwrap() != ctx.accounts.payer.key() {
            return err!(ComponentErrors::InvalidOwner)
        }
        
        // Unit must be recovered from last used
        let clock = Clock::get().unwrap();
        let unit_last_used_component = ctx.accounts.unit.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.last_used.key()).unwrap();
        let mut unit_last_used = ComponentLastUsed::try_from_slice(&unit_last_used_component.data.as_slice()).unwrap();
        if unit_last_used.last_used != 0 && (unit_last_used.last_used + unit_last_used.recovery) <= clock.slot {
            return err!(ComponentErrors::UnitRecovering)
        }

        // Distance between From and To must be < Unit's Movement
        let from_location_c = ctx.accounts.from.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.location.key()).unwrap();
        let from_location = ComponentLocation::try_from_slice(&from_location_c.data.as_slice()).unwrap();

        let to_location_c = ctx.accounts.to.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.location.key()).unwrap();
        let to_location = ComponentLocation::try_from_slice(&to_location_c.data.as_slice()).unwrap();
        
        let distance:f64 = (((to_location.x - from_location.x).pow(2) + (to_location.y - from_location.y).pow(2)) as f64).sqrt();
        let unit_range_component = ctx.accounts.unit.components.iter().find(|&comp| comp.component_key == ctx.accounts.system_signer.components.range.key()).unwrap();
        let unit_range = ComponentRange::try_from_slice(&unit_range_component.data.as_slice()).unwrap();
        if unit_range.movement < distance as u64 {
            return err!(ComponentErrors::UnitLacksMovement)
        }

        let system_signer_seeds:&[&[u8]] = &[
            b"System_Signer",
            &[*ctx.bumps.get("system_signer").unwrap()]
        ];
        let signer_seeds = &[system_signer_seeds];

        // Modify Unit's last_used & location
        unit_last_used.last_used = clock.slot;

        let modify_unit_ctx = CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(),
            dominariworld::cpi::accounts::ModifyComponent {
                world_config: ctx.accounts.world_config.to_account_info(),
                entity: ctx.accounts.unit.to_account_info(),
                system: ctx.accounts.system_signer.to_account_info(),
                system_registration: ctx.accounts.system_registration.to_account_info(),
                universe: ctx.accounts.universe.to_account_info(),
            },
            signer_seeds
        );
        dominariworld::cpi::req_modify_component(modify_unit_ctx, vec![
                ctx.accounts.system_signer.components.last_used.key(),
                ctx.accounts.system_signer.components.location.key(),    
            ],
            vec![
                unit_last_used.try_to_vec().unwrap(),
                to_location_c.data.clone()
            ])?;

        // Modify From Occupant to be None
        from_occupant.occupant_id = None;
        let modify_from_ctx = CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(),
            dominariworld::cpi::accounts::ModifyComponent {
                world_config: ctx.accounts.world_config.to_account_info(),
                entity: ctx.accounts.from.to_account_info(),
                system: ctx.accounts.system_signer.to_account_info(),
                system_registration: ctx.accounts.system_registration.to_account_info(),
                universe: ctx.accounts.universe.to_account_info(),
            },
            signer_seeds
        );
        dominariworld::cpi::req_modify_component(modify_from_ctx, vec![ctx.accounts.system_signer.components.occupant.key()], vec![from_occupant.try_to_vec().unwrap()])?;    

        // Modify To Occupant to be Unit
        to_occupant.occupant_id = Some(ctx.accounts.unit.entity_id);
        let modify_to_ctx = CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(),
            dominariworld::cpi::accounts::ModifyComponent {
                world_config: ctx.accounts.world_config.to_account_info(),
                entity: ctx.accounts.to.to_account_info(),
                system: ctx.accounts.system_signer.to_account_info(),
                system_registration: ctx.accounts.system_registration.to_account_info(),
                universe: ctx.accounts.universe.to_account_info(),
            },
            signer_seeds
        );
        dominariworld::cpi::req_modify_component(modify_to_ctx, vec![ctx.accounts.system_signer.components.occupant.key()], vec![to_occupant.try_to_vec().unwrap()])?;
        // Emit Troop Movement
        emit!(TroopMovement {
            instance: ctx.accounts.world_instance.instance,
            from: ctx.accounts.from.entity_id,
            to: ctx.accounts.to.entity_id,
            unit: ctx.accounts.unit.entity_id
        });

        Ok(())
    }
    
    pub fn attack_tile(ctx:Context<AttackTile>) -> Result<()> {
        // Attacker could be Feature or Unit (just needs Damage Component)
        let attacker = &ctx.accounts.attacker;
        let defender = &ctx.accounts.defender;
        let reference = &ctx.accounts.system_signer.components;

        // Check that attacker is owned by Payer
        let attacker_owner_c = attacker.components.iter().find(|&comp| comp.component_key == reference.owner.key()).unwrap();
        let attacker_owner = ComponentOwner::try_from_slice(&attacker_owner_c.data.as_slice()).unwrap();
        if attacker_owner.owner != Some(ctx.accounts.payer.key()) {
            return err!(ComponentErrors::InvalidOwner)
        }
        
        // Check that attacker is active
        let attacker_active_c = attacker.components.iter().find(|&comp| comp.component_key == reference.active.key()).unwrap();
        let attacker_active = ComponentActive::try_from_slice(&attacker_active_c.data.as_slice()).unwrap();
        if attacker_active.active == false {
            return err!(ComponentErrors::UnitDead)
        }

        // Check that defender is NOT owned by Payer
        let defender_owner_c = defender.components.iter().find(|&comp| comp.component_key == reference.owner.key()).unwrap();
        let defender_owner = ComponentOwner::try_from_slice(&defender_owner_c.data.as_slice()).unwrap();
        if defender_owner.owner == Some(ctx.accounts.payer.key()) {
            return err!(ComponentErrors::FriendlyFire)
        }

        // Check attacker has damage component
        let attacker_damage_c = attacker.components.iter().find(|&comp| comp.component_key == reference.damage.key()).unwrap();
        let attacker_damage = ComponentDamage::try_from_slice(&attacker_damage_c.data.as_slice()).unwrap();

        // Check defender is active and has health component
        let defender_active_c = defender.components.iter().find(|&comp| comp.component_key == reference.active.key()).unwrap();
        let mut defender_active = ComponentActive::try_from_slice(&defender_active_c.data.as_slice()).unwrap();
        if defender_active.active == false {
            return err!(ComponentErrors::UnitDead)
        }
        let defender_health_c = defender.components.iter().find(|&comp| comp.component_key == reference.health.key()).unwrap();
        let mut defender_health = ComponentHealth::try_from_slice(&defender_health_c.data.as_slice()).unwrap();

        // Defender must be in Range of Attacker
        let attacker_location_c = attacker.components.iter().find(|&comp| comp.component_key == reference.location.key()).unwrap();
        let attacker_location = ComponentLocation::try_from_slice(&attacker_location_c.data.as_slice()).unwrap();
        let defender_location_c = defender.components.iter().find(|&comp| comp.component_key == reference.location.key()).unwrap();
        let defender_location = ComponentLocation::try_from_slice(&defender_location_c.data.as_slice()).unwrap();
        
        let distance:f64 = (((defender_location.x - attacker_location.x).pow(2) + (defender_location.y - attacker_location.y).pow(2)) as f64).sqrt();
        let attacker_range_c = attacker.components.iter().find(|&comp| comp.component_key == reference.range.key()).unwrap();
        let attacker_range = ComponentRange::try_from_slice(&attacker_range_c.data.as_slice()).unwrap();
        if distance as u64 > attacker_range.attack_range {
            return err!(ComponentErrors::OutOfRange)
        }

        // Check attacker last used isn't violated
        let clock = Clock::get().unwrap();
        let attacker_last_used_c = attacker.components.iter().find(|&comp| comp.component_key == reference.last_used.key()).unwrap();
        let mut attacker_last_used = ComponentLastUsed::try_from_slice(&attacker_last_used_c.data.as_slice()).unwrap();
        if attacker_last_used.last_used != 0 && (attacker_last_used.last_used + attacker_last_used.recovery) <= clock.slot {
            return err!(ComponentErrors::UnitRecovering)
        }
        attacker_last_used.last_used = clock.slot;        
        
        let system_signer_seeds:&[&[u8]] = &[
            b"System_Signer",
            &[*ctx.bumps.get("system_signer").unwrap()]
        ];
        let signer_seeds = &[system_signer_seeds];

        // Modify attacker last used
        let modify_attacker_ctx = CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(),
            dominariworld::cpi::accounts::ModifyComponent {
                world_config: ctx.accounts.world_config.to_account_info(),
                entity: ctx.accounts.attacker.to_account_info(),
                system: ctx.accounts.system_signer.to_account_info(),
                system_registration: ctx.accounts.system_registration.to_account_info(),
                universe: ctx.accounts.universe.to_account_info(),
            },
            signer_seeds
        );
        dominariworld::cpi::req_modify_component(modify_attacker_ctx, vec![
                ctx.accounts.system_signer.components.last_used.key(),
            ],
            vec![
                attacker_last_used.try_to_vec().unwrap(),
            ])?;

        // Roll Damage for Attacker, apply modifiers 
        let mut dmg = get_random_u64(attacker_damage.max_damage); 
        
        // check if defender is Feature, if not, look for it's TroopClass
        let defender_metadata_c = defender.components.iter().find(|&comp| comp.component_key == reference.metadata.key() ).unwrap();
        let defender_metadata = ComponentMetadata::try_from_slice(&defender_metadata_c.data.as_slice()).unwrap();

        if defender_metadata.entity_type == EntityType::Feature {
            dmg += attacker_damage.bonus_feature as u64;
        } else {
            let defender_troop_class_c = defender.components.iter().find(|&comp| comp.component_key == reference.troop_class.key()).unwrap();
            let defender_troop_class = ComponentTroopClass::try_from_slice(&defender_troop_class_c.data.as_slice()).unwrap();
            match defender_troop_class.class {
                TroopClass::Aircraft => dmg += attacker_damage.bonus_aircraft as u64,
                TroopClass::Infantry => dmg += attacker_damage.bonus_infantry as u64,
                TroopClass::Armor => dmg += attacker_damage.bonus_armor as u64,
            }
        }

        if dmg < attacker_damage.min_damage {
            dmg = attacker_damage.min_damage;
        }

        if dmg > defender_health.health {
            defender_health.health = 0;
            defender_active.active = false;
        } else {
            defender_health.health -= dmg;
        }

        // Modify defender health
            // If defender health at 0, Modify active as well
        let modify_defender_ctx = CpiContext::new_with_signer(
            ctx.accounts.world_program.to_account_info(),
            dominariworld::cpi::accounts::ModifyComponent {
                world_config: ctx.accounts.world_config.to_account_info(),
                entity: ctx.accounts.defender.to_account_info(),
                system: ctx.accounts.system_signer.to_account_info(),
                system_registration: ctx.accounts.system_registration.to_account_info(),
                universe: ctx.accounts.universe.to_account_info(),
            },
            signer_seeds
        );
        dominariworld::cpi::req_modify_component(modify_defender_ctx, vec![
                reference.health.key(),
                reference.active.key(),
            ],
            vec![
                defender_health.try_to_vec().unwrap(),
                defender_active.try_to_vec().unwrap()
            ])?;
    

        emit!(TileAttacked{
            instance: ctx.accounts.world_instance.instance,
            attacker: attacker.entity_id,
            defender: defender.entity_id,
            damage: dmg
        });

        Ok(())
    }
    //pub fn modify_unit(ctx:Context<ModUnit>) -> Result<()> {}

    //pub fn build_feature(ctx:Context<BuildFeature>) -> Result<()> {}
    //pub fn use_[feature](ctx:Context<UseFeature>) -> Result<()> {}

    // Pass in multiple entities through remaining accounts; will iterate and remove them if they are marked inactive
    //pub fn reclaim_entity_sol(ctx:Context<ReclaimSol>) -> Result<()> {}

}

pub fn get_random_u64(max: u64) -> u64 {
    let clock = Clock::get().unwrap();
    let slice = &hash(&clock.slot.to_be_bytes()).to_bytes()[0..8];
    let num: u64 = u64::from_be_bytes(slice.try_into().unwrap());
    let target = num/(u64::MAX/max);
    return target;
}