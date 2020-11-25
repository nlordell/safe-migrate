use sha3::{Digest, Keccak256};

/// Returns the 32-byte hash of the input data.
pub fn keccak256(data: impl AsRef<[u8]>) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data.as_ref());
    hasher.finalize().into()
}
