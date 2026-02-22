use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;

use instructions::*;
declare_id!("9FVkz5L9LZix4zXYmqJLzKBwRpm9aJ539J26UsZSrLWN");

#[program]
pub mod permissionless_on_chain_liquid_inheritance {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeAdmin>, fees: u64) -> Result<()> {

        ctx.accounts.initialize_config(ctx.bumps, fees)
    }

    pub fn update_config_fees(ctx: Context<UpdateConfigFees>, fees: u64) -> Result<()> {

        ctx.accounts.update_config_fees(fees)
    }

    pub fn update_config_burned(ctx: Context<UpdateConfigBurned>, amount: u64) -> Result<()> {

        ctx.accounts.update_config_burned(amount)
    }

    pub fn flip_protocol(ctx: Context<FlipProtocol>) -> Result<()> {

        ctx.accounts.flip_protocol()
    }

    pub fn initialize_inheritance(ctx: Context<InitializeInheritance>, seed: u64, inheritor: Pubkey, inheritance_amount: u64, bounty_amount: u64, inactivity_time: u64) -> Result<()> {

        ctx.accounts.initialize_inheritance(ctx.bumps, seed, inheritor, inheritance_amount, bounty_amount, inactivity_time)

    }

    pub fn check_in(ctx: Context<CheckIn>) -> Result<()> {

        ctx.accounts.check_in()
    }

    pub fn close_inheritance(ctx: Context<CloseInheritance>) -> Result<()> {

        ctx.accounts.close_inheritance()
    }

    pub fn reduce_inheritance(ctx: Context<ReduceInheritance>, amount: u64) -> Result<()> {

        ctx.accounts.reduce_inheritance(amount)
    }

    pub fn increase_inheritance(ctx: Context<IncreaseInheritance>, amount: u64) -> Result<()> {

        ctx.accounts.increase_inheritance(amount)
    }

    pub fn increase_inheritance_bounty(ctx: Context<IncreaseInheritanceBounty>, bounty_amount: u64) -> Result<()> {

        ctx.accounts.increase_inheritance_bounty(bounty_amount)
    }

    pub fn change_inheritor(ctx: Context<ChangeInheritor>, inheritor: Pubkey) -> Result<()> {

        ctx.accounts.change_inheritor(inheritor)
    }

    pub fn change_inactivity_time(ctx: Context<ChangeInactivityTime>, inactivity_time: u64) -> Result<()> {

        ctx.accounts.change_inactivity_time(inactivity_time)
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {

        ctx.accounts.withraw_sol(amount)
    }
}
