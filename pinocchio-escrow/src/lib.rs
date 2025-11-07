use pinocchio::{
    entrypoint,
    pubkey::Pubkey,
    ProgramResult,
    account_info::AccountInfo
};

mod state;
mod instructions;

entrypoint!(process_instruction);

pinocchio_pubkey::declare_id!("Fx7D2FUmfoYUCtH3WqLQ73VNjvBmd6rniT5idj1Z4VjG");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {

    assert_eq!(program_id, &ID);

    let (discriminator, data) = instruction_data.split_first().ok_or(pinocchio::program_error::ProgramError::InvalidInstructionData)?;

    

    Ok(())
}
