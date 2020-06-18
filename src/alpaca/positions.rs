use serde::Deserialize;
use serde::Serialize;
//use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Portfolio {
    pub(crate) positions: Vec<Position>,
}

impl Portfolio {
    pub fn to_json_str(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

/*
{
  "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
  "symbol": "AAPL",
  "exchange": "NASDAQ",
  "asset_class": "us_equity",
  "avg_entry_price": "100.0",
  "qty": "5",
  "side": "long",
  "market_value": "600.0",
  "cost_basis": "500.0",
  "unrealized_pl": "100.0",
  "unrealized_plpc": "0.20",
  "unrealized_intraday_pl": "10.0",
  "unrealized_intraday_plpc": "0.0084",
  "current_price": "120.0",
  "lastday_price": "119.0",
  "change_today": "0.0084"
}
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub asset_class: String,
    pub(crate) avg_entry_price: f64,
    pub qty: u64,
    pub side: String,
    pub market_value: f64,
    pub cost_basis: f64,
    pub unrealized_pl: f64,
    pub unrealized_plpc: f64,
    pub unrealized_intraday_pl: f64,
    pub unrealized_intraday_plpc: f64,
    pub current_price: f64,
    pub lastday_price: f64,
    pub change_today: f64,
}

impl Position {}
