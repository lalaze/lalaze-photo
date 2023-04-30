use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub name: String,
    pub location: String,
    pub tag: Option<String>,
}