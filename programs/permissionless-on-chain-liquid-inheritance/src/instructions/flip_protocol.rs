use anchor_lang::prelude::*;

use crate::{errors::ProtocolError, state::Config, program::PermissionlessOnChainLiquidInheritance};

#[derive(Accounts)]
pub struct FlipProtocol<'info> {
    #[account(
        mut,
        constraint = this_program.programdata_address()? == Some(admin.key()) @ ProtocolError::InvalidAdmin
    )]
    pub admin: Signer<'info>,
    pub this_program: Program<'info, PermissionlessOnChainLiquidInheritance>,
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>
}

impl<'info> FlipProtocol<'info> {

    pub fn flip_protocol(&mut self) -> Result<()> {

        self.config.locked = !self.config.locked;

        Ok(())
    } 
}