use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub fees: u64,
    pub locked: u64,
    pub burned: u64,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub bump: u8
}