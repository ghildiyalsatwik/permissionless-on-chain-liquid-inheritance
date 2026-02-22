use anchor_lang::prelude::*;

use crate::errors::ProtocolError;

#[account]
#[derive(InitSpace)]
pub struct Inheritance {

    pub maker: Pubkey,
    pub seed: u64,
    pub inheritor: Pubkey,
    pub inheritance_amount: u64,
    pub bounty_amount: u64,
    pub inactivity_time: u64,
    pub last_check_in: u64,
    pub shares: u64,
    pub bump: u8

}

impl Inheritance {

    pub fn calculate_token_to_mint(deposit: u64, total_assets: u64, total_shares: u64) -> Result<u64> {

        if total_shares == 0 || total_assets == 0 {

            return Ok(deposit);
        }

        let shares = (deposit as u128).checked_mul(total_shares as u128).ok_or(ProtocolError::MathOverflow)?
        .checked_div(total_assets as u128).ok_or(ProtocolError::MathOverflow)?;

        Ok(shares as u64)
    }

    pub fn calculate_lamports_to_return(shares: u64, total_assets: u64, total_shares: u64) -> Result<u64> {

        let assets = (shares as u128).checked_mul(total_assets as u128).ok_or(ProtocolError::MathOverflow)?
        .checked_div(total_shares as u128).ok_or(ProtocolError::MathOverflow)?;

        Ok(assets as u64)

    }
}