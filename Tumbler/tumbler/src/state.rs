use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
};

pub const MIXER_STATE_SIZE: usize = 1024; // Example size, adjust as needed

#[derive(Debug)]
pub struct MixerState {
    pub root: [u8; 32],           // Merkle tree root
    pub commitments: Vec<[u8; 32]>, // List of commitments (simplified for example)
    pub nullifiers: Vec<[u8; 32]>, // Spent nullifiers
}

impl Sealed for MixerState {}

impl Pack for MixerState {
    const LEN: usize = MIXER_STATE_SIZE;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        self.serialize(dst).expect("Serialization failed");
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        Self::deserialize(src)
    }
}

impl MixerState {
    pub fn deserialize(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() < 32 {
            return Err(ProgramError::InvalidAccountData);
        }
        let root = data[0..32].try_into().unwrap();
        // Simplified: Assume commitments and nullifiers are stored elsewhere or truncated
        Ok(MixerState {
            root,
            commitments: Vec::new(), // In practice, deserialize from a separate account
            nullifiers: Vec::new(),
        })
    }

    pub fn serialize(&self, data: &mut [u8]) -> Result<(), ProgramError> {
        if data.len() < 32 {
            return Err(ProgramError::InvalidAccountData);
        }
        data[0..32].copy_from_slice(&self.root);
        // Add logic for commitments and nullifiers as needed
        Ok(())
    }

    pub fn add_commitment(&mut self, commitment: [u8; 32]) -> Result<(), ProgramError> {
        self.commitments.push(commitment);
        // Update root (simplified; real impl recomputes Merkle tree)
        self.root = commitment; // Placeholder
        Ok(())
    }

    pub fn mark_nullifier_spent(&mut self, nullifier: &[u8; 32]) -> Result<(), ProgramError> {
        self.nullifiers.push(*nullifier);
        Ok(())
    }

    pub fn is_nullifier_spent(&self, nullifier: &[u8; 32]) -> bool {
        self.nullifiers.contains(nullifier)
    }
}
