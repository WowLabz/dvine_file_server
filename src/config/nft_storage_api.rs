use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct NftStorageApiConfig {
    pub server: String,
    pub upload_nft: String,
}

impl Default for NftStorageApiConfig {
    fn default() -> Self {
        Self {
            server: String::from("https://api.nft.storage"),
            upload_nft: String::from("/upload"),
        }
    }
} 