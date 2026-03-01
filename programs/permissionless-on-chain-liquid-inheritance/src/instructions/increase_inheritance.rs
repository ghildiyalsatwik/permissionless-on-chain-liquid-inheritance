use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken, token_2022::{MintTo, mint_to, ID as TOKEN_2022_PROGRAM_ID}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{errors::ProtocolError, state::{Inheritance, Vault, Config}};

#[derive(Accounts)]
pub struct IncreaseInheritance<'info> {
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
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"mint"],
        bump = config.mint_bump,
        address = config.mint.key() @ ProtocolError::InvalidMintAccount
    )]
    pub protocol_mint: InterfaceAccount<'info, Mint>,
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
    #[account(
        constraint = token_program.key() == TOKEN_2022_PROGRAM_ID @ ProtocolError::InvalidTokenProgram     
    )]
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> IncreaseInheritance<'info> {

    pub fn increase_inheritance(&mut self, amount: u64) -> Result<()> {

        let now = Clock::get()?.unix_timestamp as u64;

        let last_check_in = self.inheritance.last_check_in;

        let time_elapsed = now.checked_sub(last_check_in).ok_or(ProtocolError::InvalidTimestamp)?;

        if time_elapsed >= self.inheritance.inactivity_time {

            return err!(ProtocolError::TimeElapsed);
        };

        self.inheritance.last_check_in = now;

        let tokens_to_mint = Inheritance::calculate_token_to_mint(amount, self.config.amount_locked, self.protocol_mint.supply)?;

        let cpi_program_1 = self.system_program.to_account_info();

        let cpi_accounts_1 = Transfer {

            from: self.maker.to_account_info(),
            to: self.vault.to_account_info()
        };

        let cpi_ctx_1 = CpiContext::new(cpi_program_1, cpi_accounts_1);

        transfer(cpi_ctx_1, amount)?;

        let cpi_program_2 = self.token_program.to_account_info();

        let config_bump = &self.config.bump.to_le_bytes();

        let config_signer_seeds: &[&[&[u8]]] = &[&[b"config"], &[config_bump.as_ref()]];

        let cpi_accounts_2 = MintTo {

            mint: self.protocol_mint.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.config.to_account_info()
        };

        let cpi_ctx_2 = CpiContext::new_with_signer(cpi_program_2, cpi_accounts_2, config_signer_seeds);

        mint_to(cpi_ctx_2, tokens_to_mint)?;

        self.config.amount_locked += amount;

        self.inheritance.inheritance_amount += amount;

        self.inheritance.shares += tokens_to_mint;

        Ok(())
    }
}