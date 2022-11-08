use anchor_lang::prelude::*;

use crate::account::*;



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
        space=8
    )]
    pub system_signer: Account<'info, SystemConfig>
}
