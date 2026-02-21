use anchor_lang::prelude::*;

use crate::state::Inheritance;
use crate::errors::ProtocolError;

#[derive(Accounts)]
pub struct CheckIn<'info> {
    #[account(
        mut,
        address = inheritance.maker.key() @ ProtocolError::InvalidMaker
    )]
    pub maker: Signer<'info>,
    #[account(
        mut,
        seeds = [b"inheritance", maker.key().as_ref(), inheritance.inheritor.as_ref(), inheritance.seed.to_le_bytes().as_ref()],
        bump = inheritance.bump
    )]
    pub inheritance: Account<'info, Inheritance>,
    pub system_program: Program<'info, System>
}

impl<'info> CheckIn<'info> {

    pub fn check_in(&mut self) -> Result<()> {

        let now = Clock::get()?.unix_timestamp as u64;

        let time_elapsed = now.checked_sub(self.inheritance.last_check_in).ok_or(ProtocolError::InvalidTimestamp)?;

        if time_elapsed >= self.inheritance.inactivity_time {

            return err!(ProtocolError::TimeElapsed);
        }

        self.inheritance.last_check_in = now;

        Ok(())
    }
}