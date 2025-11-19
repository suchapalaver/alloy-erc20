mod common;

use alloy::primitives::U256;
use alloy_erc20::{LazyToken, LazyTokenSigner};
use common::{
    TestContext, ANVIL_ADDRESS_0, ANVIL_ADDRESS_1, ANVIL_ADDRESS_2, HUNDRED_TOKENS, ONE_TOKEN,
    TEN_TOKENS,
};

// =============================================================================
// LazyToken Read Operations Tests
// =============================================================================

#[tokio::test]
async fn test_lazy_token_read_operations() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider();

    let token = LazyToken::new(token_address, provider);

    let name = token.name().await.unwrap();
    assert_eq!(name, "Test Token");

    let symbol = token.symbol().await.unwrap();
    assert_eq!(symbol, "TEST");

    let decimals = token.decimals().await.unwrap();
    assert_eq!(*decimals, 18);

    let total_supply = token.total_supply().await.unwrap();
    assert_eq!(total_supply, U256::ZERO);

    let balance = token.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    assert_eq!(balance, U256::ZERO);

    let allowance = token
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
        .await
        .unwrap();
    assert_eq!(allowance, U256::ZERO);
}

#[tokio::test]
async fn test_lazy_token_get_balance() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(ONE_TOKEN);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider();

    let token = LazyToken::new(token_address, provider);

    let balance_raw = token.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    let balance_decimal = token.get_balance(balance_raw).await.unwrap();

    assert_eq!(balance_decimal.to_string(), "1.000000000000000000");
}

// =============================================================================
// LazyTokenSigner Transfer Tests
// =============================================================================

#[tokio::test]
async fn test_lazy_token_signer_transfer() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(ONE_TOKEN);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    let initial_balance = token.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    assert_eq!(initial_balance, mint_amount);

    let recipient_initial = token.balance_of(ANVIL_ADDRESS_1).await.unwrap();
    assert_eq!(recipient_initial, U256::ZERO);

    let transfer_amount = U256::from(ONE_TOKEN / 2);
    let _tx_hash = token
        .transfer(ANVIL_ADDRESS_1, transfer_amount)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let sender_balance = token.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    assert_eq!(sender_balance, mint_amount - transfer_amount);

    let recipient_balance = token.balance_of(ANVIL_ADDRESS_1).await.unwrap();
    assert_eq!(recipient_balance, transfer_amount);
}

#[tokio::test]
async fn test_lazy_token_signer_transfer_full_balance() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(TEN_TOKENS);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    let _tx_hash = token
        .transfer(ANVIL_ADDRESS_1, mint_amount)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let sender_balance = token.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    assert_eq!(sender_balance, U256::ZERO);

    let recipient_balance = token.balance_of(ANVIL_ADDRESS_1).await.unwrap();
    assert_eq!(recipient_balance, mint_amount);
}

#[tokio::test]
async fn test_lazy_token_signer_multiple_transfers() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(HUNDRED_TOKENS);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    let transfer_amount = U256::from(TEN_TOKENS);

    for _ in 0..3 {
        token
            .transfer(ANVIL_ADDRESS_1, transfer_amount)
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();
    }

    let sender_balance = token.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    assert_eq!(
        sender_balance,
        mint_amount - (transfer_amount * U256::from(3))
    );

    let recipient_balance = token.balance_of(ANVIL_ADDRESS_1).await.unwrap();
    assert_eq!(recipient_balance, transfer_amount * U256::from(3));
}

// =============================================================================
// LazyTokenSigner Approval Tests
// =============================================================================

#[tokio::test]
async fn test_lazy_token_signer_approve() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    let initial_allowance = token
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
        .await
        .unwrap();
    assert_eq!(initial_allowance, U256::ZERO);

    let approval_amount = U256::from(ONE_TOKEN);
    let _tx_hash = token
        .approve(ANVIL_ADDRESS_1, approval_amount)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let allowance = token
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
        .await
        .unwrap();
    assert_eq!(allowance, approval_amount);
}

#[tokio::test]
async fn test_lazy_token_signer_approve_update() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    let first_approval = U256::from(ONE_TOKEN);
    token
        .approve(ANVIL_ADDRESS_1, first_approval)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let second_approval = U256::from(TEN_TOKENS);
    token
        .approve(ANVIL_ADDRESS_1, second_approval)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let allowance = token
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
        .await
        .unwrap();
    assert_eq!(allowance, second_approval);
}

#[tokio::test]
async fn test_lazy_token_signer_approve_zero() {
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

// =============================================================================
// LazyTokenSigner TransferFrom Tests
// =============================================================================

#[tokio::test]
async fn test_lazy_token_signer_transfer_from() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(TEN_TOKENS);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;

    let provider_0 = ctx.create_provider_with_signer(0);
    let provider_1 = ctx.create_provider_with_signer(1);

    let token_0 = LazyTokenSigner::new(token_address, provider_0);
    let token_1 = LazyTokenSigner::new(token_address, provider_1);

    let approval_amount = U256::from(TEN_TOKENS / 2);
    token_0
        .approve(ANVIL_ADDRESS_1, approval_amount)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let transfer_amount = U256::from(TEN_TOKENS / 5);
    token_1
        .transfer_from(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1, transfer_amount)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let account_0_balance = token_0.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    assert_eq!(account_0_balance, mint_amount - transfer_amount);

    let account_1_balance = token_0.balance_of(ANVIL_ADDRESS_1).await.unwrap();
    assert_eq!(account_1_balance, transfer_amount);

    let remaining_allowance = token_0
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
        .await
        .unwrap();
    assert_eq!(remaining_allowance, approval_amount - transfer_amount);
}

#[tokio::test]
async fn test_lazy_token_signer_transfer_from_to_third_party() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(HUNDRED_TOKENS);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;

    let provider_0 = ctx.create_provider_with_signer(0);
    let provider_1 = ctx.create_provider_with_signer(1);

    let token_0 = LazyTokenSigner::new(token_address, provider_0);
    let token_1 = LazyTokenSigner::new(token_address, provider_1);

    let approval_amount = U256::from(TEN_TOKENS);
    token_0
        .approve(ANVIL_ADDRESS_1, approval_amount)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let transfer_amount = U256::from(TEN_TOKENS);
    token_1
        .transfer_from(ANVIL_ADDRESS_0, ANVIL_ADDRESS_2, transfer_amount)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let account_0_balance = token_0.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    assert_eq!(account_0_balance, mint_amount - transfer_amount);

    let account_1_balance = token_0.balance_of(ANVIL_ADDRESS_1).await.unwrap();
    assert_eq!(account_1_balance, U256::ZERO);

    let account_2_balance = token_0.balance_of(ANVIL_ADDRESS_2).await.unwrap();
    assert_eq!(account_2_balance, transfer_amount);

    let remaining_allowance = token_0
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
        .await
        .unwrap();
    assert_eq!(remaining_allowance, U256::ZERO);
}

#[tokio::test]
async fn test_lazy_token_signer_transfer_from_full_allowance() {
    let ctx = TestContext::new().await;
    let mint_amount = U256::from(TEN_TOKENS);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, mint_amount).await;

    let provider_0 = ctx.create_provider_with_signer(0);
    let provider_1 = ctx.create_provider_with_signer(1);

    let token_0 = LazyTokenSigner::new(token_address, provider_0);
    let token_1 = LazyTokenSigner::new(token_address, provider_1);

    let approval_amount = U256::from(TEN_TOKENS);
    token_0
        .approve(ANVIL_ADDRESS_1, approval_amount)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    token_1
        .transfer_from(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1, approval_amount)
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let account_0_balance = token_0.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    assert_eq!(account_0_balance, U256::ZERO);

    let account_1_balance = token_0.balance_of(ANVIL_ADDRESS_1).await.unwrap();
    assert_eq!(account_1_balance, approval_amount);

    let remaining_allowance = token_0
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
        .await
        .unwrap();
    assert_eq!(remaining_allowance, U256::ZERO);
}

// =============================================================================
// LazyTokenSigner Combined Read and Write Operations Tests
// =============================================================================

#[tokio::test]
async fn test_lazy_token_signer_all_read_operations() {
    let ctx = TestContext::new().await;
    let token_address = ctx.deploy_token().await;
    let provider = ctx.create_provider_with_signer(0);

    let token = LazyTokenSigner::new(token_address, provider);

    let name = token.name().await.unwrap();
    assert_eq!(name, "Test Token");

    let symbol = token.symbol().await.unwrap();
    assert_eq!(symbol, "TEST");

    let decimals = token.decimals().await.unwrap();
    assert_eq!(*decimals, 18);

    let total_supply = token.total_supply().await.unwrap();
    assert_eq!(total_supply, U256::ZERO);

    let balance = token.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    assert_eq!(balance, U256::ZERO);

    let allowance = token
        .allowance(ANVIL_ADDRESS_0, ANVIL_ADDRESS_1)
        .await
        .unwrap();
    assert_eq!(allowance, U256::ZERO);
}

#[tokio::test]
async fn test_lazy_token_signer_complex_workflow() {
    let ctx = TestContext::new().await;
    let initial_mint = U256::from(HUNDRED_TOKENS);
    let token_address = ctx.deploy_and_mint(ANVIL_ADDRESS_0, initial_mint).await;

    let provider_0 = ctx.create_provider_with_signer(0);
    let provider_1 = ctx.create_provider_with_signer(1);

    let token_0 = LazyTokenSigner::new(token_address, provider_0);
    let token_1 = LazyTokenSigner::new(token_address, provider_1);

    token_0
        .transfer(ANVIL_ADDRESS_1, U256::from(TEN_TOKENS))
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    token_0
        .approve(ANVIL_ADDRESS_1, U256::from(TEN_TOKENS))
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    token_1
        .transfer(ANVIL_ADDRESS_2, U256::from(TEN_TOKENS / 2))
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    token_1
        .transfer_from(ANVIL_ADDRESS_0, ANVIL_ADDRESS_2, U256::from(TEN_TOKENS / 2))
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let balance_0 = token_0.balance_of(ANVIL_ADDRESS_0).await.unwrap();
    let balance_1 = token_0.balance_of(ANVIL_ADDRESS_1).await.unwrap();
    let balance_2 = token_0.balance_of(ANVIL_ADDRESS_2).await.unwrap();

    let total_supply = token_0.total_supply().await.unwrap();

    assert_eq!(balance_0 + balance_1 + balance_2, total_supply);
    assert_eq!(total_supply, initial_mint);
}
