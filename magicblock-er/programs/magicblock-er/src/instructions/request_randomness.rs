use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::{
    anchor::vrf,
    instructions::{create_request_randomness_ix, RequestRandomnessParams},
    types::SerializableAccountMeta,
    consts::DEFAULT_QUEUE
};

use crate::UserAccount;
use crate::instruction;

#[vrf]
#[derive(Accounts)]
pub struct FetchRandomness<'info> {
     #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    /// CHECK: VRF program identity PDA
    pub vrf_identity: UncheckedAccount<'info>,

    /// CHECK: Oracle queue for randomness
    #[account(
        mut, 
        address = DEFAULT_QUEUE
    )]
    pub oracle_queue: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: Slot hashes sysvar
    pub slot_hashes: UncheckedAccount<'info>,
}

impl<'info> FetchRandomness<'info> {
    pub fn handler(ctx: Context<FetchRandomness>, entropy: u8) -> Result<()> {

        let request_ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: ctx.accounts.user.key(),
            oracle_queue: ctx.accounts.oracle_queue.key(),
            callback_program_id: crate::ID,
            callback_discriminator: instruction::Update::DISCRIMINATOR.to_vec(),
            caller_seed: [entropy; 32],
            accounts_metas: Some(vec![SerializableAccountMeta {
                pubkey: ctx.accounts.user.key(),
                is_signer: false,
                is_writable: true,
            }]),
            ..Default::default()
        });

        ctx.accounts
            .invoke_signed_vrf(&ctx.accounts.user.to_account_info(), &request_ix)?;

        msg!("Randomness request sent with entropy: {}", entropy);
        Ok(())
    }
}
