use solana_program::hash::{hashv, Hasher};

pub fn hash_commitment(amount: u64, secret: &[u8]) -> [u8; 32] {
    let mut hasher = Hasher::default();
    hasher.hash(&amount.to_le_bytes());
    hasher.hash(secret);
    hasher.result().to_bytes()
}

pub fn verify_merkle_proof(root: &[u8; 32], proof: &[u8], nullifier: &[u8; 32]) -> bool {
    // Simplified Merkle proof verification
    // In reality, proof would contain sibling hashes and path indices
    let mut current = hashv(&[nullifier]);
    for chunk in proof.chunks(32) {
        let sibling: [u8; 32] = chunk.try_into().expect("Invalid proof length");
        current = hashv(&[&current.to_bytes(), &sibling]);
    }
    current.to_bytes() == *root
}
