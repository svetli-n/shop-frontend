use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct LineItem {
    pub id: String,
    pub qty: u32,
    pub price: u32,
}

pub struct Basket {
    pub items: BTreeMap<String, LineItem>,
    pub total_price: u32,
}

impl Basket {

    pub fn new() -> Self {
        Basket {
            items: BTreeMap::new(),
            total_price: 0,
        }
    }

    pub fn total(&self) -> u32 {
        self.items.iter().map(|item| item.1.price*item.1.qty).sum()
    }

    pub fn add(&mut self, item: BTreeMap<String, String>) {
        let id = &item["id"];
        if self.items.contains_key(id) {
            self.items.get_mut(id).unwrap().qty += 1;
            log::info!("nutt");
        } else {
            let price = item.get("price").unwrap().split(".").collect::<Vec<_>>()[0].parse::<u32>().unwrap();
            let line_item = LineItem {
                id: id.clone(),
                qty: 1,
                price,
            };
            self.items.insert(id.clone(), line_item);
        }
    }

    pub fn remove(&mut self, id: String) {
        self.items.remove(id.as_str()).unwrap();
    }
}
