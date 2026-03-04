use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub fees: u64,
    pub amount_locked: u64,
    pub burned: u64,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub locked: bool,
    pub bump: u8,
    pub mint_bump: u8,
    pub vault_bump: u8
}