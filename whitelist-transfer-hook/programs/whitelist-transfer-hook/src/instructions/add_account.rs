use anchor_lang::prelude::*;

use crate::state::Whitelist;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct AddAccount<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 1,
        seeds = [b"whitelist", user.key().as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddAccount<'info> {
    pub fn add_account(&mut self, address: Pubkey, bump: u8) -> Result<()> {
        self.whitelist.set_inner(Whitelist {
            user_address: address,
            bump,
        });

        Ok(())
    }
}
