use anchor_lang::prelude::*;

use crate::{errors::ProtocolError, state::Config, program::PermissionlessOnChainLiquidInheritance};

#[derive(Accounts)]
pub struct UpdateConfigBurned<'info> {
    #[account(
        mut,
        constraint = this_program.programdata_address()? == Some(admin.key()) @ ProtocolError::InvalidAdmin
    )]
    pub admin: Signer<'info>,
    pub this_program: Program<'info, PermissionlessOnChainLiquidInheritance>,
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.locked == true @ ProtocolError::ProtocolUnlocked
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>
}

impl<'info> UpdateConfigBurned<'info> {

    pub fn update_config_burned(&mut self, amount: u64) -> Result<()> {

        self.config.burned += amount;

        Ok(())
    } 
}