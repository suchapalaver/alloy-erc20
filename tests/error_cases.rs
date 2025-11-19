mod common;

use alloy::primitives::U256;
use alloy_erc20::LazyTokenSigner;
use common::{
    TestContext, ANVIL_ADDRESS_0, ANVIL_ADDRESS_1, ANVIL_ADDRESS_2, ONE_TOKEN, TEN_TOKENS,
};

// =============================================================================
// Insufficient Balance Tests
// =============================================================================

#[tokio::test]
#[should_panic(expected = "Insufficient balance")]
async fn test_transfer_insufficient_balance() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(ONE_TOKEN);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    let transfer_amount = U256::from(TEN_TOKENS);
    token
        .transfer(ANVIL_ADDRESS_1, transfer_amount)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_transfer_exact_balance_succeeds() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(ONE_TOKEN);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    let result = token
        .transfer(ANVIL_ADDRESS_1, mint_amount)
        .await
        .unwrap()
        .watch()
        .await;

    assert!(result.is_ok());

    let balance = token.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    assert_eq!(balance, U256::ZERO);
}

#[tokio::test]
async fn test_transfer_zero_amount() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(ONE_TOKEN);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    let result = token
        .transfer(ANVIL_ADDRESS_1, U256::ZERO)
        .await
        .unwrap()
        .watch()
        .await;

    assert!(result.is_ok());

    let balance = token.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    assert_eq!(balance, mint_amount);
}

// =============================================================================
// Insufficient Allowance Tests
// =============================================================================

#[tokio::test]
#[should_panic(expected = "Insufficient allowance")]
async fn test_transfer_from_insufficient_allowance() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(TEN_TOKENS);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;

    let provider_0 = ctx.create_provider_with_signer(0);
    let provider_1 = ctx.create_provider_with_signer(1);

    let token_0 = LazyTokenSigner::new(token_address, provider_0);
    let token_1 = LazyTokenSigner::new(token_address, provider_1);

    token_0
        .approve(ANVIL_ADDRESS_1, U256::from(ONE_TOKEN))
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    token_1
        .transfer_from(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1, U256::from(TEN_TOKENS))
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "Insufficient allowance")]
async fn test_transfer_from_zero_allowance() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(TEN_TOKENS);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;

    let provider_1 = ctx.create_provider_with_signer(1);
    let token_1 = LazyTokenSigner::new(token_address, provider_1);

    token_1
        .transfer_from(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1, U256::from(ONE_TOKEN))
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_transfer_from_exact_allowance_succeeds() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(TEN_TOKENS);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;

    let provider_0 = ctx.create_provider_with_signer(0);
    let provider_1 = ctx.create_provider_with_signer(1);

    let token_0 = LazyTokenSigner::new(token_address, provider_0);
    let token_1 = LazyTokenSigner::new(token_address, provider_1);

    let approval_amount = U256::from(ONE_TOKEN);
    token_0
        .approve(ANVIL_ADDRESS_1, approval_amount)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let result = token_1
        .transfer_from(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1, approval_amount)
        .await
        .unwrap()
        .watch()
        .await;

    assert!(result.is_ok());

    let remaining_allowance = token_0
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
        .await
        .unwrap();
    assert_eq!(remaining_allowance, U256::ZERO);
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[tokio::test]
async fn test_approve_zero_to_revoke() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    token
        .approve(ANVIL_ADDRESS_1, U256::from(ONE_TOKEN))
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let allowance = token
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
        .await
        .unwrap();
    assert_eq!(allowance, U256::from(ONE_TOKEN));

    token
        .approve(ANVIL_ADDRESS_1, U256::ZERO)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let allowance = token
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
        .await
        .unwrap();
    assert_eq!(allowance, U256::ZERO);
}

#[tokio::test]
async fn test_transfer_to_self() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(ONE_TOKEN);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    let result = token
        .transfer(ANVIL_ADDRESS_0, U256::from(ONE_TOKEN / 2))
        .await
        .unwrap()
        .watch()
        .await;

    assert!(result.is_ok());

    let balance = token.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    assert_eq!(balance, mint_amount);
}

#[tokio::test]
async fn test_approve_same_spender_multiple_times() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    for amount in [ONE_TOKEN, TEN_TOKENS, ONE_TOKEN / 2] {
        token
            .approve(ANVIL_ADDRESS_1, U256::from(amount))
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();

        let allowance = token
            .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
            .await
            .unwrap();
        assert_eq!(allowance, U256::from(amount));
    }
}

#[tokio::test]
async fn test_multiple_approvals_different_spenders() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    token
        .approve(ANVIL_ADDRESS_1, U256::from(ONE_TOKEN))
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    token
        .approve(ANVIL_ADDRESS_2, U256::from(TEN_TOKENS))
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let allowance_1 = token
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
        .await
        .unwrap();
    assert_eq!(allowance_1, U256::from(ONE_TOKEN));

    let allowance_2 = token
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_2)
        .await
        .unwrap();
    assert_eq!(allowance_2, U256::from(TEN_TOKENS));
}

#[tokio::test]
async fn test_total_supply_increases_with_mints() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider.clone());

    let initial_supply = token.total_supply().await.unwrap();
    assert_eq!(initial_supply, U256::ZERO);

    ctx.mint_tokens(token_address, ANVIL_ADDRESS_0, U256::from(ONE_TOKEN))
        .await;
    let supply_after_first = token.total_supply().await.unwrap();
    assert_eq!(supply_after_first, U256::from(ONE_TOKEN));

    ctx.mint_tokens(token_address, ANVIL_ADDRESS_1, U256::from(TEN_TOKENS))
        .await;
    let supply_after_second = token.total_supply().await.unwrap();
    assert_eq!(supply_after_second, U256::from(ONE_TOKEN + TEN_TOKENS));
}

#[tokio::test]
async fn test_transfer_does_not_affect_total_supply() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(TEN_TOKENS);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    let initial_supply = token.total_supply().await.unwrap();

    token
        .transfer(ANVIL_ADDRESS_1, U256::from(ONE_TOKEN))
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let supply_after_transfer = token.total_supply().await.unwrap();
    assert_eq!(initial_supply, supply_after_transfer);
}
