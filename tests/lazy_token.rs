mod common;

use alloy::primitives::U256;
use alloy_erc20::LazyToken;
use common::{TestContext, ANVIL_ADDRESS_0, ANVIL_ADDRESS_1, ONE_TOKEN};

#[tokio::test]
async fn test_lazy_token_name() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token = LazyToken::new(token_address, provider);
    let name = token.name().await.unwrap();

    assert_eq!(name, "Test Token");
}

#[tokio::test]
async fn test_lazy_token_symbol() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token = LazyToken::new(token_address, provider);
    let symbol = token.symbol().await.unwrap();

    assert_eq!(symbol, "TEST");
}

#[tokio::test]
async fn test_lazy_token_decimals() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token = LazyToken::new(token_address, provider);
    let decimals = token.decimals().await.unwrap();

    assert_eq!(*decimals, 18);
}

#[tokio::test]
async fn test_lazy_token_total_supply_initially_zero() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token = LazyToken::new(token_address, provider);
    let total_supply = token.total_supply().await.unwrap();

    assert_eq!(total_supply, U256::ZERO);
}

#[tokio::test]
async fn test_lazy_token_total_supply_after_mint() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(ONE_TOKEN);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider();

    let token = LazyToken::new(token_address, provider);
    let total_supply = token.total_supply().await.unwrap();

    assert_eq!(total_supply, mint_amount);
}

#[tokio::test]
async fn test_lazy_token_balance_of_zero() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token = LazyToken::new(token_address, provider);
    let balance = token.balance_of(ANVIL_ADDRESS_0).await.unwrap();

    assert_eq!(balance, U256::ZERO);
}

#[tokio::test]
async fn test_lazy_token_balance_of_after_mint() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(ONE_TOKEN);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider();

    let token = LazyToken::new(token_address, provider);
    let balance = token.balance_of(ANVIL_ADDRESS_0).await.unwrap();

    assert_eq!(balance, mint_amount);
}

#[tokio::test]
async fn test_lazy_token_get_balance_conversion() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(ONE_TOKEN);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider();

    let token = LazyToken::new(token_address, provider);
    let balance_raw = token.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    let balance_decimal = token.get_balance(balance_raw).await.unwrap();

    assert_eq!(balance_decimal.to_string(), "1.000000000000000000");
}

#[tokio::test]
async fn test_lazy_token_allowance_initially_zero() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token = LazyToken::new(token_address, provider);
    let allowance = token
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
        .await
        .unwrap();

    assert_eq!(allowance, U256::ZERO);
}

#[tokio::test]
async fn test_lazy_token_caches_metadata() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token = LazyToken::new(token_address, provider);

    let name1 = token.name().await.unwrap();
    let name2 = token.name().await.unwrap();
    assert_eq!(name1, name2);
    assert_eq!(name1, "Test Token");

    let symbol1 = token.symbol().await.unwrap();
    let symbol2 = token.symbol().await.unwrap();
    assert_eq!(symbol1, symbol2);
    assert_eq!(symbol1, "TEST");

    let decimals1 = token.decimals().await.unwrap();
    let decimals2 = token.decimals().await.unwrap();
    assert_eq!(decimals1, decimals2);
    assert_eq!(*decimals1, 18);
}
