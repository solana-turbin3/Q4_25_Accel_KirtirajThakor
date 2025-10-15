use anchor_lang::prelude::*;

#[account]
pub struct Whitelist{
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bump: u8
}