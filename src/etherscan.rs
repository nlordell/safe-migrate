use crate::safe::{data::ExecutedTransaction, Network};

/// Renders an Etherscan link for the specified transaction.
pub fn render_link(network: Network, tx: &ExecutedTransaction) -> String {
    let prefix = match network {
        Network::Mainnet => "",
        Network::Rinkeby => "rinkeby.",
    };
    format!(
        "https://{}etherscan.io/tx/0x{}",
        prefix,
        hex::encode(tx.transaction_hash),
    )
}
