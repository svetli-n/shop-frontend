use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: u32,
    pub description: String,
    pub price: u32,
}
