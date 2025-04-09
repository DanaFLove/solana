use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
    program::invoke,
    system_instruction,
};
use crate::{state::{MixerState, MIXER_STATE_SIZE}, utils::{hash_commitment, verify_merkle_proof}};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?; // User sending SOL
    let mixer_account = next_account_info(accounts_iter)?; // Program-owned account for state
    let system_program = next_account_info(accounts_iter)?; // System program for transfers

    if mixer_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    match instruction_data[0] {
        0 => {
            // Deposit instruction
            let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            let secret = &instruction_data[9..41]; // 32-byte secret

            // Create commitment
            let commitment = hash_commitment(amount, secret);

            // Transfer SOL to mixer account
            let transfer_ix = system_instruction::transfer(payer.key, mixer_account.key, amount);
            invoke(&transfer_ix, &[payer.clone(), mixer_account.clone(), system_program.clone()])?;

            // Update state (simplified: append commitment, update root)
            let mut mixer_data = mixer_account.data.borrow_mut();
            if mixer_data.len() < MIXER_STATE_SIZE {
                return Err(ProgramError::UninitializedAccount);
            }
            let mut state = MixerState::deserialize(&mixer_data)?;
            state.add_commitment(commitment)?;
            state.serialize(&mut mixer_data)?;

            msg!("Deposited {} lamports with commitment", amount);
            Ok(())
        }
        1 => {
            // Withdraw instruction
            let nullifier = &instruction_data[1..33]; // 32-byte nullifier
            let proof = &instruction_data[33..]; // Merkle proof (variable length)

            let mut mixer_data = mixer_account.data.borrow_mut();
            let mut state = MixerState::deserialize(&mixer_data)?;

            // Verify nullifier not spent
            if state.is_nullifier_spent(nullifier) {
                return Err(ProgramError::InvalidArgument);
            }

            // Verify Merkle proof
            if !verify_merkle_proof(&state.root, proof, nullifier) {
                return Err(ProgramError::InvalidArgument);
            }

            // Mark nullifier as spent and update state
            state.mark_nullifier_spent(nullifier)?;
            state.serialize(&mut mixer_data)?;

            msg!("Withdrawal processed");
            Ok(())
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
