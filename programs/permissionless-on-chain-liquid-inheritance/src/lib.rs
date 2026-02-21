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
}
