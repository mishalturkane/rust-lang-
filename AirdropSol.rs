use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, system_transaction};
use solana_sdk::commitment_config::CommitmentConfig;

fn main() {
    // Define the Solana Devnet RPC URL
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Replace with your wallet's public key
    let recipient_pubkey = "mic...".parse::<Pubkey>().expect("Invalid public key");

    // Airdrop 1 SOL (1_000_000_000 lamports)
    let amount = 1_000_000_000;

    match client.request_airdrop(&recipient_pubkey, amount) {
        Ok(signature) => println!("Airdrop successful! Tx Signature: {}", signature),
        Err(err) => eprintln!("Airdrop failed: {}", err),
    }
}
