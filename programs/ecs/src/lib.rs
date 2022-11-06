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
//use error::*;
use event::*;
//use state::*;

#[program]
pub mod ecs {
    use super::*;

    pub fn register_world(ctx:Context<RegisterWorldInstance>, world:Pubkey, instance: u64) -> Result<()> {
        emit!(NewWorldInitalized {
            world,
            instance,
            instance_address: ctx.accounts.world_instance.key()
        });
        Ok(())
    }

    pub fn mint_entity(ctx:Context<MintEntity>) -> Result<()> {
        emit!(NewEntityMinted{
            world_instance: ctx.accounts.world_instance.key(),
            mint: ctx.accounts.mint.key(),
            entity: ctx.accounts.entity.key()
        });
        Ok(())
    }

    // TODO: Add Component

    // TODO: Remove Component

    // TODO: Modify Component
        // ComponentUpdateAuthority Must be a Signer
        
}
