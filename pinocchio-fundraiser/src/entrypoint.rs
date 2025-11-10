use crate::instructions::{self, ProgramInstruction};
use pinocchio::{
    account_info::AccountInfo, no_allocator, nostd_panic_handler, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};
use pinocchio_log::log;

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match ProgramInstruction::try_from(ix_disc)? {
        ProgramInstruction::Initialize => {
            log!("Initialize instruction");
            instructions::initialize::process_initialize(accounts, instruction_data)
        }
        ProgramInstruction::Contribute => {
            log!("Contribute instruction");
            instructions::contribute::process_contribute(accounts, instruction_data)
        }
        ProgramInstruction::CheckContribution => {
            log!("CheckContribution instruction");
            instructions::checker::process_check_contribution(accounts, instruction_data)
        }
        ProgramInstruction::Refund => {
            log!("Refund instruction");
            instructions::refund::process_refund(accounts, instruction_data)
        }
    }
}
