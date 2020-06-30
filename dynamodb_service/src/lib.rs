use failure::Error;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDbClient, DynamoDb, PutItemInput};
use std::collections::HashMap;
use std::env;

pub struct Client(DynamoDbClient);

impl Client {
    pub fn new() -> Self {
        Self(DynamoDbClient::new(Region::ApNortheast1))
    }

    #[tokio::main]
    pub async fn put(&self, bucket: &str, key: &str, is_thumb: bool)
        -> Result<(), Error>
    {
        let table_name = env::var("IMAGE_METADATA_TABLE").unwrap();
        let data = ImageMetaData::new(bucket, key, is_thumb);
        let input = PutItemInput {
            table_name,
            item: data.to_dynamodb_item(),
            ..Default::default()
        };
        self.0.put_item(input).await.unwrap();
        Ok(())
    }
}

#[derive(Debug)]
struct ImageMetaData {
    image_id: String,
    bucket: String,
    key: String,
    thumbnails: Vec<String>,
    is_thumb: bool,
}

impl ImageMetaData {
    fn new(bucket: &str, key: &str, is_thumb: bool) -> Self {
        Self {
            image_id: get_image_id(key, is_thumb),
            bucket: bucket.to_owned(),
            key: key.to_owned(),
            thumbnails: Vec::new(),
            is_thumb,
        }
    }

    fn to_dynamodb_item(&self) -> HashMap<String, AttributeValue> {
        let mut item: HashMap<String, AttributeValue> = HashMap::new();
        item.insert(
            String::from("ImageId"),
            get_s_attr(&self.image_id),
        );
        item.insert(
            String::from("Bucket"),
            get_s_attr(&self.bucket),
        );
        item.insert(
            String::from("Key"),
            get_s_attr(&self.key),
        );

        if !&self.thumbnails.is_empty() {
            item.insert(
                String::from("Thumbnails"),
                get_ss_attr(&self.thumbnails),
            );
        }

        item.insert(
            String::from("IsThumb"),
            get_bool_attr(self.is_thumb),
        );
        item
    }
}

fn get_image_id(key: &str, is_thumb: bool) -> String {
    let texts: Vec<&str> = key.split('/').collect();
    let mut filename = String::from(texts[1]);

    if is_thumb {
        filename = filename + "_thumbnail";
    }

    filename
}

fn get_s_attr(val: &str) -> AttributeValue {
    AttributeValue {
        s: Some(val.to_owned()),
        ..Default::default()
    }
}

fn get_ss_attr(val: &Vec<String>) -> AttributeValue {
    AttributeValue {
        ss: Some(val.to_owned()),
        ..Default::default()
    }
}

fn get_bool_attr(val: bool) -> AttributeValue {
    AttributeValue {
        bool: Some(val),
        ..Default::default()
    }
}
