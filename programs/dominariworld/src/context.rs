use anchor_lang::prelude::*;

use crate::account::*;

use ecs::{
    self,
    account::{WorldInstance, Entity},
    ID as UniverseID,
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
        space=8+32,
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
        space=8+512
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
    pub component: Account<'info, ComponentSchema>,

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
            component.key().as_ref(),
            world_instance.key().as_ref(),
            system.key().as_ref()
        ],
        bump,
        space=8+8+32+32
    )]
    pub system_registration: Account<'info, SystemRegistration>,
    pub system: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(comp: SerializedComponent)]
pub struct AddComponent<'info>{
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
        constraint = system_registration.component.key() == comp.component_key.key() && system_registration.system.key() == system.key()
    )]
    pub system_registration: Account<'info, SystemRegistration>,

    pub universe: Program<'info, Ecs>, 
}

#[derive(Accounts)]
#[instruction(comp: usize)]
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
        constraint = system_registration.component.key() == entity.components.get(comp).unwrap().component_key.key() && system_registration.system.key() == system.key()
    )]
    pub system_registration: Account<'info, SystemRegistration>,

    pub universe: Program<'info, Ecs>, 
}

#[derive(Accounts)]
#[instruction(comp: usize, data:Vec<u8>)]
pub struct ModifyComponent<'info>{
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
        constraint = system_registration.component.key() == entity.components.get(comp).unwrap().component_key.key() && system_registration.system.key() == system.key()
    )]
    pub system_registration: Account<'info, SystemRegistration>,

    pub universe: Program<'info, Ecs>, 
}