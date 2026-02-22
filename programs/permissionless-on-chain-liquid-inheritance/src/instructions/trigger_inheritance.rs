use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, TokenAccount}, token_2022::{Burn, Token2022, burn}};

use crate::{errors::ProtocolError, state::{Config, Inheritance, Vault}};

#[derive(Accounts)]
pub struct TriggerInheritance<'info> {
    #[account(mut)]
    pub keeper: Signer<'info>,
    #[account(
        mut,
        address = inheritance.inheritor.key() @ ProtocolError::InvalidInheritor
    )]
    pub inheritor: SystemAccount<'info>,
    #[account(
        mut,
        address = inheritance.maker.key() @ProtocolError::InvalidMaker
    )]
    pub maker: SystemAccount<'info>,
    #[account(
        mut,
        associated_token::mint = protocol_mint,
        associated_token::token_program = token_program,
        associated_token::authority = maker
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        address = config.mint.key() @ProtocolError::InvalidMintAccount
    )]
    pub protocol_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.locked == false @ ProtocolError::ProtocolLocked
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        close = keeper,
        seeds = [b"inheritance", inheritance.maker.key().as_ref(), inheritance.inheritor.key().as_ref(), inheritance.seed.to_le_bytes().as_ref()],
        bump = inheritance.bump
    )]
    pub inheritance: Account<'info, Inheritance>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token: Program<'info, AssociatedToken>
}

impl<'info> TriggerInheritance<'info> {

    pub fn trigger_inheritance(&mut self) -> Result<()> {

        let now = Clock::get()?.unix_timestamp as u64;

        let last_check_in = self.inheritance.last_check_in;

        let time_elapsed = now.checked_sub(last_check_in).ok_or(ProtocolError::InvalidTimestamp)?;

        if time_elapsed < self.inheritance.inactivity_time {

            return err!(ProtocolError::TimeNotElapsed);
        };

        let shares = self.inheritance.shares;

        let balance = self.maker_ata.amount;

        let final_shares: u64;

        if balance >= shares {

            final_shares = shares;
        
        } else {

            final_shares = balance;
        }

        let lamports_to_transfer = Inheritance::calculate_lamports_to_return(final_shares, self.config.amount_locked, self.protocol_mint.supply)?;

        let cpi_program_1 = self.token_program.to_account_info();

        let cpi_accounts_1 = Burn {

            mint: self.protocol_mint.to_account_info(),
            from: self.maker_ata.to_account_info(),
            authority: self.config.to_account_info()
        };

        let config_bump = &self.config.bump.to_le_bytes();

        let config_signer_seeds: &[&[&[u8]]] = &[&[b"config"], &[config_bump.as_ref()]];

        let cpi_ctx_1 = CpiContext::new_with_signer(cpi_program_1, cpi_accounts_1, config_signer_seeds);

        burn(cpi_ctx_1, final_shares)?;

        let vault_bump = &self.vault.bump.to_le_bytes();

        let vault_signer_seeds: &[&[&[u8]]] = &[&[b"vault"], &[vault_bump.as_ref()]];

        let inheritance_bump = &self.inheritance.bump.to_le_bytes();

        let inheritance_maker_key = &self.inheritance.maker.key();

        let inheritance_inheritor_key = &self.inheritance.inheritor.key();

        let inheritance_account_seed = &self.inheritance.seed.to_le_bytes();

        let inheritance_signer_seeds: &[&[&[u8]]] = &[&[b"inheritance", inheritance_maker_key.as_ref(), inheritance_inheritor_key.as_ref(), inheritance_account_seed.as_ref()], &[inheritance_bump.as_ref()]];

        let cpi_program_2 = self.system_program.to_account_info();

        let cpi_accounts_2 = Transfer {

            from: self.vault.to_account_info(),
            to: self.inheritor.to_account_info()
        };

        let cpi_ctx_2 = CpiContext::new_with_signer(cpi_program_2, cpi_accounts_2, vault_signer_seeds);

        transfer(cpi_ctx_2, lamports_to_transfer)?;

        let cpi_program_3 = self.system_program.to_account_info();

        let cpi_accounts_3 = Transfer {

            from: self.inheritance.to_account_info(),
            to: self.keeper.to_account_info()
        };

        let cpi_ctx_3 = CpiContext::new_with_signer(cpi_program_3, cpi_accounts_3, inheritance_signer_seeds);

        transfer(cpi_ctx_3, self.inheritance.bounty_amount)?;

        self.config.amount_locked -= lamports_to_transfer;

        Ok(())
    }
}