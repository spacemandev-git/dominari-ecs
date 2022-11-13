use anchor_lang::prelude::*;

use crate::account::*;
use crate::constant::*;

use ecs::{
    self,
    account::{WorldInstance, Entity},
    program::Ecs,
    state::SerializedComponent
};

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer=payer,
        seeds=[b"world_signer"],
        bump,
        space=8+32+8+8
    )]
    pub world_config: Account<'info, WorldConfig>,
}

#[derive(Accounts)]
pub struct InstanceWorld<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds=[b"world_signer"],
        bump,
    )]
    pub world_config: Account<'info, WorldConfig>,

    /* 
    #[account(
        seeds=[
            b"World",
            program_id.key().as_ref(),
            (world_config.instances+1_u64).to_be_bytes().as_ref()
        ],
        bump,
        seeds::program = UniverseID,
    )]
    pub world_instance: Account<'info, WorldInstance>,
    */
    /// CHECK: Initalized in CPI call
    #[account(mut)]
    pub world_instance: AccountInfo<'info>,
    pub universe: Program<'info, Ecs>,

    // Instance Authority is in charge of allowing new systems onto this instance
    #[account(
        init,
        payer=payer,
        seeds=[
            b"Instance_Authority",
            world_instance.key().as_ref()
        ],
        bump,
        space=8+8+32,
    )]
    pub instance_authority: Account<'info, InstanceAuthority>

}

#[derive(Accounts)]
#[instruction(schema:String)]
pub struct RegisterComponent<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer=payer,
        seeds=[
            schema.as_bytes(),
        ],
        bump,
        space=8+STRING_MAX_SIZE
    )]
    pub component: Account<'info, ComponentSchema>,

    #[account(
        mut,
        seeds=[b"world_signer"],
        bump,
    )]
    pub world_config: Account<'info, WorldConfig>,
}

#[derive(Accounts)]
pub struct RegisterSystem <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    /// Universe World Instance Account
    /// Make sure that its a world instance that belongs to *this* world
    #[account(
        constraint = world_instance.world.key() == program_id.key()
    )]
    pub world_instance: Account<'info, WorldInstance>,

    /// Make sure the instance authority is of the world instance that's passed in
    #[account(
        constraint = instance_authority.instance == world_instance.instance
    )]
    pub instance_authority: Account<'info, InstanceAuthority>,
    
    #[account(
        init,
        payer=payer,
        seeds=[
            b"System_Registration",
            world_instance.key().as_ref(),
            system.key().as_ref()
        ],
        bump,
        space=8+8+32+32
    )]
    pub system_registration: Account<'info, SystemRegistration>,

    /// CHECK: This can be any pubkey, but likely will be pubkey of 
    /// PDA Signer from System
    pub system: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(components: Vec<Pubkey>)]
pub struct AddComponentsToSystemRegistration <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    /// Universe World Instance Account
    /// Make sure that its a world instance that belongs to *this* world
    #[account(
        constraint = world_instance.world.key() == program_id.key()
    )]
    pub world_instance: Account<'info, WorldInstance>,

    /// Make sure the instance authority is of the world instance that's passed in
    #[account(
        constraint = instance_authority.instance == world_instance.instance
    )]
    pub instance_authority: Account<'info, InstanceAuthority>,
    
    #[account(
        mut,
        realloc = system_registration.to_account_info().data_len() + (components.len()*32),
        realloc::payer = payer,
        realloc::zero = false,
        seeds=[
            b"System_Registration",
            world_instance.key().as_ref(),
            system.key().as_ref()
        ],
        bump,
    )]
    pub system_registration: Account<'info, SystemRegistration>,

    /// CHECK: This can be any pubkey, but likely will be pubkey of 
    /// PDA Signer from System
    pub system: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MintEntity<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    //Used to Sign Tx for the CPI
    pub world_config: Account<'info, WorldConfig>,

    /// CHECK: Initalized via CPI
    #[account(mut)]
    pub entity: AccountInfo<'info>,
    
    #[account(
        constraint = world_instance.world.key() == program_id.key() && world_instance.instance == system_registration.instance
    )]
    pub world_instance: Account<'info, WorldInstance>,
    pub system: Signer<'info>,
    // All systems can make any entities they want
    #[account(
        constraint = system_registration.system.key() == system.key()
    )]
    pub system_registration: Account<'info, SystemRegistration>,
    pub universe: Program<'info, Ecs>,     
}

#[derive(Accounts)]
#[instruction(components: Vec<SerializedComponent>)]
pub struct AddComponents<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    //Used to Sign Tx for the CPI
    pub world_config: Account<'info, WorldConfig>,

    #[account(
        mut,
        constraint = entity.world.key() == program_id.key() && entity.instance == system_registration.instance
    )]
    pub entity: Account<'info, Entity>,
    
    pub system: Signer<'info>,
    
    // System is allowed to modify the component it's adding
    // System is a signer
    #[account(
        constraint = check_components_can_be_modified_by_system(&get_pubkeys_from_components(&components), &system_registration.components) && system_registration.system.key() == system.key()
    )]
    pub system_registration: Account<'info, SystemRegistration>,

    pub universe: Program<'info, Ecs>, 
}

#[derive(Accounts)]
#[instruction(components: Vec<Pubkey>)]
pub struct RemoveComponent<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    //Used to Sign Tx for the CPI
    pub world_config: Account<'info, WorldConfig>,

    #[account(
        mut,
        constraint = entity.world.key() == program_id.key() && entity.instance == system_registration.instance
    )]
    pub entity: Account<'info, Entity>,
    
    pub system: Signer<'info>,
    
    // System is allowed to modify the component it's adding
    // System is a signer
    #[account(
        constraint = check_components_can_be_modified_by_system(&components, &system_registration.components) && system_registration.system.key() == system.key()
    )]
    pub system_registration: Account<'info, SystemRegistration>,

    pub universe: Program<'info, Ecs>, 
}

#[derive(Accounts)]
#[instruction(components: Vec<Pubkey>, data:Vec<Vec<u8>>)]
pub struct ModifyComponent<'info>{
    //Used to Sign Tx for the CPI
    pub world_config: Account<'info, WorldConfig>,

    #[account(
        mut,
        constraint = entity.world.key() == program_id.key() && entity.instance == system_registration.instance
    )]
    pub entity: Account<'info, Entity>,
    
    pub system: Signer<'info>,
    
    // System is allowed to modify the component it's adding
    // System is a signer
    #[account(
        constraint = check_components_can_be_modified_by_system(&components, &system_registration.components) && system_registration.system.key() == system.key()
    )]
    pub system_registration: Account<'info, SystemRegistration>,

    pub universe: Program<'info, Ecs>, 
}

/*************************************************UTIL Functions */

pub fn check_components_can_be_modified_by_system(components: &Vec<Pubkey>, system_components: &Vec<Pubkey>) -> bool {
    let mut pubkeys_found:usize = 0;
    for component in system_components.iter() {
        if components.contains(component) {
            pubkeys_found += 1;
        }
    }

    if pubkeys_found == components.len() {
        return true;
    } else {
        return false;
    }
}

pub fn get_pubkeys_from_components(components: &Vec<SerializedComponent>) -> Vec<Pubkey> {
    let mut pubkeys = vec![];
    for comp in components {
        pubkeys.push(comp.component_key.key());
    }
    pubkeys
}