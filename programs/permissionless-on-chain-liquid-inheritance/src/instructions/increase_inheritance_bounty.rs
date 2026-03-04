use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::{errors::ProtocolError, state::{/*Vault,*/ Inheritance, Config}};

#[derive(Accounts)]
pub struct IncreaseInheritanceBounty<'info> {
    #[account(
        mut,
        address = inheritance.maker.key() @ ProtocolError::InvalidMaker
    )]
    pub maker: Signer<'info>,
    #[account(
        mut,
        seeds = [b"inheritance_vault", inheritance.key().as_ref()],
        bump = inheritance.vault_bump
    )]
    pub inheritance_vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"inheritance", maker.key().as_ref(), inheritance.initial_inheritor.key().as_ref(), inheritance.seed.to_le_bytes().as_ref()],
        bump = inheritance.bump
    )]
    pub inheritance: Account<'info, Inheritance>,
    #[account(
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.locked == false @ ProtocolError::ProtocolLocked
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>
}

impl<'info> IncreaseInheritanceBounty<'info> {

    pub fn increase_inheritance_bounty(&mut self, bounty_amount: u64) -> Result<()> {

        let now = Clock::get()?.unix_timestamp as u64;

        let last_check_in = self.inheritance.last_check_in;

        let time_elapsed = now.checked_sub(last_check_in).ok_or(ProtocolError::InvalidTimestamp)?;

        if time_elapsed >= self.inheritance.inactivity_time {

            return err!(ProtocolError::TimeElapsed);
        };

        self.inheritance.last_check_in = now;

        self.inheritance.bounty_amount += bounty_amount;

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {

            from: self.maker.to_account_info(),
            to: self.inheritance_vault.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, bounty_amount)?;

        Ok(())
    }
}