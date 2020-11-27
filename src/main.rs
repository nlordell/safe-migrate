mod address;
mod etherscan;
mod hash;
mod safe;
mod secret;
mod term;

use crate::{
    address::Address,
    safe::{
        abi,
        data::{EstimateParameters, Operation},
        tx::SafeTransaction,
        Client, Network,
    },
    secret::PrivateKey,
};
use anyhow::{ensure, Result};
use std::{fmt::Display, process};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "safe-migrate",
    about = "Migrate a Safe from Legacy App to Multisig"
)]
struct Options {
    /// The address of the Safe to migrate.
    #[structopt(name = "SAFE")]
    safe: Address,

    /// The address of the new owner to be added to the Safe.
    #[structopt(name = "OWNER")]
    owner: Address,

    /// The Safe's Ethereum network.
    #[structopt(long)]
    network: Option<Network>,

    /// The token to pay transaction gas in.
    #[structopt(long = "gas-token")]
    gas_token: Option<Address>,
}

fn main() {
    let options = Options::from_args();
    if let Err(err) = run(options) {
        if cfg!(debug_assertions) {
            eprintln!("ERROR: {}", err);
        } else {
            eprintln!("ERROR: {:?}", err);
        }

        process::exit(1);
    }
}

fn run(options: Options) -> Result<()> {
    let seed_phrase = term::read_password("Legacy Safe recovery phrase")?;
    let recovery_key = PrivateKey::from_mnemonic(&seed_phrase)?;
    let secondary_recovery_address = PrivateKey::from_mnemonic_at_index(&seed_phrase, 1)?.address();

    println!("Using Safe {}", options.safe);
    println!("Using Recovery accounts:");
    println!("  - {}", recovery_key.address());
    println!("  - {}", secondary_recovery_address);

    let network = options.network.unwrap_or(Network::Rinkeby);
    let client = Client::for_network(network);

    let info = client.get_safe(options.safe)?;
    {
        ensure!(info.version == "1.1.1", "unsupported Safe version");
        ensure!(
            info.owners.len() == 3 && info.threshold == 1,
            "unsupported Safe configuration"
        );
        ensure!(
            info.owners.contains(&recovery_key.address())
                && info.owners.contains(&secondary_recovery_address),
            "recovery phrase is not for this Safe"
        );
    }

    let estimate = client.estimate_safe_transaction(EstimateParameters {
        safe: options.safe,
        to: options.safe,
        value: 0,
        data: abi::add_owner_with_threshold(options.owner, 1),
        operation: Operation::Call,
        gas_token: options.gas_token,
    })?;

    term::confirm(format!(
        "About to add {} as an owner (yes to continue)",
        options.owner,
    ))?;
    term::confirm(format!(
        "Are you sure, this will add a new owner to the Safe {}",
        options.safe,
    ))?;
    term::confirm("Are you absolutely sure!")?;

    let tx = SafeTransaction {
        to: options.safe,
        value: 0,
        data: abi::add_owner_with_threshold(options.owner, 1),
        operation: Operation::Call,
        safe_tx_gas: estimate.safe_tx_gas,
        base_gas: estimate.base_gas,
        gas_price: estimate.gas_price,
        gas_token: estimate.gas_token,
        refund_receiver: estimate.refund_receiver,
        nonce: info.nonce,
    };
    println!("  to: {}", tx.to);
    println!("  value: {}", tx.value);
    println!("  data: 0x{}", hex::encode(&tx.data));
    println!("  operation: {}", tx.operation);
    println!("  safe transaction gas: {}", tx.safe_tx_gas);
    println!("  base gas: {}", tx.base_gas);
    println!("  gas price: {}", tx.gas_price);
    println!("  gas token: {}", display_option(tx.gas_token));
    println!("  refund receiver: {}", display_option(tx.refund_receiver));
    println!("  nonce: {}", tx.nonce);
    println!("  hash: 0x{}", hex::encode(tx.hash(options.safe)));
    term::confirm("Are you still 100% sure")?;

    let signed_tx = tx.sign(options.safe, &recovery_key);
    println!("Using signature {}", signed_tx.signatures[0]);
    term::confirm("Are absolutely positively undoubtedly sure")?;

    let executed_tx = client.post_transaction(signed_tx)?;
    println!("Transaction successfully relayed:");
    println!("{}", etherscan::render_link(network, &executed_tx));

    Ok(())
}

fn display_option<T>(value: Option<T>) -> String
where
    T: Display,
{
    match value {
        Some(value) => value.to_string(),
        _ => "-".to_owned(),
    }
}
