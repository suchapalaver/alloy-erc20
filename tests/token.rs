mod common;

use alloy::primitives::U256;
use alloy_erc20::{Erc20ProviderExt, Token};
use common::{TestContext, ANVIL_ADDRESS_0, ONE_TOKEN};

#[tokio::test]
async fn test_retrieve_token_symbol() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token = provider.retrieve_token(token_address).await.unwrap();

    assert_eq!(token.symbol, "TEST");
}

#[tokio::test]
async fn test_retrieve_token_decimals() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token = provider.retrieve_token(token_address).await.unwrap();

    assert_eq!(token.decimals, 18);
}

#[tokio::test]
async fn test_retrieve_token_address() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token = provider.retrieve_token(token_address).await.unwrap();

    assert_eq!(token.address, token_address);
}

#[tokio::test]
async fn test_token_get_balance_conversion() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token = provider.retrieve_token(token_address).await.unwrap();
    let amount = U256::from(ONE_TOKEN);
    let balance = token.get_balance(amount);

    assert_eq!(balance.to_string(), "1.000000000000000000");
}

#[tokio::test]
async fn test_token_get_balance_multiple_tokens() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token = provider.retrieve_token(token_address).await.unwrap();
    let amount = U256::from(common::TEN_TOKENS);
    let balance = token.get_balance(amount);

    assert_eq!(balance.to_string(), "10.000000000000000000");
}

#[tokio::test]
async fn test_token_equality() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token1 = provider.retrieve_token(token_address).await.unwrap();
    let token2 = provider.retrieve_token(token_address).await.unwrap();

    assert_eq!(token1.address, token2.address);
    assert_eq!(token1.symbol, token2.symbol);
    assert_eq!(token1.decimals, token2.decimals);
}

#[tokio::test]
async fn test_token_new() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;

    let token = Token::new(token_address, "TEST".to_string(), 18);

    assert_eq!(token.address, token_address);
    assert_eq!(token.symbol, "TEST");
    assert_eq!(token.decimals, 18);
}

#[tokio::test]
async fn test_provider_balance_of_zero() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let balance = provider
        .balance_of(token_address, ANVIL_ADDRESS_0)
        .await
        .unwrap();

    assert_eq!(balance.to_string(), "0");
}

#[tokio::test]
async fn test_provider_balance_of_after_mint() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(ONE_TOKEN);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider();

    let balance = provider
        .balance_of(token_address, ANVIL_ADDRESS_0)
        .await
        .unwrap();

    assert_eq!(balance.to_string(), "1.000000000000000000");
}

#[tokio::test]
async fn test_provider_balance_of_multiple_tokens() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(common::TEN_TOKENS);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider();

    let balance = provider
        .balance_of(token_address, ANVIL_ADDRESS_0)
        .await
        .unwrap();

    assert_eq!(balance.to_string(), "10.000000000000000000");
}
