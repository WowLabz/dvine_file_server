use crate::services::file_service::MultipartHandler;
use reqwest::{header, Method};

pub struct Client {
    pub http_client: reqwest::Client,
    pub request_url: Option<String>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            request_url: None,
        }
    }
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_request_url(mut self, server_url: String) -> Self {
        self.request_url = Some(server_url);
        self
    }

    pub fn url(cid: String) -> String {
        // https://ipfs.io/ipfs/cid
        format!("https://ipfs.io/ipfs/{}", Self::rem_first_and_last(&cid))
    }

    fn rem_first_and_last(value: &str) -> &str {
        let mut chars = value.chars();
        chars.next();
        chars.next_back();
        chars.as_str()
    }

    pub async fn upload_nft(&self, multipart: MultipartHandler) -> Result<String, reqwest::Error> {
        dotenv::dotenv().ok();
        let auth_token = std::env::var("NFT_STORAGE_AUTH").expect("NFT_STORAGE_AUTH not set");

        let req = self
            .http_client
            .post(&self.request_url.clone().unwrap())
            .body(multipart.raw)
            .bearer_auth(auth_token)
            .send()
            .await?;

        let res: serde_json::Value = req.json().await?;
        println!("cid: {:#?}", res);
        let cid: String = res["value"]["cid"].to_string();
        let view_nft_url = Self::url(cid);
        println!("view_nft_url: {}", view_nft_url);

        Ok(view_nft_url)
    }
}
