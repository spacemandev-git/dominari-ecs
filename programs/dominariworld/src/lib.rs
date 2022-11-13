use anchor_lang::prelude::*;
use ecs::state::SerializedComponent;

declare_id!("H5mieGWWK6qukHoNzbR6ysLxReeQC4JHZcNM6JkPQnm3");

pub mod account;
pub mod context;
pub mod constant;
pub mod error;
pub mod event;
pub mod state;

//use account::*;
use context::*;
//use constant::*;
//use error::*;
use event::*;
//use state::*;

#[program]
pub mod dominariworld {
    use super::*;

    pub fn initalize(ctx:Context<Initialize>, universe: Pubkey) -> Result<()> {
        ctx.accounts.world_config.universe = universe;
        ctx.accounts.world_config.instances = 0;
        ctx.accounts.world_config.components = 0;
        Ok(())
    }

    /**
     * Instance World should normally be regulated by governance; 
     * In this case, we allow anyone to instance a new dominari world.
     * We also set the Instance Authority for the World to the Payer
     * This authority is the only one that can add systems to a given instance
     */
    pub fn instance_world(ctx:Context<InstanceWorld>) -> Result<()> {
        let universe = ctx.accounts.universe.to_account_info();
        let accounts = ecs::cpi::accounts::RegisterWorldInstance {
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            world_instance: ctx.accounts.world_instance.to_account_info(),
            world_signer: ctx.accounts.world_config.to_account_info()
        };
        let world_signer_seeds:&[&[u8]] = &[
            b"world_signer",
            &[*ctx.bumps.get("world_config").unwrap()]
        ];
        let signer_seeds = &[world_signer_seeds];

        let register_world_ctx = CpiContext::new_with_signer(
            universe,
            accounts,
            signer_seeds
        );
        
        ecs::cpi::register_world(register_world_ctx, ctx.program_id.clone(), ctx.accounts.world_config.instances+1_u64)?;
        ctx.accounts.world_config.instances += 1; // basically the UUID for the instance
        ctx.accounts.instance_authority.instance = ctx.accounts.world_config.instances;
        ctx.accounts.instance_authority.authority = ctx.accounts.payer.key(); // fancier Worlds might have different governance setup for this

        emit!(NewWorldInstance{
            world_instance: ctx.accounts.world_instance.key(),
            instance_authority: ctx.accounts.instance_authority.key()
        });
        
        Ok(())
    }

    /**
     * Anyone can register new components as long as they use unique URIs
     */
    pub fn register_component(ctx:Context<RegisterComponent>, schema:String) -> Result<()> {
        ctx.accounts.component.url = schema.clone();
        ctx.accounts.world_config.components += 1;

        emit!(NewComponentRegistered{
            component: ctx.accounts.component.key(),
            schema: schema
        });
        Ok(())
    }

    pub fn register_system(ctx: Context<RegisterSystem>) -> Result<()> {
        ctx.accounts.system_registration.system = ctx.accounts.system.key();

        emit!(NewSystemRegistration {
            world_instance: ctx.accounts.world_instance.key(),
            system: ctx.accounts.system.key(),
            system_registration: ctx.accounts.system_registration.key()
        });
        Ok(())
    }

    pub fn add_components_to_system_registration(ctx:Context<AddComponentsToSystemRegistration>, components:Vec<Pubkey>) -> Result<()> {
        ctx.accounts.system_registration.components.append(components.clone().as_mut());
        Ok(())
    }

    pub fn mint_entity(ctx:Context<MintEntity>) -> Result<()> {
        let accounts = ecs::cpi::accounts::MintEntity {
            entity: ctx.accounts.entity.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            world_instance: ctx.accounts.world_instance.to_account_info(),
            world_signer: ctx.accounts.world_config.to_account_info(),
        };  
        let world_signer_seeds:&[&[u8]] = &[
            b"world_signer",
            &[*ctx.bumps.get("world_config").unwrap()]
        ];
        let signer_seeds = &[world_signer_seeds];

        ecs::cpi::mint_entity(CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            accounts,
            signer_seeds
        ))?;

        Ok(())
    }

    pub fn req_add_component(ctx:Context<AddComponents>, components: Vec<SerializedComponent>) -> Result<()> {
        let accounts = ecs::cpi::accounts::AddComponent {
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            entity: ctx.accounts.entity.to_account_info(),
            world_signer: ctx.accounts.world_config.to_account_info()
        };
        let world_signer_seeds:&[&[u8]] = &[
            b"world_signer",
            &[*ctx.bumps.get("world_config").unwrap()]
        ];
        let signer_seeds = &[world_signer_seeds];
        
        ecs::cpi::add_components(CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            accounts,
            signer_seeds
        ), components)?;

        //No need to emit an event, as Universe will do so
        Ok(())
    }

    pub fn req_remove_component(ctx:Context<RemoveComponent>, components: Vec<Pubkey>) -> Result<()> {
        let accounts = ecs::cpi::accounts::RemoveComponent {
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            entity: ctx.accounts.entity.to_account_info(),
            world_signer: ctx.accounts.world_config.to_account_info()
        };
        let world_signer_seeds:&[&[u8]] = &[
            b"world_signer",
            &[*ctx.bumps.get("world_config").unwrap()]
        ];
        let signer_seeds = &[world_signer_seeds];
        
        ecs::cpi::remove_component(CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            accounts,
            signer_seeds
        ), components)?;

        //No need to emit an event, as Universe will do so
        Ok(())
    }

    /**
     * In this implementation we can only modify one component at at time because we have 
     * 1:1 mapping between system registration to component accounts.
     * 
     * Universe contract supports batch modification but would require redoing how system registration
     * works on the world level.
     */
    pub fn req_modify_component(ctx:Context<ModifyComponent>, components: Vec<Pubkey>, data:Vec<Vec<u8>>) -> Result<()> {
        let accounts = ecs::cpi::accounts::ModifyComponent {
            entity: ctx.accounts.entity.to_account_info(),
            world_signer: ctx.accounts.world_config.to_account_info()
        };
        let world_signer_seeds:&[&[u8]] = &[
            b"world_signer",
            &[*ctx.bumps.get("world_config").unwrap()]
        ];
        let signer_seeds = &[world_signer_seeds];
        
        ecs::cpi::modify_components(CpiContext::new_with_signer(
            ctx.accounts.universe.to_account_info(),
            accounts,
            signer_seeds
        ), components, data)?;

        //No need to emit an event, as Universe will do so
        Ok(())
    }
    
}