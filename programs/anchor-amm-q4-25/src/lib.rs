use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;

use instructions::*;
declare_id!("9FVkz5L9LZix4zXYmqJLzKBwRpm9aJ539J26UsZSrLWN");

#[program]
pub mod permissionless_on_chain_liquid_inheritance {
    use super::*;
}
