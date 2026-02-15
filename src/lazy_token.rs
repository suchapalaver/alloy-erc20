use crate::provider::Erc20Contract;
use alloy::{
    contract::Error,
    network::Network,
    primitives::{Address, U256},
    providers::Provider,
};
use async_once_cell::OnceCell;
use bigdecimal::{
    num_bigint::{BigInt, Sign},
    BigDecimal,
};
use futures::TryFutureExt;
use std::{
    fmt::Debug,
    future::{ready, IntoFuture},
};

#[derive(Debug)]
/// A token with an embedded contract instance that lazily query the
/// blockchain.
pub struct LazyToken<P, N> {
    name: OnceCell<String>,
    symbol: OnceCell<String>,
    decimals: OnceCell<u8>,
    instance: Erc20Contract::Erc20ContractInstance<P, N>,
}

impl<P, N> LazyToken<P, N>
where
    P: Provider<N>,
    N: Network,
{
    /// Creates a new [`LazyToken`].
    pub const fn new(address: Address, provider: P) -> Self {
        Self {
            name: OnceCell::new(),
            symbol: OnceCell::new(),
            decimals: OnceCell::new(),
            instance: Erc20Contract::new(address, provider),
        }
    }

    /// Returns the token contract address.
    pub const fn address(&self) -> &Address {
        self.instance.address()
    }

    /// Returns the name of the token.
    pub async fn name(&self) -> Result<&String, Error> {
        self.name
            .get_or_try_init(
                self.instance
                    .name()
                    .call()
                    .into_future()
                    .and_then(|r| ready(Ok(r))),
            )
            .await
    }

    /// Returns the symbol of the token.
    pub async fn symbol(&self) -> Result<&String, Error> {
        self.symbol
            .get_or_try_init(
                self.instance
                    .symbol()
                    .call()
                    .into_future()
                    .and_then(|r| ready(Ok(r))),
            )
            .await
    }

    /// Returns the decimals places of the token.
    pub async fn decimals(&self) -> Result<&u8, Error> {
        self.decimals
            .get_or_try_init(
                self.instance
                    .decimals()
                    .call()
                    .into_future()
                    .and_then(|r| ready(Ok(r))),
            )
            .await
    }

    /// Returns the amount of tokens in existence.
    pub async fn total_supply(&self) -> Result<U256, Error> {
        self.instance
            .totalSupply()
            .call()
            .into_future()
            .and_then(|r| ready(Ok(r)))
            .await
    }

    /// Returns the value of tokens owned by `account`.
    pub async fn balance_of(&self, account: Address) -> Result<U256, Error> {
        self.instance
            .balanceOf(account)
            .call()
            .into_future()
            .and_then(|r| ready(Ok(r)))
            .await
    }

    /// Returns the remaining number of tokens that `spender` will be
    /// allowed to spend on behalf of `owner`.
    pub async fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Error> {
        self.instance
            .allowance(owner, spender)
            .call()
            .into_future()
            .and_then(|r| ready(Ok(r)))
            .await
    }

    /// Gets the token balance as a [`BigDecimal`]
    pub async fn get_balance(&self, amount: U256) -> Result<BigDecimal, Error> {
        let decimals = self.decimals().await?;

        let balance = BigDecimal::from((
            BigInt::from_bytes_be(Sign::Plus, &amount.to_be_bytes::<{ U256::BYTES }>()),
            *decimals as i64,
        ));

        Ok(balance)
    }
}

#[derive(Debug)]
/// A token with write operation support using a signer-capable provider.
///
/// This struct wraps [`LazyToken`] and adds methods for write operations like
/// `transfer`, `approve`, and `transferFrom`. The provider must be capable of
/// signing transactions.
///
/// # Examples
///
/// ```no_run
/// use alloy::network::EthereumWallet;
/// use alloy::primitives::{address, U256};
/// use alloy::providers::ProviderBuilder;
/// use alloy::signers::local::PrivateKeySigner;
/// use alloy_erc20::LazyTokenSigner;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let signer = PrivateKeySigner::random();
/// let wallet = EthereumWallet::from(signer);
/// let provider = ProviderBuilder::new()
///     .wallet(wallet)
///     .connect_http("https://eth.llamarpc.com".parse()?);
///
/// let token = LazyTokenSigner::new(
///     address!("6B175474E89094C44Da98b954EedeAC495271d0F"), // DAI
///     provider
/// );
///
/// // Transfer tokens
/// let tx = token.transfer(
///     address!("70997970C51812dc3A010C7d01b50e0d17dc79C8"),
///     U256::from(1000000000000000000u64) // 1 token with 18 decimals
/// ).await?;
///
/// // Wait for confirmation
/// let tx_hash = tx.watch().await?;
/// println!("Transfer confirmed: {}", tx_hash);
/// # Ok(())
/// # }
/// ```
pub struct LazyTokenSigner<P, N>
where
    P: Provider<N>,
    N: Network,
{
    token: LazyToken<P, N>,
    instance: Erc20Contract::Erc20ContractInstance<P, N>,
}

impl<P, N> LazyTokenSigner<P, N>
where
    P: Provider<N> + Clone,
    N: Network,
{
    /// Creates a new [`LazyTokenSigner`].
    ///
    /// # Arguments
    ///
    /// * `address` - The ERC-20 token contract address
    /// * `provider` - A provider capable of signing transactions
    pub fn new(address: Address, provider: P) -> Self {
        Self {
            token: LazyToken::new(address, provider.clone()),
            instance: Erc20Contract::new(address, provider),
        }
    }

    /// Returns the token contract address.
    pub const fn address(&self) -> &Address {
        self.token.address()
    }

    /// Returns the name of the token.
    pub async fn name(&self) -> Result<&String, Error> {
        self.token.name().await
    }

    /// Returns the symbol of the token.
    pub async fn symbol(&self) -> Result<&String, Error> {
        self.token.symbol().await
    }

    /// Returns the decimals places of the token.
    pub async fn decimals(&self) -> Result<&u8, Error> {
        self.token.decimals().await
    }

    /// Returns the amount of tokens in existence.
    pub async fn total_supply(&self) -> Result<U256, Error> {
        self.token.total_supply().await
    }

    /// Returns the value of tokens owned by `account`.
    pub async fn balance_of(&self, account: Address) -> Result<U256, Error> {
        self.token.balance_of(account).await
    }

    /// Returns the remaining number of tokens that `spender` will be
    /// allowed to spend on behalf of `owner`.
    pub async fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Error> {
        self.token.allowance(owner, spender).await
    }

    /// Gets the token balance as a [`BigDecimal`]
    pub async fn get_balance(&self, amount: U256) -> Result<BigDecimal, Error> {
        self.token.get_balance(amount).await
    }

    /// Transfers `amount` tokens to `to`.
    ///
    /// This sends the transaction to the network. Use the returned
    /// pending transaction builder to wait for confirmation:
    ///
    /// ```no_run
    /// # use alloy::primitives::{address, U256};
    /// # use alloy_erc20::LazyTokenSigner;
    /// # async fn example(token: LazyTokenSigner<impl alloy::providers::Provider<alloy::network::Ethereum> + Clone, alloy::network::Ethereum>) -> Result<(), Box<dyn std::error::Error>> {
    /// let pending_tx = token.transfer(
    ///     address!("70997970C51812dc3A010C7d01b50e0d17dc79C8"),
    ///     U256::from(1000000000000000000u64)
    /// ).await?;
    ///
    /// let receipt = pending_tx.watch().await?;
    /// println!("Transfer confirmed!");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the transaction fails to send or if there's
    /// insufficient balance/gas.
    pub async fn transfer(
        &self,
        to: Address,
        amount: U256,
    ) -> Result<alloy::providers::PendingTransactionBuilder<N>, Error> {
        self.instance.transfer(to, amount).send().await
    }

    /// Approves `spender` to transfer up to `amount` tokens on behalf of the caller.
    ///
    /// This sends the transaction to the network. Use the returned
    /// pending transaction builder to wait for confirmation.
    ///
    /// # Errors
    ///
    /// Returns an error if the transaction fails to send.
    pub async fn approve(
        &self,
        spender: Address,
        amount: U256,
    ) -> Result<alloy::providers::PendingTransactionBuilder<N>, Error> {
        self.instance.approve(spender, amount).send().await
    }

    /// Transfers `amount` tokens from `from` to `to` using the allowance mechanism.
    ///
    /// The caller must have sufficient allowance from `from` to transfer the tokens.
    /// This sends the transaction to the network. Use the returned
    /// pending transaction builder to wait for confirmation.
    ///
    /// # Errors
    ///
    /// Returns an error if the transaction fails to send or if there's
    /// insufficient allowance.
    pub async fn transfer_from(
        &self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<alloy::providers::PendingTransactionBuilder<N>, Error> {
        self.instance.transferFrom(from, to, amount).send().await
    }
}
