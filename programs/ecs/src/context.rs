use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::account::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(world:Pubkey, instance:u64)]
pub struct RegisterWorldInstance <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer=payer,
        space=8+32+8,
        seeds=[
            b"World",
            world.key().to_bytes().as_ref(),
            instance.to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub world_instance: Account<'info, WorldInstance>
}

#[derive(Accounts)]
pub struct MintEntity<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    pub world_instance: Account<'info, WorldInstance>,
    pub mint: Account<'info,Mint>,
    #[account(
        init,
        payer=payer,
        space=8+32+32+8, //It is expected this will get Realloc'd every time a component is added
        seeds = [
            b"Entity",
            mint.key().as_ref(),
            world_instance.key().as_ref()
        ],
        bump,
    )]
    pub entity: Account<'info, Entity>
}

#[derive(Accounts)]
#[instruction(comp:SerializedComponent)]
pub struct AddComponent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        realloc = entity.to_account_info().data_len() + comp.max_size,
        realloc::payer = payer,
        realloc::zero = false,
    )]
    pub entity: Account<'info, Entity>,

    // Only the Entity's World can make changes to the Entity
    #[account(
        seeds = [
            b"world_signer",
        ],
        bump,
        seeds::program = entity.world.key()
    )]
    pub world_signer: Signer<'info>
}

#[derive(Accounts)]
#[instruction(idx:usize)]
pub struct RemoveComponent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        realloc = entity.to_account_info().data_len() - entity.components.get(idx).unwrap().max_size,
        realloc::payer = payer,
        realloc::zero = false,
    )]
    pub entity: Account<'info, Entity>,

    // Only the Entity's World can make changes to the Entity
    #[account(
        seeds = [
            b"world_signer",
        ],
        bump,
        seeds::program = entity.world.key()
    )]
    pub world_signer: Signer<'info>
}

#[derive(Accounts)]
#[instruction(idx:usize)]
pub struct ModifyComponent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub entity: Account<'info, Entity>,

    // Only the Entity's World can make changes to the Entity
    #[account(
        seeds = [
            b"world_signer",
        ],
        bump,
        seeds::program = entity.world.key()
    )]
    pub world_signer: Signer<'info>
}

