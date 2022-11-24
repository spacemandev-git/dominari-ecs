use anchor_lang::prelude::*;

use crate::account::*;
use crate::component::MaxSize;
use crate::constant::*;
use crate::state::RelevantComponentKeys;

use ecs::{
    state::SerializedComponent, 
    account::{WorldInstance, Entity},
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
        space= 8 + 32 + RelevantComponentKeys::get_max_size() as usize,
    )]
    pub system_signer: Account<'info, SystemConfig>
}

#[derive(Accounts)]
#[instruction(name:String, components: Vec<SerializedComponent>)]
pub struct RegisterBlueprint <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        constraint = system_config.authority.key() == payer.key()
    )]
    pub system_config: Account<'info, SystemConfig>,

    #[account(
        init,
        payer=payer,
        seeds=[
            b"Blueprint",
            name.as_bytes().as_ref()
        ],
        bump,
        space=8 + STRING_MAX_SIZE as usize + compute_comp_arr_max_size(&components)
    )]
    pub blueprint: Account<'info, Blueprint>,
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
    pub system_signer: Box<Account<'info, SystemConfig>>,

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
    pub system_signer: Box<Account<'info, SystemConfig>>,

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
    pub tile_entity: AccountInfo<'info>,

    #[account(
        mut,
        realloc = instance_index.to_account_info().data_len() + 32,
        realloc::payer = payer,
        realloc::zero = false,
        seeds=[
            b"Instance_Index",
            world_instance.key().as_ref()
        ],
        bump,
    )]
    pub instance_index: Box<Account<'info, InstanceIndex>>,
}


#[derive(Accounts)]
pub struct SystemInstanceFeature<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        constraint = system_signer.authority.key() == payer.key(), //Only System Auth can make new Features 
        seeds=[b"System_Signer"],
        bump,
    )]
    pub system_signer: Box<Account<'info, SystemConfig>>,

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
    pub feature_entity: AccountInfo<'info>,
    pub blueprint: Box<Account<'info, Blueprint>>,
    
    #[account(mut)]
    pub tile_entity: Box<Account<'info, Entity>>,

    #[account(
        mut,
        realloc = instance_index.to_account_info().data_len() + 32,
        realloc::payer = payer,
        realloc::zero = false,
        seeds=[
            b"Instance_Index",
            world_instance.key().as_ref()
        ],
        bump,
    )]
    pub instance_index: Box<Account<'info, InstanceIndex>>,
}

/********************************************UTIL Fns */
pub fn compute_comp_arr_max_size(components: &Vec<SerializedComponent>) -> usize {
    let mut max_size:usize = 0;
    for comp in components {
        max_size += comp.max_size as usize + 44;
    }
    return max_size;
}