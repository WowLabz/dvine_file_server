use crate::services::file_service::MultipartHandler;
use rusoto_credential::ProvideAwsCredentials;
use rusoto_core::{Region, RusotoError, credential::{AwsCredentials, StaticProvider}};
use rusoto_s3::{S3Client, PutObjectRequest, S3, PutObjectError};

pub struct Client {
    region: Region,
    s3: S3Client,
    bucket_name: String,
}

impl Client {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let region = Region::ApSouth1;

        println!(
            "Bucket Name: {}",
            std::env::var("AWS_S3_BUCKET_NAME").expect("AWS_S3_BUCKET_NAME not set")
        );

        println!("Region Name: {}", std::env::var("AWS_REGION").expect("AWS_REGION not set"));
        
        let aws_credential = AwsCredentials::new(
            std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not set"),
            std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not set"),
            None,
            None
        );

        Self {
            region: region.to_owned(),
            s3: S3Client::new_with(
                rusoto_core::request::HttpClient::new().expect("Failed to creat HTTP client"),
                StaticProvider::from(aws_credential),
                region
            ),
            bucket_name: std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
        }
    }

    pub fn url(&self, key: &str) -> String {
        dotenv::dotenv().ok();
        let url_str = format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            std::env::var("AWS_S3_BUCKET_NAME").expect("aws env config not set"),
            std::env::var("AWS_REGION").expect("aws env config not set"),
            key
        );
        println!("url_str: {:#?}", &url_str);
        url_str
    }

    pub async fn put_object(&self, multipart: MultipartHandler) -> Result<String, RusotoError<PutObjectError>> {
        let key = format!("vines/{}", multipart.file_name.to_owned());
         let put_request = PutObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: key.clone(),
            body: Some(multipart.raw.into()),
            ..Default::default()
        };

        self
            .s3
            .put_object(put_request)
            .await?;

        Ok(self.url(&key))
    }
}