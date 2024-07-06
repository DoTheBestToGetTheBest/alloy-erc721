use alloy::providers::ProviderBuilder;
use alloy_erc721::Erc721Provider;
#[tokio::main]
async fn main() {
    let provider = ProviderBuilder::new().on_http("https://eth.rpc.blxrbdn.com".parse().unwrap());
    let balance = provider
        .balance_of_token(
            "0x269616D549D7e8Eaa82DFb17028d0B212D11232A"
                .parse()
                .unwrap(),
            "0x27B0f73721DA882fAAe00B6e43512BD9eC74ECFA"
                .parse()
                .unwrap(),
        )
        .await;

    println!("{:?}", balance);
}
