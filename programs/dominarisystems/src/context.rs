use anchor_lang::prelude::*;

use crate::account::*;
use ecs::{
    state::SerializedComponent, 
    account::WorldInstance,
    program::Ecs
};
use dominariworld::{
    program::Dominariworld, account::{WorldConfig, SystemRegistration}
};

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer=payer,
        seeds=[b"System_Signer"],
        bump,
        space=8+32+608
    )]
    pub system_signer: Account<'info, SystemConfig>
}

#[derive(Accounts)]
#[instruction(components: Vec<SerializedComponent>, entity_name: String)]
pub struct RegisterBlueprint <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        constraint = system_signer.authority.key() == payer.key()
    )]
    pub system_signer: Account<'info, SystemConfig>,

    #[account(
        init,
        payer=payer,
        seeds=[
            b"Blueprint",
            entity_name.as_bytes()
        ],
        bump,
        space=8+128+compute_comp_arr_max_size(&components)
    )]
    pub blueprint: Account<'info, Blueprint>,
}

#[derive(Accounts)]
pub struct SystemRegisterPlayer <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        constraint = system_signer.authority.key() == payer.key(), //Only System Auth can make new Maps
        seeds=[b"System_Signer"],
        bump,
    )]
    pub system_signer: Account<'info, SystemConfig>,
    
    pub world_config: Account<'info, WorldConfig>,

    pub world_program: Program<'info, Dominariworld>,
    pub universe: Program<'info, Ecs>, 

    pub system_registration: Account<'info, SystemRegistration>,
    pub world_instance: Account<'info, WorldInstance>,    

    /// CHECK: Initalized through CPI
    #[account(mut)]
    pub player_entity: AccountInfo<'info>,

    #[account(
        seeds=[
            b"Blueprint",
            b"Starting_Card",
        ],
        bump,
    )]
    pub starting_card_blueprint: Account<'info, Blueprint>,

    #[account(
        mut,
        realloc = instance_index.to_account_info().data_len() + 32,
        realloc::payer=payer,
        realloc::zero=false,
        seeds=[
            b"Instance_Index",
            world_instance.key().as_ref()
        ],
        bump
    )]
    pub instance_index: Account<'info, InstanceIndex>,
}

#[derive(Accounts)]
pub struct SystemInitMap<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        constraint = system_signer.authority.key() == payer.key(), //Only System Auth can make new Maps
        seeds=[b"System_Signer"],
        bump,
    )]
    pub system_signer: Account<'info, SystemConfig>,

    /// CHECK: Signing account for DM Worlds
    
    #[account(
        seeds = [
            b"world_signer",
        ],
        bump,
        seeds::program = world_instance.world.key()
    )]
    pub world_config: Account<'info, WorldConfig>,

    pub world_program: Program<'info, Dominariworld>,
    pub universe: Program<'info, Ecs>, 

    pub system_registration: Box<Account<'info, SystemRegistration>>,
    pub world_instance: Account<'info, WorldInstance>,    

    /// CHECK: Initalized through CPI
    #[account(mut)]
    pub map_entity: AccountInfo<'info>,

    #[account(
        init,
        payer=payer,
        seeds=[
            b"Instance_Index",
            world_instance.key().as_ref()
        ],
        bump,
        space=8+32+4+4+4+4
    )]
    pub instance_index: Box<Account<'info, InstanceIndex>>,
}

#[derive(Accounts)]
pub struct SystemInitTile<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        constraint = system_signer.authority.key() == payer.key(), //Only System Auth can make new Maps
        seeds=[b"System_Signer"],
        bump,
    )]
    pub system_signer: Account<'info, SystemConfig>,
    
    pub world_config: Account<'info, WorldConfig>,

    pub world_program: Program<'info, Dominariworld>,
    pub universe: Program<'info, Ecs>, 

    pub system_registration: Account<'info, SystemRegistration>,
    pub world_instance: Account<'info, WorldInstance>,    

    /// CHECK: Initalized through CPI
    #[account(mut)]
    pub tile_entity: AccountInfo<'info>,

    #[account(
        mut,
        realloc = instance_index.to_account_info().data_len() + 32,
        realloc::payer=payer,
        realloc::zero=false,
        seeds=[
            b"Instance_Index",
            world_instance.key().as_ref()
        ],
        bump
    )]
    pub instance_index: Account<'info, InstanceIndex>,
}

/********************************************UTIL Fns */
pub fn compute_comp_arr_max_size(components: &Vec<SerializedComponent>) -> usize {
    let mut max_size:usize = 0;
    for comp in components {
        max_size += comp.max_size as usize;
    }
    return max_size;
}