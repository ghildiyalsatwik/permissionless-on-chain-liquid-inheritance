use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{token::{Mint, TokenAccount}, associated_token::AssociatedToken, token_2022::{Token2022, burn, Burn}};

use crate::{errors::ProtocolError, state::{Config, Vault, Inheritance}};

#[derive(Accounts)]
pub struct ReduceInheritance<'info> {
    #[account(
        mut,
        address = inheritance.maker.key() @ ProtocolError::InvalidMaker
    )]
    pub maker: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = protocol_mint,
        associated_token::token_program = token_program,
        associated_token::authority = maker
    )]
    pub maker_ata: Account<'info, TokenAccount>,
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
    #[account(
        mut,
        seeds = [b"inheritance", maker.key().as_ref(), inheritance.inheritor.key().as_ref(), inheritance.seed.to_le_bytes().as_ref()],
        bump = inheritance.bump
    )]
    pub inheritance: Account<'info, Inheritance>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> ReduceInheritance<'info> {

    pub fn reduce_inheritance(&mut self, amount: u64) -> Result<()> {

        let now = Clock::get()?.unix_timestamp as u64;

        let last_check_in = self.inheritance.last_check_in;

        let time_elapsed = now.checked_sub(last_check_in).ok_or(ProtocolError::InvalidTimestamp)?;

        if time_elapsed >= self.inheritance.inactivity_time {

            return err!(ProtocolError::TimeElapsed);
        };

        let tokens_to_burn = Inheritance::calculate_tokens_to_burn(amount, self.config.amount_locked, self.protocol_mint.supply)?;

        let shares_minted = self.inheritance.shares;

        if tokens_to_burn > shares_minted {

            return err!(ProtocolError::InvalidTokenAmount);
        }

        if tokens_to_burn == shares_minted {

            return err!(ProtocolError::InvalidInstruction);
        }

        let shares_available = self.maker_ata.amount;

        if shares_available == 0 {

            return err!(ProtocolError::NoSharesAvailable);
        };

        if tokens_to_burn > shares_available {

            return err!(ProtocolError::InvalidTokenAmount);
        }

        let cpi_program_1 = self.token_program.to_account_info();

        let config_bump = &self.config.bump.to_le_bytes();

        let config_signer_seeds: &[&[&[u8]]]= &[&[b"config"], &[config_bump.as_ref()]];

        let cpi_accounts_1 = Burn {

            mint: self.protocol_mint.to_account_info(),
            from: self.maker_ata.to_account_info(),
            authority: self.config.to_account_info()
        };

        let cpi_ctx_1 = CpiContext::new_with_signer(cpi_program_1, cpi_accounts_1, config_signer_seeds);

        burn(cpi_ctx_1, tokens_to_burn)?;

        let vault_bump = &self.vault.bump.to_le_bytes();

        let vault_signer_seeds: &[&[&[u8]]] = &[&[b"vault"], &[vault_bump.as_ref()]];

        let cpi_program_2 = self.system_program.to_account_info();

        let cpi_accounts_2 = Transfer {

            from: self.vault.to_account_info(),
            to: self.maker.to_account_info(),
        };

        let cpi_ctx_2 = CpiContext::new_with_signer(cpi_program_2, cpi_accounts_2, vault_signer_seeds);

        transfer(cpi_ctx_2, amount)?;

        self.inheritance.inheritance_amount -= amount;

        self.config.amount_locked -= amount;

        Ok(())
    }
}