use anchor_lang::prelude::*;

use crate::state::UserAccount;


#[derive(Accounts)]
pub struct CloseUser<'info> {
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    pub system_program: Program<'info, System>,
}

impl<'info> CloseUser<'info> {
    
    pub fn close(&mut self) -> Result<()> {
        // closing done by `close` in accounts struct
        Ok(())
    }
}