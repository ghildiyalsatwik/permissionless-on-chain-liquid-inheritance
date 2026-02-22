use anchor_lang::prelude::*;

use crate::{errors::ProtocolError, state::{Inheritance, Config}};

#[derive(Accounts)]
pub struct ChangeInheritor<'info> {
    #[account(
        mut,
        address = inheritance.maker.key() @ ProtocolError::InvalidMaker
    )]
    pub maker: Signer<'info>,
    #[account(
        mut,
        seeds = [b"inheritance", maker.key().as_ref(), inheritance.inheritor.key().as_ref(), inheritance.seed.to_le_bytes().as_ref()],
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

impl<'info> ChangeInheritor<'info> {

    pub fn change_inheritor(&mut self, inheritor: Pubkey) -> Result<()> {

        let now = Clock::get()?.unix_timestamp as u64;

        let last_check_in = self.inheritance.last_check_in;

        let time_elapsed = now.checked_sub(last_check_in).ok_or(ProtocolError::InvalidTimestamp)?;

        if time_elapsed >= self.inheritance.inactivity_time {

            return err!(ProtocolError::TimeElapsed);
        };

        self.inheritance.last_check_in = now;

        self.inheritance.inheritor = inheritor;

        Ok(())

    }
}