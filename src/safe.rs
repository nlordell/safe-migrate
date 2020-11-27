pub mod abi;
pub mod data;
mod http;
pub mod tx;

use self::data::*;
use crate::address::Address;
use anyhow::{bail, Result};
use std::str::FromStr;

/// Networks supporting Safe services.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u64)]
pub enum Network {
    Mainnet = 1,
    Rinkeby = 4,
}

impl FromStr for Network {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "mainnet" => Network::Mainnet,
            "rinkeby" => Network::Rinkeby,
            _ => bail!("invalid network '{}'", s),
        })
    }
}

/// A client to the Gnosis Safe Multisig services.
pub struct Client {
    relay: String,
}

impl Client {
    /// Create a new client for the specified network.
    pub fn for_network(network: Network) -> Self {
        let relay = match network {
            Network::Mainnet => "https://safe-relay.gnosis.io/api",
            Network::Rinkeby => "https://safe-relay.rinkeby.gnosis.io/api",
        };

        Client {
            relay: relay.into(),
        }
    }

    /// Retrieves the list of owners of the specified Safe.
    pub fn get_safe(&self, safe: Address) -> Result<SafeInfo> {
        http::get_json(format!("{}/v1/safes/{}/", self.relay, safe))
    }

    /// Estimates the gas for a transaction.
    pub fn estimate_safe_transaction(&self, tx: EstimateParameters) -> Result<Estimate> {
        http::post_json(
            format!("{}/v2/safes/{}/transactions/estimate/", self.relay, tx.safe),
            &tx,
        )
    }

    /// Posts a signed transaction to the relay service for execution.
    pub fn post_transaction(&self, tx: SignedSafeTransaction) -> Result<()> {
        let result: serde_json::Value = http::post_json(
            format!("{}/v1/safes/{}/transactions/", self.relay, tx.safe),
            &tx,
        )?;
        println!("{}", result);
        Ok(())
    }
}
