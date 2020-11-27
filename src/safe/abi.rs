use crate::{address::Address, hash};

/// ABI-encode transaction data to add a new owner.
pub fn add_owner_with_threshold(owner: Address, threshold: u32) -> Vec<u8> {
    let mut data = vec![0u8; 68];

    data[0..4].copy_from_slice(&hash::keccak256("addOwnerWithThreshold(address,uint256)")[..4]);
    data[16..36].copy_from_slice(&*owner);
    data[64..68].copy_from_slice(&threshold.to_be_bytes());

    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn add_eeeeeeee_owner() {
        assert_eq!(
            add_owner_with_threshold(Address([0xee; 20]), 0x42),
            hex!(
                "0d582f13
                 000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
                 0000000000000000000000000000000000000000000000000000000000000042"
            ),
        )
    }
}
