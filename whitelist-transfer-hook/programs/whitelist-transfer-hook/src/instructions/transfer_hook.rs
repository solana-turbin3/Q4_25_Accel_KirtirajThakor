use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, Mint};

use crate::Whitelist;

#[derive(Accounts)]
pub struct TransferHook<'info>{
    pub source_token: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    pub destination_token: InterfaceAccount<'info, TokenAccount>,

    pub owner: UncheckedAccount<'info>,

    pub extra_account_meta_list: UncheckedAccount<'info>,

    pub whitelist: Account<'info, Whitelist>,
}