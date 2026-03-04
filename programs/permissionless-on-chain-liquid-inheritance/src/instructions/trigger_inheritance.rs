use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}, token_2022::{Burn, burn, ID as TOKEN_2022_PROGRAM_ID}};

use crate::{errors::ProtocolError, state::{Config, Inheritance, /*Vault*/}, program::PermissionlessOnChainLiquidInheritance};

#[derive(Accounts)]
pub struct TriggerInheritance<'info> {
    #[account(mut)]
    pub keeper: Signer<'info>,
    #[account(
        mut,
        address = inheritance.initial_inheritor.key() @ ProtocolError::InvalidInheritor
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
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = program_data.upgrade_authority_address == Some(admin.key()) @ ProtocolError::InvalidAdmin
    )]
    pub admin: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"mint"],
        bump = config.mint_bump,
        address = config.mint.key() @ProtocolError::InvalidMintAccount
    )]
    pub protocol_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [b"vault"],
        bump = config.vault_bump
    )]
    pub vault: SystemAccount<'info>,
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
        seeds = [b"inheritance", inheritance.maker.key().as_ref(), inheritance.initial_inheritor.key().as_ref(), inheritance.seed.to_le_bytes().as_ref()],
        bump = inheritance.bump
    )]
    pub inheritance: Account<'info, Inheritance>,
    #[account(
        mut,
        seeds = [b"inheritance_vault", inheritance.key().as_ref()],
        bump = inheritance.vault_bump
    )]
    pub inheritance_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        constraint = token_program.key() == TOKEN_2022_PROGRAM_ID @ ProtocolError::InvalidTokenProgram     
    )]
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token: Program<'info, AssociatedToken>,
    pub program_data: Account<'info, ProgramData>,
    #[account(
        constraint = this_program.programdata_address()? == Some(program_data.key()) @ ProtocolError::InvalidProgram 
    )]
    pub this_program: Program<'info, PermissionlessOnChainLiquidInheritance>,
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

        let fee_amount = (lamports_to_transfer as u128).checked_mul(self.config.fees as u128).ok_or(ProtocolError::MathOverflow)?
        .checked_div(10_000u128).ok_or(ProtocolError::MathOverflow)? as u64;

        let final_amount = lamports_to_transfer.checked_sub(fee_amount).ok_or(ProtocolError::MathOverflow)?;

        let cpi_program_1 = self.token_program.to_account_info();

        let cpi_accounts_1 = Burn {

            mint: self.protocol_mint.to_account_info(),
            from: self.maker_ata.to_account_info(),
            authority: self.config.to_account_info()
        };

        //let config_bump = &self.config.bump.to_le_bytes();

        //let config_signer_seeds: &[&[&[u8]]] = &[&[b"config"], &[config_bump.as_ref()]];

        let config_signer_seeds: &[&[&[u8]]] = &[&[b"config", &[self.config.bump]]];

        let cpi_ctx_1 = CpiContext::new_with_signer(cpi_program_1, cpi_accounts_1, config_signer_seeds);

        burn(cpi_ctx_1, final_shares)?;

        //let vault_bump = &self.config.vault_bump.to_le_bytes();

        //let vault_signer_seeds: &[&[&[u8]]] = &[&[b"vault"], &[vault_bump.as_ref()]];

        let vault_signer_seeds: &[&[&[u8]]] = &[&[b"vault", &[self.config.vault_bump]]];

        // let inheritance_bump = &self.inheritance.bump.to_le_bytes();

        // let inheritance_maker_key = &self.inheritance.maker.key();

        // let inheritance_inheritor_key = &self.inheritance.current_inheritor.key();

        // let inheritance_account_seed = &self.inheritance.seed.to_le_bytes();

        // let inheritance_signer_seeds: &[&[&[u8]]] = &[&[b"inheritance", inheritance_maker_key.as_ref(), inheritance_inheritor_key.as_ref(), inheritance_account_seed.as_ref()], &[inheritance_bump.as_ref()]];

        let inheritance_key = &self.inheritance.key();

        let inheritance_signer_seeds: &[&[&[u8]]] = &[&[b"inheritance_vault", inheritance_key.as_ref(), &[self.inheritance.vault_bump]]];

        let cpi_program_2 = self.system_program.to_account_info();

        let cpi_accounts_2 = Transfer {

            from: self.vault.to_account_info(),
            to: self.inheritor.to_account_info()
        };

        let cpi_ctx_2 = CpiContext::new_with_signer(cpi_program_2, cpi_accounts_2, vault_signer_seeds);

        transfer(cpi_ctx_2, final_amount)?;

        let cpi_program_3 = self.system_program.to_account_info();

        let cpi_accounts_3 = Transfer {

            from: self.inheritance_vault.to_account_info(),
            to: self.keeper.to_account_info()
        };

        let cpi_ctx_3 = CpiContext::new_with_signer(cpi_program_3, cpi_accounts_3, inheritance_signer_seeds);

        transfer(cpi_ctx_3, self.inheritance.bounty_amount)?;

        let cpi_program_4 = self.system_program.to_account_info();

        let cpi_accounts_4 = Transfer {

            from: self.vault.to_account_info(),
            to: self.admin.to_account_info()
        };

        let cpi_ctx_4 = CpiContext::new_with_signer(cpi_program_4, cpi_accounts_4, vault_signer_seeds);

        transfer(cpi_ctx_4, fee_amount)?;

        //let cpi_program_5 = self.system_program.to_account_info();

        // let cpi_accounts_5 = Transfer {

        //     from: self.inheritance_vault.to_account_info(),
        //     to: self.keeper.to_account_info()
        // };

        // let inheritance_key = &self.inheritance.key();

        // let inheritance_vault_signer_seeds: &[&[&[u8]]] = &[&[b"inheritance_vault", inheritance_key.as_ref(), &[self.inheritance.vault_bump]]];

        // let cpi_ctx_5 = CpiContext::new_with_signer(cpi_program_5, cpi_accounts_5, inheritance_vault_signer_seeds);

        // let rent = self.inheritance_vault.to_account_info().lamports();

        // transfer(cpi_ctx_5, rent)?;

        let vault_info = self.inheritance_vault.to_account_info();
        
        let keeper_info = self.keeper.to_account_info();

        let rent = **vault_info.lamports.borrow();

        **vault_info.lamports.borrow_mut() = 0;
        
        **keeper_info.lamports.borrow_mut() += rent;

        self.config.amount_locked -= lamports_to_transfer;

        Ok(())
    }
}