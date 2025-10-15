#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;


pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Hpj96iQNXfgX8umTeUMGBhjcpN5wmb8htRuhTjs8ngFX");

#[program]
pub mod whitelist_transfer_hook {
    use super::*;

    pub fn initialize_whitelist(ctx: Context<InitializeWhitelist>) -> Result<()> {
        ctx.accounts.initialize_whitelist(ctx.bumps)
    }

    // pub fn add_to_whitelist(ctx: Context<>) -> Result<<()> {
    //     ctx.accounts.add_to_whitelist()
    // }
}
