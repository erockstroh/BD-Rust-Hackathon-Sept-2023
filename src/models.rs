use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  pub user_id: String,
  pub email: String,
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub user_id: String,
    pub ticker: String,
    pub quantity: i64,
    pub limit_order: bool,
    pub limit_price: f64,
}
