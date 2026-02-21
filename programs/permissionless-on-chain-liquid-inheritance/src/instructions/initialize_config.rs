use anchor_lang::prelude::*;
use anchor_spl::{token::Mint, token_2022::Token2022};

use crate::{errors::ProtocolError, program::PermissionlessOnChainLiquidInheritance, state::{Config, Vault}};

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(
        mut,
        constraint = this_program.programdata_address()? == Some(admin.key()) @ ProtocolError::InvalidAdmin
    )]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        mint::decimals = 9,
        mint::authority = config
    )]
    pub protocol_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        space = Config::DISCRIMINATOR.len() + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        space = Vault::DISCRIMINATOR.len() + Vault::INIT_SPACE,
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub this_program: Program<'info, PermissionlessOnChainLiquidInheritance>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>

}

impl<'info> InitializeAdmin<'info> {

    pub fn initialize_config(&mut self, bumps: InitializeAdminBumps, fees: u64) -> Result<()> {

        self.config.set_inner(Config { fees: (fees), locked: (0), burned: (0), mint: (self.protocol_mint.key()), vault: (self.vault.key()), bump: (bumps.config) });

        self.vault.set_inner(Vault {

            bump: bumps.vault
        });

        Ok(())
    }

}