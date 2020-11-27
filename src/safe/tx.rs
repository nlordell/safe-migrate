use crate::{
    address::Address,
    hash,
    safe::data::{Operation, SignedSafeTransaction},
    secret::PrivateKey,
};

/// A Safe transaction.
pub struct SafeTransaction {
    /// The transaction to address.
    pub to: Address,
    /// The amount of ETH being transaferred.
    pub value: u128,
    /// The transfer call data.
    pub data: Vec<u8>,
    /// The operation kind.
    pub operation: Operation,
    /// The safe transaction gas.
    pub safe_tx_gas: u128,
    /// The base gas required to execute the transaction.
    pub base_gas: u128,
    /// The gas price.
    pub gas_price: u128,
    /// The gas token to pay the transaction in.
    pub gas_token: Option<Address>,
    /// The receiver of the gas refund.
    pub refund_receiver: Option<Address>,
    /// The transaction nonce to use.
    pub nonce: u64,
}

impl SafeTransaction {
    /// Computes the transaction EIP-712 signing hash for this transaction.
    pub fn hash(&self, safe: Address) -> [u8; 32] {
        let mut buffer = [0u8; 66];
        buffer[0..2].copy_from_slice(b"\x19\x01");
        buffer[2..34].copy_from_slice(&{
            let mut buffer = [0u8; 64];
            buffer[0..32].copy_from_slice(&hash::keccak256(
                "EIP712Domain(\
                    address verifyingContract\
                )",
            ));
            buffer[44..64].copy_from_slice(&*safe);
            hash::keccak256(buffer)
        });
        buffer[34..66].copy_from_slice(&{
            let mut buffer = [0u8; 352];
            buffer[0..32].copy_from_slice(&hash::keccak256(
                "SafeTx(\
                    address to,\
                    uint256 value,\
                    bytes data,\
                    uint8 operation,\
                    uint256 safeTxGas,\
                    uint256 baseGas,\
                    uint256 gasPrice,\
                    address gasToken,\
                    address refundReceiver,\
                    uint256 nonce\
                )",
            ));
            buffer[44..64].copy_from_slice(&*self.to);
            buffer[80..96].copy_from_slice(&self.value.to_be_bytes());
            buffer[96..128].copy_from_slice(&hash::keccak256(&self.data));
            buffer[159] = self.operation as u8;
            buffer[176..192].copy_from_slice(&self.safe_tx_gas.to_be_bytes());
            buffer[208..224].copy_from_slice(&self.base_gas.to_be_bytes());
            buffer[240..256].copy_from_slice(&self.gas_price.to_be_bytes());
            buffer[268..288].copy_from_slice(&*self.gas_token.unwrap_or_default());
            buffer[300..320].copy_from_slice(&*self.refund_receiver.unwrap_or_default());
            buffer[344..352].copy_from_slice(&self.nonce.to_be_bytes());
            hash::keccak256(buffer)
        });

        hash::keccak256(buffer)
    }

    /// Signs a transaction with the specified private key.
    pub fn sign(&self, safe: Address, key: &PrivateKey) -> SignedSafeTransaction {
        SignedSafeTransaction {
            safe,
            to: self.to,
            value: self.value,
            data: self.data.clone(),
            operation: self.operation,
            gas_token: self.gas_token,
            safe_tx_gas: self.safe_tx_gas,
            data_gas: self.base_gas,
            gas_price: self.gas_price,
            refund_receiver: self.refund_receiver,
            nonce: self.nonce,
            signatures: vec![key.sign(self.hash(safe))],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn transaction_hash() {
        assert_eq!(
            SafeTransaction {
                to: Address([1; 20]),
                value: 2,
                data: vec![3],
                operation: Operation::Call,
                safe_tx_gas: 4,
                base_gas: 5,
                gas_price: 6,
                gas_token: Some(Address([7; 20])),
                refund_receiver: Some(Address([8; 20])),
                nonce: 9,
            }
            .hash(Address(hex!("0b54478f3a29BfAD2b67a0d7Dbe23e8f61B1EbC6"))),
            hex!("59485d05fff460e1687ea64c018781e440cbd8cb6a14c82d1ee2c7756fe4f7cb"),
        );
    }
}
