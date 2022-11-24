use anchor_lang::prelude::*;

use crate::account::*;
use crate::state::*;
use crate::constant::*;

#[derive(Accounts)]
#[instruction(world:Pubkey, instance:u64)]
pub struct RegisterWorldInstance <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer=payer,
        space=8+32+8+8,
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
#[instruction(entity_id:u64, components: Vec<SerializedComponent>)]
pub struct MintEntity<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    pub world_instance: Account<'info, WorldInstance>,

    #[account(
        init,
        payer=payer,
        space=8+8+8+32+32+4+compute_comp_arr_max_size(&components), //It is expected this will get Realloc'd every time a component is added
        seeds = [
            b"Entity",
            entity_id.to_be_bytes().as_ref(),
            world_instance.key().as_ref()
        ],
        bump,
    )]
    pub entity: Box<Account<'info, Entity>>,

    // Only the Entity's World can make changes to the Entity
    #[account(
        seeds = [
            b"world_signer",
        ],
        bump,
        seeds::program = world_instance.world.key()
    )]
    pub world_signer: Signer<'info>
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
        realloc::zero = true,
        constraint = entity.world_signer.key() == world_signer.key()
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
    pub benefactor: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        realloc = entity.to_account_info().data_len() - get_removed_size(&entity.components, &removed_components),
        realloc::payer = benefactor,
        realloc::zero = false,
        constraint = entity.world_signer.key() == world_signer.key()
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
    #[account(
        mut,
        constraint = entity.world_signer.key() == world_signer.key()
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
pub struct RemoveEntity<'info>{
    #[account(mut)]
    pub benefactor: Signer<'info>,
    pub system_program: Program<'info, System>,
    
    #[account(
        mut,
        constraint = entity.world_signer.key() == world_signer.key() && entity.components.len() == 0, // Can only delete empty Entities
        close = benefactor
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

/************************************************ Utility Functions */
pub fn compute_comp_arr_max_size(components: &Vec<SerializedComponent>) -> usize {
    let mut max_size:usize = 0;
    for comp in components {
        max_size += comp.max_size as usize + SERIALIZED_COMPONENT_EXTRA_SPACE as usize; //44 is for the pubkey (32) and max_size (8) value and empty vec (4) in serialized comp itself
    }
    return max_size;
}

pub fn get_removed_size(components: &Vec<SerializedComponent>, removed_components: &Vec<Pubkey>) -> usize {
    let mut removed_size:usize = 0;
    for comp in components.iter() {
        if removed_components.contains(&comp.component_key) {
            removed_size += comp.max_size as usize + SERIALIZED_COMPONENT_EXTRA_SPACE as usize; //44 is for the pubkey (32) and max_size (8) value and empty vec (4) in serialized comp itself
        }
    }
    return removed_size;
}
