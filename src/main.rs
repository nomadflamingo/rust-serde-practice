use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use serde_yaml::to_string as to_yaml;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use toml::to_string as to_toml;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Debug {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

#[derive(Serialize, Deserialize, Debug)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

fn main() {
    let event = Event {
        name: "Event 1".to_string(),
        date: "2021-11-14".to_string(),
    };

    let json = serde_json::to_string(&event).unwrap();
    println!("\nJSON\n{}", json);

    let deserialized_event: Event = serde_json::from_str(&json).unwrap();
    println!("\nDeserialized JSON: \n{:?}\n", deserialized_event);

    let mut file = File::open("request.json").unwrap();
    let mut json_str = String::new();
    file.read_to_string(&mut json_str).unwrap();

    let request: Request = serde_json::from_str(&json_str).unwrap();
    println!("{:?}", request);

    let yaml_str = to_yaml(&request).unwrap();
    println!("\n{}", yaml_str);

    let toml_str = to_toml(&request).unwrap();
    println!("\n{}", toml_str);
}

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    name: String,
    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    date: String,
}

fn serialize_date<S: Serializer>(date: &str, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("Date: {}", date))
}

fn deserialize_date<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let data: &str = Deserialize::deserialize(deserializer)?;
    let res = data.replace("Date: ", "");
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut file = File::open("request.json").unwrap();
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();

        let request: Request = serde_json::from_str(&json_str).unwrap();

        assert_eq!(
            request.stream.user_id,
            Uuid::parse_str("8d234120-0bda-49b2-b7e0-fbd3912f6cbf").unwrap()
        );
        assert_eq!(request.debug.duration, Duration::from_millis(234));

        assert_eq!(
            request.stream.shard_url,
            Url::parse("https://n3.example.com/sapi").unwrap()
        );

        assert_eq!(request.gifts.len(), 2);
        assert_eq!(request.gifts[0].id, 1);
        assert_eq!(request.gifts[1].id, 2);
    }
}
