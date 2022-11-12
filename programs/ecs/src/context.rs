use anchor_lang::prelude::*;
use anchor_spl::token::ID as SPLID;

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
    pub world_instance: Account<'info, WorldInstance>,

    // Only the world can register new instances of itself. It's left to the world how to implement this.
    #[account(
        seeds = [
            b"world_signer",
        ],
        bump,
        seeds::program = world.key()
    )]
    pub world_signer: Signer<'info>
}

#[derive(Accounts)]
pub struct MintEntity<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    pub world_instance: Account<'info, WorldInstance>,
    /// CHECK: Passed in as AI because it'll be initalized through CPI
    pub mint: AccountInfo<'info>,
    #[account(
        init,
        payer=payer,
        space=8+32+32+32+8, //It is expected this will get Realloc'd every time a component is added
        seeds = [
            b"Entity",
            mint.key().as_ref(),
            world_instance.key().as_ref()
        ],
        bump,
    )]
    pub entity: Account<'info, Entity>,
    /// CHECK: This account will be marked owner & mint authority on the SPL Token
    pub entity_owner: Signer<'info>,
    /// CHECK: AI because it'll be an ATA that's created via CPI
    #[account(mut)]
    pub mint_ata: AccountInfo<'info>,

    /// CHECK: Checks to make sure it's the right SPL Token Program
    #[account(
        constraint = spl_token_program.key() == SPLID
    )]
    pub spl_token_program: AccountInfo<'info>
}

#[derive(Accounts)]
#[instruction(components:Vec<SerializedComponent>)]
pub struct AddComponent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        realloc = entity.to_account_info().data_len() + compute_comp_arr_max_size(&components),
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
#[instruction(removed_components: Vec<Pubkey>)]
pub struct RemoveComponent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        realloc = entity.to_account_info().data_len() - get_removed_size(&entity.components, &removed_components),
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
#[instruction(components: Vec<Pubkey>, data: Vec<Vec<u8>>)]
pub struct ModifyComponent<'info> {
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

/************************************************ Utility Functions */
pub fn compute_comp_arr_max_size(components: &Vec<SerializedComponent>) -> usize {
    let mut max_size:usize = 0;
    for comp in components {
        max_size += comp.max_size;
    }
    return max_size;
}

pub fn get_removed_size(components: &Vec<SerializedComponent>, removed_components: &Vec<Pubkey>) -> usize {
    let mut removed_size:usize = 0;
    for comp in components.iter() {
        if removed_components.contains(&comp.component_key) {
            removed_size += comp.max_size;
        }
    }
    return removed_size;
}
