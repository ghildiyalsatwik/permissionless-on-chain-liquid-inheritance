use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, TokenAccount, Burn, burn}, token_2022::Token2022};

use crate::{errors::ProtocolError, state::{Config, Inheritance, Vault}};

#[derive(Accounts)]
pub struct CloseInheritance<'info> {
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
        close = maker,
        seeds = [b"inheritance", maker.key().as_ref(), inheritance.inheritor.key().as_ref(), inheritance.seed.to_le_bytes().as_ref()],
        bump = inheritance.bump
    )]
    pub inheritance: Account<'info, Inheritance>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> CloseInheritance<'info> {

    pub fn close_inheritance(&mut self) -> Result<()> {

        let now = Clock::get()?.unix_timestamp as u64;

        let last_check_in = self.inheritance.last_check_in;

        let time_elapsed = now.checked_sub(last_check_in).ok_or(ProtocolError::InvalidTimestamp)?;

        if time_elapsed >= self.inheritance.inactivity_time {

            return err!(ProtocolError::TimeElapsed);
        };

        let shares_minted = self.inheritance.shares;

        let shares_available = self.maker_ata.amount;

        if shares_available == 0 {

            return err!(ProtocolError::NoSharesAvailable);
        };

        let tokens_to_burn: u64;

        if shares_available >= shares_minted {

            tokens_to_burn = shares_minted;
        
        } else {

            tokens_to_burn = shares_available;
        }

        let lamports_to_return = Inheritance::calculate_lamports_to_return(tokens_to_burn, self.config.amount_locked, self.config.amount_locked - self.config.burned)?;

        let config_signer_seeds: &[&[&[u8]]] = &[&[b"config", &[self.config.bump]]];

        let vault_signer_seeds: &[&[&[u8]]] = &[&[b"vault", &[self.vault.bump]]];

        let inheritance_bump = &self.inheritance.bump.to_le_bytes();

        let inheritance_maker_key = &self.inheritance.maker.key();

        let inheritance_inheritor_key = &self.inheritance.inheritor.key();

        let inheritance_account_seed = &self.inheritance.seed.to_le_bytes();

        let inheritance_signer_seeds: &[&[&[u8]]] = &[&[b"inheritance", inheritance_maker_key.as_ref(), inheritance_inheritor_key.as_ref(), inheritance_account_seed.as_ref()], &[inheritance_bump.as_ref()]];

        let cpi_program_1 = self.token_program.to_account_info();

        let cpi_accounts_1 = Burn {

            mint: self.protocol_mint.to_account_info(),
            from: self.maker_ata.to_account_info(),
            authority: self.config.to_account_info()
        };

        let cpi_ctx_1 = CpiContext::new_with_signer(cpi_program_1, cpi_accounts_1, config_signer_seeds);

        burn(cpi_ctx_1, tokens_to_burn)?;

        let cpi_program_2 = self.system_program.to_account_info();

        let cpi_accounts_2 = Transfer {

            from: self.vault.to_account_info(),
            to: self.maker.to_account_info()
        };

        let cpi_ctx_2 = CpiContext::new_with_signer(cpi_program_2, cpi_accounts_2, vault_signer_seeds);

        transfer(cpi_ctx_2, lamports_to_return)?;

        let cpi_program_3 = self.system_program.to_account_info();

        let cpi_accounts_3 = Transfer {

            from: self.inheritance.to_account_info(),

            to: self.maker.to_account_info()
        };

        let cpi_ctx_3 = CpiContext::new_with_signer(cpi_program_3, cpi_accounts_3, inheritance_signer_seeds);

        transfer(cpi_ctx_3, self.inheritance.bounty_amount)?;

        self.config.amount_locked -= lamports_to_return;

        Ok(())
    }
}