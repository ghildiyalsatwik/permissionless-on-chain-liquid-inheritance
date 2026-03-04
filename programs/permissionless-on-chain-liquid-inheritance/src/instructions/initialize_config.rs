use anchor_lang::prelude::*;
use anchor_spl::{token_2022::ID as TOKEN_2022_PROGRAM_ID, token_interface::{Mint, TokenInterface}};

use crate::{errors::ProtocolError, program::PermissionlessOnChainLiquidInheritance, state::{Config, /*Vault*/}};

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(
        mut,
        constraint = program_data.upgrade_authority_address == Some(admin.key()) @ ProtocolError::InvalidAdmin
    )]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        mint::decimals = 9,
        mint::authority = config,
        mint::token_program = token_program,
        seeds = [b"mint"],
        bump
    )]
    pub protocol_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = admin,
        space = Config::DISCRIMINATOR.len() + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        //init,
        //payer = admin,
        //space = Vault::DISCRIMINATOR.len() + Vault::INIT_SPACE,
        seeds = [b"vault"],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        constraint = this_program.programdata_address()? == Some(program_data.key()) @ ProtocolError::InvalidProgram 
    )]
    pub this_program: Program<'info, PermissionlessOnChainLiquidInheritance>,
    pub program_data: Account<'info, ProgramData>,
    pub system_program: Program<'info, System>,
    #[account(
        constraint = token_program.key() == TOKEN_2022_PROGRAM_ID @ ProtocolError::InvalidTokenProgram
    )]
    pub token_program: Interface<'info, TokenInterface>

}

impl<'info> InitializeAdmin<'info> {

    pub fn initialize_config(&mut self, bumps: InitializeAdminBumps, fees: u64) -> Result<()> {

        self.config.set_inner(Config { fees: (fees), amount_locked: (0), burned: (0), mint: (self.protocol_mint.key()), vault: (self.vault.key()), locked: false, bump: (bumps.config), mint_bump: (bumps.protocol_mint), vault_bump: (bumps.vault) });

        // self.vault.set_inner(Vault {

        //     bump: bumps.vault
        // });

        Ok(())
    }

}