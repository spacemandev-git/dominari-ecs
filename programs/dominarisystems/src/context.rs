use anchor_lang::prelude::*;

use crate::account::*;
use ecs::{
    state::SerializedComponent, 
    account::{Entity, WorldInstance},
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
        space=8+32+576
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
        seeds=[b"System_Signer"],
        bump,
    )]
    pub system_signer: Account<'info, SystemConfig>,

    #[account(
        mut,
        constraint = player_entity.authority.key() == payer.key()
    )]
    pub player_entity: Account<'info, Entity>,
    #[account(
        seeds=[
            b"Blueprint",
            b"Starting_Card",
        ],
        bump,
    )]
    pub starting_card_blueprint: Account<'info, Blueprint>,

    #[account(
        constraint = world_instance.world.key() == player_entity.world.key() && world_instance.instance == player_entity.instance
    )]
    pub world_instance: Account<'info, WorldInstance>,

    pub system_registration: Account<'info, SystemRegistration>,
    pub world_config: Account<'info, WorldConfig>,
    pub world_program: Program<'info, Dominariworld>,
    pub universe: Program<'info, Ecs>, 
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
    pub system_registration: Account<'info, SystemRegistration>,
    pub world_config: Account<'info, WorldConfig>,
    pub world_program: Program<'info, Dominariworld>,
    pub universe: Program<'info, Ecs>, 

    pub world_instance: Account<'info, WorldInstance>,    

    /// CHECK: Initalized through CPI
    #[account(mut)]
    pub map_entity: AccountInfo<'info>,

    /// CHECK: Initialized through CPI
    #[account(mut)]
    pub map_mint: AccountInfo<'info>,
    
    /// CHECK: Initialized through CPI
    #[account(mut)]
    pub map_mint_ata: AccountInfo<'info>,

    /// CHECK: Entity.rs will Check it
    pub spl_token_program: AccountInfo<'info>
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
    pub system_registration: Account<'info, SystemRegistration>,
    pub world_config: Account<'info, WorldConfig>,
    pub world_program: Program<'info, Dominariworld>,
    pub universe: Program<'info, Ecs>, 
    #[account(
        constraint = world_instance.world.key() == tile_entity.world.key() && world_instance.instance == tile_entity.instance
    )]
    pub world_instance: Account<'info, WorldInstance>,    
    #[account(
        mut,
        constraint = tile_entity.authority.key() == payer.key()
    )]
    pub tile_entity: Account<'info, Entity>,
}

/********************************************UTIL Fns */
pub fn compute_comp_arr_max_size(components: &Vec<SerializedComponent>) -> usize {
    let mut max_size:usize = 0;
    for comp in components {
        max_size += comp.max_size;
    }
    return max_size;
}