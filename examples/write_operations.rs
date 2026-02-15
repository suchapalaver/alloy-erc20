use alloy::primitives::address;
use alloy::providers::ProviderBuilder;
use alloy_erc20::LazyTokenSigner;
use dotenvy::dotenv;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();

    let eth_rpc = env::var("ETH_RPC").unwrap();

    // Note: This example requires a provider with signing capabilities
    // You would need to add a wallet to the provider builder:
    //
    // let provider = ProviderBuilder::new()
    //     .with_recommended_fillers()
    //     .wallet(your_wallet)
    //     .connect_http(eth_rpc.parse().unwrap());

    // For demonstration purposes, we'll just show the API usage
    let provider = ProviderBuilder::new().connect_http(eth_rpc.parse().unwrap());

    let dai = LazyTokenSigner::new(
        address!("6B175474E89094C44Da98b954EedeAC495271d0F"), // DAI on mainnet
        provider,
    );

    // Get token information (read operations work the same as LazyToken)
    println!("Token: {}", dai.symbol().await.unwrap());
    println!("Decimals: {}", dai.decimals().await.unwrap());

    // Example: Transfer tokens
    // Uncomment when using a provider with signing capabilities
    /*
    let recipient = address!("0x0000000000000000000000000000000000000000");
    let amount = U256::from(1000000000000000000u64); // 1 token with 18 decimals

    println!("Transferring {} tokens to {}", amount, recipient);
    let pending_tx = dai.transfer(recipient, amount).await.unwrap();

    println!("Transaction sent, waiting for confirmation...");
    let receipt = pending_tx.watch().await.unwrap();

    println!("Transfer confirmed in block {}", receipt.block_number.unwrap());
    */

    // Example: Approve spender
    /*
    let spender = address!("0x0000000000000000000000000000000000000000");
    let amount = U256::from(1000000000000000000u64);

    println!("Approving {} to spend {} tokens", spender, amount);
    let pending_tx = dai.approve(spender, amount).await.unwrap();
    let receipt = pending_tx.watch().await.unwrap();

    println!("Approval confirmed in block {}", receipt.block_number.unwrap());
    */

    // Example: Transfer from (requires prior approval)
    /*
    let from = address!("0x0000000000000000000000000000000000000000");
    let to = address!("0x0000000000000000000000000000000000000000");
    let amount = U256::from(1000000000000000000u64);

    println!("Transferring {} tokens from {} to {}", amount, from, to);
    let pending_tx = dai.transfer_from(from, to, amount).await.unwrap();
    let receipt = pending_tx.watch().await.unwrap();

    println!("TransferFrom confirmed in block {}", receipt.block_number.unwrap());
    */

    println!("\nNote: Uncomment the write operation examples above when using a");
    println!("provider with signing capabilities (e.g., with a wallet attached).");
}
