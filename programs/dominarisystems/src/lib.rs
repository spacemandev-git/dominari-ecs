use anchor_lang::prelude::*;

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

declare_id!("Gp1okeWiTe7PK58aiEdMWgQQp5DCYAkh223dARExNf1Y");

#[program]
pub mod dominarisystems {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}
