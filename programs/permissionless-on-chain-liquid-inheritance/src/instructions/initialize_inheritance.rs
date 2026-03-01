use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken, token_2022::{Approve, MintTo, approve, mint_to, ID as TOKEN_2022_PROGRAM_ID}, token_interface::{Mint, TokenInterface, TokenAccount}};

use crate::{state::{Inheritance, Config, Vault}, errors::ProtocolError};

#[derive(Accounts)]
#[instruction(seed: u64, inheritor: Pubkey)]
pub struct InitializeInheritance<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
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
        seeds = [b"mint"],
        bump = config.mint_bump,
        address = config.mint @ ProtocolError::InvalidMintAccount
    )]
    pub protocol_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = protocol_mint,
        associated_token::token_program = token_program,
        associated_token::authority = maker
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = Inheritance::DISCRIMINATOR.len() + Inheritance::INIT_SPACE,
        seeds = [b"inheritance", maker.key().as_ref(), inheritor.as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub inheritance: Account<'info, Inheritance>,
    pub system_program: Program<'info, System>,
    #[account(
        constraint = token_program.key() == TOKEN_2022_PROGRAM_ID @ ProtocolError::InvalidTokenProgram     
    )]
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> InitializeInheritance <'info> {

    pub fn initialize_inheritance(&mut self, bumps: InitializeInheritanceBumps, seed: u64, inheritor: Pubkey, inheritance_amount: u64, bounty_amount: u64, inactivity_time: u64) -> Result<()> {

        let now = Clock::get()?.unix_timestamp as u64;

        let cpi_program_1 = self.system_program.to_account_info();

        let cpi_1_accounts = Transfer {

            from: self.maker.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_1_ctx = CpiContext::new(cpi_program_1, cpi_1_accounts);

        transfer(cpi_1_ctx, inheritance_amount)?;

        let amount_locked_before = self.config.amount_locked;

        self.config.amount_locked += inheritance_amount;

        let cpi_program_2 = self.system_program.to_account_info();

        let cpi_2_accounts = Transfer {

            from: self.maker.to_account_info(),
            to: self.inheritance.to_account_info(),
        };

        let cpi_2_ctx = CpiContext::new(cpi_program_2, cpi_2_accounts);

        transfer(cpi_2_ctx, bounty_amount)?;

        let cpi_program_3 = self.token_program.to_account_info();

        let cpi_3_accounts = MintTo {

            mint: self.protocol_mint.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.config.to_account_info()

        };

        let bump = self.config.bump;

        let signer_seeds: &[&[&[u8]]] = &[&[b"config", &[bump]]];

        let cpi_3_ctx = CpiContext::new_with_signer(cpi_program_3, cpi_3_accounts, signer_seeds);

        let mint_amount = Inheritance::calculate_token_to_mint(inheritance_amount, amount_locked_before, self.protocol_mint.supply)?;

        self.inheritance.set_inner(Inheritance { maker: (self.maker.key()), seed, inheritor, inheritance_amount, bounty_amount, inactivity_time, last_check_in: (now), shares: (mint_amount), bump: (bumps.inheritance) });

        mint_to(cpi_3_ctx, mint_amount)?;

        let cpi_program_4 = self.token_program.to_account_info();

        let cpi_4_accounts = Approve {

            to: self.maker_ata.to_account_info(),
            delegate: self.config.to_account_info(),
            authority: self.maker.to_account_info()

        };

        let cpi_4_ctx = CpiContext::new(cpi_program_4, cpi_4_accounts);

        approve(cpi_4_ctx, mint_amount)?;

        Ok(())
    }
}