use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NFT {
    pub id: u64,
    pub owner: String,
    pub metadata: String, // URI to metadata
    pub price: Option<u64>,
}

pub struct NFTMarketplace {
    pub nfts: HashMap<u64, NFT>,
    pub next_id: u64,
}

impl NFTMarketplace {
    pub fn new() -> Self {
        Self {
            nfts: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn mint_nft(&mut self, owner: &str, metadata: &str) -> u64 {
        let nft = NFT {
            id: self.next_id,
            owner: owner.to_string(),
            metadata: metadata.to_string(),
            price: None,
        };

        self.nfts.insert(self.next_id, nft);
        self.next_id += 1;

        self.next_id - 1
    }

    pub fn list_nft_for_sale(&mut self, nft_id: u64, price: u64) {
        if let Some(nft) = self.nfts.get_mut(&nft_id) {
            nft.price = Some(price);
        }
    }

    pub fn buy_nft(&mut self, nft_id: u64, buyer: &str) -> bool {
        if let Some(nft) = self.nfts.get_mut(&nft_id) {
            if nft.price.is_some() {
                nft.owner = buyer.to_string();
                nft.price = None;
                return true;
            }
        }
        false
    }
}
