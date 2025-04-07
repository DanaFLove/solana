use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?; // User sending SOL
    let mixer_account = next_account_info(accounts_iter)?; // Program-owned account for pool
    let system_program = next_account_info(accounts_iter)?; // System program for transfers

    // Ensure the mixer account is rent-exempt
    let rent = Rent::get()?;
    if !rent.is_exempt(mixer_account.lamports(), mixer_account.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    // Parse instruction data (e.g., deposit or withdraw)
    match instruction_data[0] {
        0 => {
            // Deposit logic
            let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            msg!("Depositing {} lamports", amount);

            // Transfer SOL from payer to mixer account
            let transfer_instruction = solana_program::system_instruction::transfer(
                payer.key,
                mixer_account.key,
                amount,
            );
            solana_program::program::invoke(
                &transfer_instruction,
                &[payer.clone(), mixer_account.clone(), system_program.clone()],
            )?;

            // Store commitment (hash of amount + secret) in mixer account data
            // (Simplified here; real implementation would use a Merkle tree)
            msg!("Commitment stored");
        }
        1 => {
            // Withdrawal logic
            msg!("Processing withdrawal");
            // Verify proof (e.g., nullifier check) and transfer SOL to new address
            // Add obfuscation logic here
        }
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    Ok(())
}
