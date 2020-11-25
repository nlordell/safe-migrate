use crate::hash;
use anyhow::{anyhow, Result};
use bip39::{Language, Mnemonic, Seed};
use secp256k1::{
    key::{PublicKey, SecretKey},
    Message, Secp256k1,
};
use tiny_hderive::bip32::ExtendedPrivKey;

/// A struct representing an Ethereum private key.
pub struct PrivateKey(SecretKey);

impl PrivateKey {
    /// Derives a private key from a mnemonic seed phrase.
    pub fn from_mnemonic(seed_phrase: impl AsRef<str>) -> Result<Self> {
        let password = "";
        let mnemonic = Mnemonic::from_phrase(seed_phrase.as_ref(), Language::English)?;
        let seed = Seed::new(&mnemonic, password);

        let derived_key = ExtendedPrivKey::derive(seed.as_bytes(), "m/44'/60'/0'/0/0")
            .map_err(|_| anyhow!("failed to derive key from seed"))?;
        let key = SecretKey::from_slice(&derived_key.secret())?;

        Ok(PrivateKey(key))
    }

    /// Returns the public address for the private key.
    pub fn address(&self) -> [u8; 20] {
        let secp = Secp256k1::signing_only();
        let public_key = PublicKey::from_secret_key(&secp, &self.0).serialize_uncompressed();

        // NOTE: An ethereum address is the last 20 bytes of the keccak hash of
        // the public key. Note that `libsecp256k1` public key is serialized
        // into 65 bytes as the first byte is always 0x04 as a tag to mark a
        // uncompressed public key. Discard it for the public address
        // calculation.
        debug_assert_eq!(public_key[0], 0x04);
        let hash = hash::keccak256(&public_key[1..]);

        let mut address = [0u8; 20];
        address.copy_from_slice(&hash[12..]);
        address
    }

    /// Generate a signature for the specified message.
    pub fn sign(&self, message: [u8; 32]) -> Signature {
        let message = Message::from_slice(&message).expect("invalid message");

        let (recovery_id, raw_signature) = Secp256k1::signing_only()
            .sign_recoverable(&message, &self.0)
            .serialize_compact();
        debug_assert!(matches!(recovery_id.to_i32(), 0 | 1));

        let mut signature = Signature::default();
        signature.v = 27 + (recovery_id.to_i32() as u8);
        signature.r.copy_from_slice(&raw_signature[..32]);
        signature.s.copy_from_slice(&raw_signature[32..]);

        signature
    }
}

/// A recoverable signature in electrum notation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Signature {
    /// Signature V value in electrum notation (either 27 or 28).
    pub v: u8,
    /// Signature R value.
    pub r: [u8; 32],
    /// Normalized signature S value, with `s < n/2 - 1`.
    pub s: [u8; 32],
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    const DETERMINISTIC_MNEMONIC: &str =
        "myth like bonus scare over problem client lizard pioneer submit female collect";

    #[test]
    fn ganache_determinitic_address() {
        let key = PrivateKey::from_mnemonic(DETERMINISTIC_MNEMONIC).unwrap();
        assert_eq!(
            key.address(),
            hex!("90F8bf6A479f320ead074411a4B0e7944Ea8c9C1"),
        );
    }

    #[test]
    fn ganache_deterministic_signature() {
        let key = PrivateKey::from_mnemonic(DETERMINISTIC_MNEMONIC).unwrap();
        let message = hash::keccak256(b"\x19Ethereum Signed Message:\n12Hello World!");
        assert_eq!(
            key.sign(message),
            Signature {
                v: 28,
                r: hex!("408790f153cbfa2722fc708a57d97a43b24429724cf060df7c915d468c43bd84"),
                s: hex!("61c96aac95ce37d7a31087b6634f4a3ea439a9f704b5c818584fa2a32fa83859"),
            },
        );
    }
}
