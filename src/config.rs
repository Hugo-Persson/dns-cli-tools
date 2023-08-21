use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub records: Vec<Record>,
    pub domain: String,
    pub api_key: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub name: String,
    pub record_type: RecordType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RecordType {
    A,
    AAA,
}

impl RecordType {
    pub fn to_string(&self) -> String {
        match self {
            RecordType::A => "A".to_string(),
            RecordType::AAA => "AAAA".to_string(),
        }
    }
}
