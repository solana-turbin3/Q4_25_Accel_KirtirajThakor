use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    sysvars::{clock::Clock, rent::Rent, Sysvar},
    ProgramResult,
};

use crate::{
    helper::{load_acc_mut_unchecked, load_ix_data, DataLen},
    state::{Fundraiser},
};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::state::TokenAccount;

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InitializeIxData {
    pub amount: u64,
    pub duration: u8,
    pub bump: u8,
}

impl DataLen for InitializeIxData {
    const LEN: usize = core::mem::size_of::<InitializeIxData>();
}

pub fn process_initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, mint_to_raise, fundraiser, vault, _system_program, _token_program, _rest @ ..] =
        accounts
    else {
        return Err(ProgramError::InvalidAccountData);
    };

    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !fundraiser.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let vault_acc = TokenAccount::from_account_info(vault)?;

    assert_eq!(vault_acc.owner(), fundraiser.key());

    let rent = Rent::get()?;
    let ix_data = unsafe { load_ix_data::<InitializeIxData>(data)? };

    let bump_seed = [ix_data.bump];
    let fundraiser_seeds = [
        Seed::from(Fundraiser::SEED.as_bytes()),
        Seed::from(maker.key().as_ref()),
        Seed::from(&bump_seed[..]),
    ];

    let fundraiser_signer = Signer::from(&fundraiser_seeds[..]);

    (CreateAccount {
        from: maker,
        to: fundraiser,
        lamports: rent.minimum_balance(Fundraiser::LEN),
        space: Fundraiser::LEN as u64,
        owner: &crate::ID,
    })
    .invoke_signed(&[fundraiser_signer])?;

    let fundraiser_state =
        (unsafe { load_acc_mut_unchecked::<Fundraiser>(fundraiser.borrow_mut_data_unchecked()) })?;

    fundraiser_state.initialize(
        *maker.key(),
        *mint_to_raise.key(),
        ix_data.amount,
        ix_data.duration,
        ix_data.bump,
        Clock::get()?.unix_timestamp,
    );

    Ok(())
}
