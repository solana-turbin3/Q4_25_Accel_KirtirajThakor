use pinocchio::log::sol_log_64;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    ProgramResult,
    msg,
    pubkey::log,
    sysvars::{rent::Rent, Sysvar}
};
use pinocchio_pubkey::derive_address;

use crate::state::Escrow;

// flow => escrow_ata -> taker_ata_mint_a && taker_ata_mint_b -> maker_ata_mint_b;

pub fn process_take_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    msg!("Processing Take instruction");

    let [taker, mint_a, mint_b, escrow_account, maker_ata, taker_ata_mint_b, taker_ata_mint_a, escrow_ata, _system_program, _token_program, _associated_token_program, _rent_sysvar @ ..] =
        accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    let amount_to_receive = unsafe { *(data.as_ptr().add(0) as *const u64) };
    let amount_to_give = unsafe { *(data.as_ptr().add(8) as *const u64) };

    {
        pinocchio_token::instructions::Transfer {
            from: &*taker_ata_mint_b,
            to: &*maker_ata,
            authority: &taker,
            amount: amount_to_receive,
        }
        .invoke()?;
    }

    let maker_ata_state = pinocchio_token::state::TokenAccount::from_account_info(&maker_ata)?;

    let maker_key = maker_ata_state.owner();
    let escrow_state = Escrow::from_account_info(&escrow_account)?;
    let bump = escrow_state.bump;

    {
        let taker_ata_state_mint_a =
            pinocchio_token::state::TokenAccount::from_account_info(&taker_ata_mint_a)?;

        if taker_ata_state_mint_a.owner() != taker.key() {
            return Err(pinocchio::program_error::ProgramError::IllegalOwner);
        }

        let taker_ata_state_mint_b =
            pinocchio_token::state::TokenAccount::from_account_info(&taker_ata_mint_b)?;

        if taker_ata_state_mint_b.owner() != taker.key() {
            return Err(pinocchio::program_error::ProgramError::IllegalOwner);
        }

        if taker_ata_state_mint_a.mint() != mint_a.key() {
            return Err(pinocchio::program_error::ProgramError::InvalidAccountData);
        }

        if taker_ata_state_mint_b.mint() != mint_b.key() {
            return Err(pinocchio::program_error::ProgramError::InvalidAccountData);
        }

        if maker_ata_state.mint() != mint_b.key() {
            return Err(pinocchio::program_error::ProgramError::InvalidAccountData);
        }

        let seed = [b"escrow".as_ref(), maker_key.as_slice(), &[bump]];

        let escrow_account_pda = derive_address(&seed, None, &crate::ID);
        log(&escrow_account_pda);
        log(escrow_account.key());

        if escrow_account_pda != *escrow_account.key() {
            return Err(pinocchio::program_error::ProgramError::InvalidAccountData);
        }
    }

    let bump = [bump];
    let seed = [
        Seed::from(b"escrow"),
        Seed::from(maker_key),
        Seed::from(&bump),
    ];
    let signer_seeds = Signer::from(&seed);

    pinocchio_token::instructions::Transfer {
        from: &escrow_ata,
        to: &taker_ata_mint_a,
        authority: &escrow_account,
        amount: amount_to_give,
    }
    .invoke_signed(&[signer_seeds])?;

    Ok(())
}
