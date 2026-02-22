use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, TokenAccount}, token_2022::{Burn, Token2022, burn}};

use crate::{errors::ProtocolError, state::{Config, Inheritance, Vault}};

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = protocol_mint,
        associated_token::token_program = token_program,
        associated_token::authority = withdrawer
    )]
    pub withdrawer_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        address = config.mint.key() @ ProtocolError::InvalidMintAccount
    )]
    pub protocol_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.locked == false @ ProtocolError::ProtocolLocked
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> WithdrawSol<'info> {

    pub fn withraw_sol(&mut self, amount: u64) -> Result<()> {

        let lamports_to_return = Inheritance::calculate_lamports_to_return(amount, self.config.amount_locked, self.protocol_mint.supply)?;

        let cpi_program_1 = self.system_program.to_account_info();

        let cpi_accounts_1 = Transfer {

            from: self.vault.to_account_info(),
            to: self.withdrawer.to_account_info()
        };

        let vault_bump = &self.vault.bump.to_le_bytes();

        let vault_signer_seeds: &[&[&[u8]]] = &[&[b"vault"], &[vault_bump.as_ref()]];

        let cpi_ctx_1 = CpiContext::new_with_signer(cpi_program_1, cpi_accounts_1, vault_signer_seeds);

        transfer(cpi_ctx_1, lamports_to_return)?;

        self.config.amount_locked -= lamports_to_return;

        let cpi_program_2 = self.token_program.to_account_info();

        let cpi_accounts_2 = Burn {

            mint: self.protocol_mint.to_account_info(),
            from: self.withdrawer_ata.to_account_info(),
            authority: self.withdrawer.to_account_info()
        };

        let cpi_ctx_2 = CpiContext::new(cpi_program_2, cpi_accounts_2);

        burn(cpi_ctx_2, amount)?;

        Ok(())
    } 
}