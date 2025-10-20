use anchor_lang::prelude::*;

#[account]
pub struct Whitelist{
    pub user_address: Pubkey,
    pub bump: u8
}