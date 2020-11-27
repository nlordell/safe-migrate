use crate::address::Address;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use serde_with::{serde_as, skip_serializing_none};
use std::fmt::{self, Display, Formatter};

/// Safe information returned by the relay service.
#[derive(Debug, Deserialize)]
pub struct SafeInfo {
    /// The current Safe transaction nonce.
    pub nonce: u64,
    /// The signature threshold required to execute a transaction.
    pub threshold: usize,
    /// The Safe owners.
    pub owners: Vec<Address>,
    /// The Safe contract version.
    pub version: String,
}

/// Safe operation kind.
#[derive(Clone, Copy, Debug, Serialize_repr)]
#[repr(u8)]
pub enum Operation {
    Call = 0,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("call")
    }
}

/// Safe transaction estimate parameters.
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateParameters {
    /// The Safe to estimate the transaction for.
    pub safe: Address,
    /// The transaction destination address.
    pub to: Address,
    /// The ETH value of the transaction.
    pub value: u128,
    /// The transaction data.
    #[serde(with = "prefixed_hex")]
    pub data: Vec<u8>,
    /// The operation kind.
    pub operation: Operation,
    /// The token to use for paying gas.
    pub gas_token: Option<Address>,
}

/// Safe transaction estimate result.
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Estimate {
    /// The gas cost for the Safe transaction.
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub safe_tx_gas: u128,
    /// The base gas cost for the transaction.
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub base_gas: u128,
    /// The gas price to use.
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub gas_price: u128,
    /// The last used nonce for the safe.
    pub last_used_nonce: Option<u64>,
    /// The gas token used for the estimate.
    pub gas_token: Option<Address>,
    /// The receiver of the gas refund.
    pub refund_receiver: Option<Address>,
}

/// A signed Safe transaction.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedSafeTransaction {
    /// The Safe to execute the transaction.
    pub safe: Address,
    /// The transaction to address.
    pub to: Address,
    /// The amount of ETH being transaferred.
    pub value: u128,
    /// The transfer call data.
    #[serde(with = "prefixed_hex")]
    pub data: Vec<u8>,
    /// The operation kind.
    pub operation: Operation,
    /// The gas token to pay the transaction in.
    pub gas_token: Option<Address>,
    /// The safe transaction gas.
    pub safe_tx_gas: u128,
    /// The base gas required to execute the transaction.
    pub data_gas: u128,
    /// The gas price.
    pub gas_price: u128,
    /// The receiver of the gas refund.
    pub refund_receiver: Option<Address>,
    /// The transaction nonce to use.
    pub nonce: u64,
    /// The owner sigatures for the transaction.
    pub signatures: Vec<Signature>,
}

/// A safe signature.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    /// Signature V value.
    pub v: u8,
    /// Signature R value.
    #[serde(with = "bigint")]
    pub r: [u8; 32],
    /// Signature S value.
    #[serde(with = "bigint")]
    pub s: [u8; 32],
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "0x{}{}{:02x}",
            hex::encode(&self.r),
            hex::encode(&self.s),
            self.v
        )
    }
}

mod prefixed_hex {
    use serde::ser::Serializer;

    pub fn serialize<T, S>(value: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsRef<[u8]>,
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(value)))
    }
}

mod bigint {
    use serde::ser::{Serialize, Serializer};
    use serde_json::Number;
    use std::cmp;

    pub fn serialize<T, S>(value: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsRef<[u8]>,
        S: Serializer,
    {
        let number = serde_json::from_str::<Number>(&htod(value.as_ref())).expect("invalid number");
        Serialize::serialize(&number, serializer)
    }

    fn htod(data: &[u8]) -> String {
        let mut digits = vec![0];

        let mut power = vec![1];
        let mut buf = vec![1];

        let bits = data.len() * 8;
        for bit in 0..bits {
            let byte = data[data.len() - 1 - bit / 8];
            let is_set = (byte >> (bit % 8)) & 0x1 != 0;
            if is_set {
                add(&mut digits, &power);
            }

            buf.clear();
            buf.extend_from_slice(&power);
            add(&mut power, &buf);
        }

        digits.reverse();
        for d in &mut digits {
            *d += b'0';
        }
        unsafe { String::from_utf8_unchecked(digits) }
    }

    fn add(x: &mut Vec<u8>, y: &[u8]) {
        let mut carry = 0;
        x.resize(cmp::max(x.len(), y.len()), 0);
        for (i, d) in x.iter_mut().enumerate() {
            *d += carry + y.get(i).copied().unwrap_or_default();
            carry = *d / 10;
            *d %= 10;
        }
        if carry == 1 {
            x.push(1)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use hex_literal::hex;

        #[test]
        fn hex_to_decimal() {
            assert_eq!(htod(b"\x13\x37"), 0x1337.to_string());
            assert_eq!(
                htod(&hex!(
                    "4219012af844056582bc69399c238dd2089815a4164d46b9c43ce315852c5aee"
                )),
                "29896827243324578634929412110615083579682215894261272964879659419979876162286",
            );
        }
    }
}
