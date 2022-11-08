use anchor_lang::prelude::*;
use ecs::{
    cpi::accounts::RegisterWorldInstance,
    state::SerializedComponent
};

declare_id!("GGNoo8tn1vbLnMwU9Hz4oFmNXQd2gVpXCG9m3ZT7LK1J");

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
    use super::*;

    pub fn initalize(ctx:Context<Initialize>, universe: Pubkey) -> Result<()> {
        ctx.accounts.world_config.universe = universe;
        ctx.accounts.world_config.instances = 0;
        ctx.accounts.world_config.components = 0;
        Ok(())
    }

    /**
     * Instance World should normally be regulated by governance; 
     * In this case, we allow anyone to instance a new dominari world.
     * We also set the Instance Authority for the World to the Payer
     * This authority is the only one that can add systems to a given instance
     */
    pub fn instance_world(ctx:Context<InstanceWorld>) -> Result<()> {
        let universe = ctx.accounts.universe.to_account_info();
        let accounts = RegisterWorldInstance {
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
        
        ecs::cpi::register_world(register_world_ctx, ctx.program_id.clone(), ctx.accounts.world_config.instances+1_u64)?;
        ctx.accounts.world_config.instances += 1; // basically the UUID for the instance
        ctx.accounts.instance_authority.instance = ctx.accounts.world_config.instances;
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

    /**
     * Systems are registered PER COMPONENT PER INSTANCE
     * Only the person that *made* the instance can register systems for that instance.
     * This could be changed in other worlds to be a DAO based vote.
     */
    pub fn register_system_for_component(ctx: Context<RegisterSystem>) -> Result<()> {
        ctx.accounts.system_registration.component = ctx.accounts.component.key();
        ctx.accounts.system_registration.system = ctx.accounts.system.key();

        emit!(NewSystemRegistration {
            world_instance: ctx.accounts.world_instance.key(),
            component: ctx.accounts.component.key(),
            system: ctx.accounts.system.key(),
            system_registration: ctx.accounts.system_registration.key()
        });
        Ok(())
    }

    pub fn add_component(ctx:Context<AddComponent>, components: Vec<SerializedComponent>) -> Result<()> {
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
        
        ecs::cpi::add_component(CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            accounts,
            signer_seeds
        ), components)?;

        //No need to emit an event, as Universe will do so
        Ok(())
    }

    pub fn remove_component(ctx:Context<RemoveComponent>, comp: usize) -> Result<()> {
        let accounts = ecs::cpi::accounts::RemoveComponent {
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
        
        ecs::cpi::remove_component(CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            accounts,
            signer_seeds
        ), comp)?;

        //No need to emit an event, as Universe will do so
        Ok(())
    }

    pub fn modify_component(ctx:Context<ModifyComponent>, comp: usize, data:Vec<u8>) -> Result<()> {
        let accounts = ecs::cpi::accounts::ModifyComponent {
            entity: ctx.accounts.entity.to_account_info(),
            world_signer: ctx.accounts.world_config.to_account_info()
        };
        let world_signer_seeds:&[&[u8]] = &[
            b"world_signer",
            &[*ctx.bumps.get("world_config").unwrap()]
        ];
        let signer_seeds = &[world_signer_seeds];
        
        ecs::cpi::modify_component(CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            accounts,
            signer_seeds
        ), comp, data)?;

        //No need to emit an event, as Universe will do so
        Ok(())
    }
    
}