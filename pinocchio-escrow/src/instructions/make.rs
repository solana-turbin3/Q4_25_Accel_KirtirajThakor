use pinocchio::{
    account_info::AccountInfo,
    instruction::{
        Seed,
        Signer,
    },
    msg,
    pubkey::log,
    sysvars::{
        rent::Rent,
        Sysvar
    },
    ProgramResult
};

use pinocchio_pubkey::derive_address;
use pinocchio_system::instructions::CreateAccount;

use crate::state::Escrow;

pub fn process_make_instruction(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    msg!("Processing Make instruction");

    let [
        maker,
        mint_a,
        mint_b,
        escrow_account,
        maker_ata,
        escrow_ata,
        system_program,
        token_program,
        _associated_token_program,
        _rent_sysvar @ ..
    ] = accounts else { 
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    let maker_ata_state = pinocchio_token::state::TokenAccount::from_account_info(&maker_ata)?;

    if maker_ata_state.owner() != maker.key() {
        return Err(pinocchio::program_error::ProgramError::IllegalOwner);
    }

    if maker_ata_state.mint() != mint_a.key() {
        return Err(pinocchio::program_error::ProgramError::InvalidAccountData);
    }

    let bump = data[0];
    let seed = [b"escrow".as_ref(), maker.key().as_slice(), &[bump]];
    let seeds = &seed[..];

    let escrow_account_pda = derive_address(&seed, None, &crate::ID);

    log(&escrow_account_pda);
    log(&escrow_account.key());
    assert_eq!(escrow_account_pda, *escrow_account.key());

    let amount_to_receive = unsafe{
        *(data.as_ptr().add(1) as *const u64)
    };
    let amount_to_give = unsafe {
        *(data.as_ptr().add(9) as *const u64)
    };

    
}