use anchor_lang::prelude::*;
use std::collections::BTreeMap;

use ecs::state::SerializedComponent;

declare_id!("H5mieGWWK6qukHoNzbR6ysLxReeQC4JHZcNM6JkPQnm3");

pub mod account;
pub mod context;
pub mod constant;
pub mod error;
pub mod event;
pub mod state;

//use account::*;
use context::*;
//use constant::*;
//use error::*;
use event::*;
//use state::*;

#[program]
pub mod dominariworld {

    use std::collections::BTreeMap;

    use super::*;

    pub fn initalize(ctx:Context<Initialize>, universe: Pubkey) -> Result<()> {
        ctx.accounts.world_config.universe = universe;
        ctx.accounts.world_config.components = 0;
        Ok(())
    }

    /**
     * Instance World should normally be regulated by governance; 
     * In this case, we allow anyone to instance a new dominari world.
     * We also set the Instance Authority for the World to the Payer
     * This authority is the only one that can add systems to a given instance
     */
    pub fn instance_world(ctx:Context<InstanceWorld>, instance:u64) -> Result<()> {
        let universe = ctx.accounts.universe.to_account_info();
        let accounts = ecs::cpi::accounts::RegisterWorldInstance {
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            world_instance: ctx.accounts.world_instance.to_account_info(),
            world_signer: ctx.accounts.world_config.to_account_info()
        };
        let world_signer_seeds:&[&[u8]] = &[
            b"world_signer",
            &[*ctx.bumps.get("world_config").unwrap()]
        ];
        let signer_seeds = &[world_signer_seeds];

        let register_world_ctx = CpiContext::new_with_signer(
            universe,
            accounts,
            signer_seeds
        );

        ecs::cpi::register_world(register_world_ctx, ctx.program_id.key(), instance)?;
        ctx.accounts.instance_authority.instance = instance;
        ctx.accounts.instance_authority.authority = ctx.accounts.payer.key(); // fancier Worlds might have different governance setup for this

        emit!(NewWorldInstance{
            world_instance: ctx.accounts.world_instance.key(),
            instance_authority: ctx.accounts.instance_authority.key()
        });
        
        Ok(())
    }

    /**
     * Anyone can register new components as long as they use unique URIs
     */
    pub fn register_component(ctx:Context<RegisterComponent>, schema:String) -> Result<()> {
        ctx.accounts.component.url = schema.clone();
        ctx.accounts.world_config.components += 1;

        emit!(NewComponentRegistered{
            component: ctx.accounts.component.key(),
            schema: schema
        });
        Ok(())
    }

    pub fn register_system(ctx: Context<RegisterSystem>) -> Result<()> {
        ctx.accounts.system_registration.system = ctx.accounts.system.key();
        ctx.accounts.system_registration.instance = ctx.accounts.world_instance.instance;

        emit!(NewSystemRegistration {
            world_instance: ctx.accounts.world_instance.key(),
            system: ctx.accounts.system.key(),
            system_registration: ctx.accounts.system_registration.key()
        });
        Ok(())
    }

    pub fn add_components_to_system_registration(ctx:Context<AddComponentsToSystemRegistration>, components:Vec<Pubkey>) -> Result<()> {
        for comp in components {
            ctx.accounts.system_registration.components.insert(comp, true);
        }
        Ok(())
    }

    pub fn mint_entity(ctx:Context<MintEntity>, entity_id: u64, components: BTreeMap<Pubkey, SerializedComponent>) -> Result<()> {
        let accounts = ecs::cpi::accounts::MintEntity {
            entity: ctx.accounts.entity.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            world_instance: ctx.accounts.world_instance.to_account_info(),
            world_signer: ctx.accounts.world_config.to_account_info(),
        };  
        let world_signer_seeds:&[&[u8]] = &[
            b"world_signer",
            &[*ctx.bumps.get("world_config").unwrap()]
        ];
        let signer_seeds = &[world_signer_seeds];
        
        ecs::cpi::mint_entity(CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            accounts,
            signer_seeds
        ), entity_id, components)?;
        
        Ok(())
    }

    pub fn req_add_component(ctx:Context<AddComponents>, components: Vec<(Pubkey,SerializedComponent)>) -> Result<()> {
        let accounts = ecs::cpi::accounts::AddComponent {
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            entity: ctx.accounts.entity.to_account_info(),
            world_signer: ctx.accounts.world_config.to_account_info()
        };
        let world_signer_seeds:&[&[u8]] = &[
            b"world_signer",
            &[*ctx.bumps.get("world_config").unwrap()]
        ];
        let signer_seeds = &[world_signer_seeds];
        
        ecs::cpi::add_components(CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            accounts,
            signer_seeds
        ), components)?;

        //No need to emit an event, as Universe will do so
        Ok(())
    }

    pub fn req_remove_component(ctx:Context<RemoveComponent>, components: Vec<Pubkey>) -> Result<()> {
        let accounts = ecs::cpi::accounts::RemoveComponent {
            benefactor: ctx.accounts.benefactor.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            entity: ctx.accounts.entity.to_account_info(),
            world_signer: ctx.accounts.world_config.to_account_info()
        };
        let world_signer_seeds:&[&[u8]] = &[
            b"world_signer",
            &[*ctx.bumps.get("world_config").unwrap()]
        ];
        let signer_seeds = &[world_signer_seeds];
        
        ecs::cpi::remove_component(CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            accounts,
            signer_seeds
        ), components)?;

        //No need to emit an event, as Universe will do so
        Ok(())
    }

    pub fn req_modify_component(ctx:Context<ModifyComponent>, components: Vec<Pubkey>, data:Vec<Vec<u8>>) -> Result<()> {
        let accounts = ecs::cpi::accounts::ModifyComponent {
            entity: ctx.accounts.entity.to_account_info(),
            world_signer: ctx.accounts.world_config.to_account_info()
        };
        let world_signer_seeds:&[&[u8]] = &[
            b"world_signer",
            &[*ctx.bumps.get("world_config").unwrap()]
        ];
        let signer_seeds = &[world_signer_seeds];
        
        ecs::cpi::modify_components(CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            accounts,
            signer_seeds
        ), components, data)?;

        //No need to emit an event, as Universe will do so
        Ok(())
    }

    pub fn req_remove_entity(ctx:Context<RemoveEntity>) -> Result<()> {
        let accounts = ecs::cpi::accounts::RemoveEntity {
            benefactor: ctx.accounts.benefactor.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            entity: ctx.accounts.entity.to_account_info(),
            world_signer: ctx.accounts.world_config.to_account_info()
        };
        let world_signer_seeds:&[&[u8]] = &[
            b"world_signer",
            &[*ctx.bumps.get("world_config").unwrap()]
        ];
        let signer_seeds = &[world_signer_seeds];
        
        ecs::cpi::remove_entity(CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            accounts,
            signer_seeds
        ))?;

        //No need to emit an event, as Universe will do so
        Ok(())
    }

    
}