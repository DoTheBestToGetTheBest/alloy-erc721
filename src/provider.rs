use crate::error::Erc721Error;
use crate::types::{Token, TokenBalance};
use alloy::{
    network::Network,
    primitives::{Address, U256},
    providers::Provider,
    sol,
    transports::Transport,
};
use Erc721Contract::{nameReturn, symbolReturn};

sol!(
    #[sol(rpc)]
    Erc721Contract,
    "abi/erc721.json"
);

pub trait Erc721Provider<T, N>: Provider<T, N> + Sized
where
    T: Transport + Clone,
    N: Network,
{
    async fn token_information(
        &self,
        address: Address,
        token_id: Option<U256>,
    ) -> Result<Token, Erc721Error> {
        let instance = Erc721Contract::Erc721ContractInstance::new(address, self);
        let symbol = instance
            .clone()
            .symbol()
            .call()
            .await
            .map_err(|e| Erc721Error::RpcError(e.to_string()))?;
        let name = instance
            .name()
            .call()
            .await
            .map_err(|e| Erc721Error::RpcError(e.to_string()))?;

        if let Some(t) = token_id {
            instance
                .tokenURI(t)
                .call()
                .await
                .map_err(|e| Erc721Error::NotFound(e.to_string()))?;
        };

        Ok(Token {
            symbol: symbol.to_string(),
            name: name.to_string(),
            address,
        })
    }

    async fn balance_of_token(
        &self,
        token: Address,
        address: Address,
    ) -> Result<TokenBalance, Erc721Error> {
        let instance = Erc721Contract::Erc721ContractInstance::new(token, self);
        let balance = instance
            .balanceOf(address)
            .call()
            .await
            .map_err(|e| Erc721Error::RpcError(e.to_string()))?;

        Ok(TokenBalance {
            address,
            token,
            balance: balance._0,
        })
    }

}
impl ToString for symbolReturn {
    fn to_string(&self) -> String {
        self._0.clone()
    }
}
impl ToString for nameReturn {
    fn to_string(&self) -> String {
        self._0.clone()
    }
}
impl<P, T, N> Erc721Provider<T, N> for P
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
}
