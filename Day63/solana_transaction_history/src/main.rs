use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_request::RpcRequest;
use serde_json::Value;

#[tokio::main]
async fn main() {
    // Replace with your RPC URL and the wallet public key you want to query
    let rpc_url = "https://api.mainnet-beta.solana.com"; // Mainnet URL
    let rpc_client = RpcClient::new(rpc_url);

    let wallet_pubkey = "your_wallet_public_key_here"; // Replace with the actual wallet public key
    let pubkey = Pubkey::from_str(wallet_pubkey).expect("Invalid public key format");

    match rpc_client.request(
        RpcRequest::GetConfirmedSignaturesForAddress2,
        serde_json::json!([pubkey.to_string(), {"limit": 10}]),
    ) {
        Ok(response) => {
            let transactions: Vec<Value> = serde_json::from_value(response).unwrap();
            println!("Recent Transactions:");
            for tx in transactions.iter() {
                if let Some(signature) = tx.get("signature") {
                    println!("Transaction Signature: {}", signature);
                }
            }
        }
        Err(err) => {
            eprintln!("Error fetching transactions: {:?}", err);
        }
    }
}
