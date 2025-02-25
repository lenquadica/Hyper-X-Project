use web3::transports::Http;
use web3::Web3;
use solana_client::rpc_client::RpcClient;

pub struct CrossChainBridge {
    pub eth_client: Web3<Http>,
    pub solana_client: RpcClient,
}

impl 
