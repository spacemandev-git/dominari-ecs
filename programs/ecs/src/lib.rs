use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

mod account;
mod context;
mod constant;
mod error;
mod event;
mod state;

//use account::*;
use context::*;
//use constant::*;
use error::*;
use event::*;
use state::*;

#[program]
pub mod ecs {

    use super::*;

    pub fn register_world(ctx:Context<RegisterWorldInstance>, world:Pubkey, instance: u64) -> Result<()> {
        ctx.accounts.world_instance.world = world;
        ctx.accounts.world_instance.instance = instance;
        
        emit!(NewWorldInitalized {
            world,
            instance,
            instance_address: ctx.accounts.world_instance.key()
        });
        
        Ok(())
    }

    pub fn mint_entity(ctx:Context<MintEntity>) -> Result<()> {
        ctx.accounts.entity.world = ctx.accounts.world_instance.world.key();
        ctx.accounts.entity.instance = ctx.accounts.world_instance.instance;

        emit!(NewEntityMinted{
            world_instance: ctx.accounts.world_instance.key(),
            mint: ctx.accounts.mint.key(),
            entity: ctx.accounts.entity.key()
        });

        Ok(())
    }

    pub fn add_component(ctx:Context<AddComponent>, comp:SerializedComponent) -> Result<()> {
        ctx.accounts.entity.components.push(comp.clone());
        
        emit!(NewComponentAdded{
            entity: ctx.accounts.entity.key(),
            component: comp
        });
        Ok(())
    }

    pub fn remove_component(ctx:Context<RemoveComponent>, idx: usize) -> Result<()> {
        let removed_comp = ctx.accounts.entity.components.remove(idx);
        
        emit!(ComponentRemoved {
            entity: ctx.accounts.entity.key(),
            component: removed_comp
        });

        Ok(())
    }

    pub fn modify_component(ctx:Context<ModifyComponent>, idx: usize, data:Vec<u8>) -> Result<()> {        
        if ctx.accounts.entity.components.get(idx).unwrap().max_size < data.len() {
            return err!(ComponentError::InvalidDataLengthError)
        }

        ctx.accounts.entity.components.get_mut(idx).unwrap().data = data;

        emit!(ComponentModified {
            entity: ctx.accounts.entity.key(),
            component: ctx.accounts.entity.components.get(idx).unwrap().clone()
        });

        Ok(())
    }

}
