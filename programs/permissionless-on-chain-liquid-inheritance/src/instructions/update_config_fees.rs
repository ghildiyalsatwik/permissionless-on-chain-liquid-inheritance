use anchor_lang::prelude::*;

use crate::{errors::ProtocolError, state::Config, program::PermissionlessOnChainLiquidInheritance};

#[derive(Accounts)]
pub struct UpdateConfigFees<'info> {
    #[account(
        mut,
        constraint = program_data.upgrade_authority_address == Some(admin.key()) @ ProtocolError::InvalidAdmin
    )]
    pub admin: Signer<'info>,
    #[account(
        constraint = this_program.programdata_address()? == Some(program_data.key()) @ ProtocolError::InvalidProgram
    )]
    pub this_program: Program<'info, PermissionlessOnChainLiquidInheritance>,
    pub program_data: Account<'info, ProgramData>,
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.locked == true @ ProtocolError::ProtocolUnlocked
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>
}

impl<'info> UpdateConfigFees<'info> {

    pub fn update_config_fees(&mut self, fees: u64) -> Result<()> {

        self.config.fees = fees;

        Ok(())
    } 
}