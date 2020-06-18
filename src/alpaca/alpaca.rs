use std::ops::Deref;

use std::thread;

use crossbeam::queue::ArrayQueue;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use serde_json::{Map, Value};

use crate::alpaca::account::APCA_SECRET_KEY;
use crate::alpaca::account::{Account, APCA_API_KEY};
use crate::alpaca::positions::{Portfolio, Position};
use crate::alpaca::tick_data::TickData;
use crate::appconfig::{AlpacaConfig, AppConfig};
use std::sync::{Arc, RwLock};
use std::time::Duration;

impl AlpacaConfig {
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

        headers.insert(
            HeaderName::from_static(APCA_API_KEY),
            HeaderValue::from_str(self.key.as_str()).unwrap(),
        );

        headers.insert(
            HeaderName::from_static(APCA_SECRET_KEY),
            HeaderValue::from_str(self.secret.as_str()).unwrap(),
        );
        headers
    }
}

impl Account {
    pub fn read_alpaca_data(account_rw_lock: Arc<RwLock<Account>>, app_config: AppConfig) {
        let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
            .default_headers(app_config.alpaca_config.headers())
            .build()
            .unwrap();

        let url = format!("{}{}", app_config.alpaca_config.url, "/v2/account");

        thread::spawn(move || {
            let res = client.get(&url).send().unwrap();
            let account: Account = res.json().unwrap();
            let mut account_guard = account_rw_lock.write().unwrap();
            *account_guard = account;
        });
    }
}

impl Portfolio {
    pub fn read_alpaca_data(queue: Arc<ArrayQueue<Portfolio>>, app_config: AppConfig) {
        let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
            .default_headers(app_config.alpaca_config.headers())
            .build()
            .unwrap();

        let url = format!("{}{}", app_config.alpaca_config.url, "/v2/account");

        thread::spawn(move || {
            let res = client.get(&url).send().unwrap();

            let portfolio_json: Vec<Map<String, Value>> = res.json().unwrap();
            println!("Portfolio data: {:?}", portfolio_json);

            let portfolio: Portfolio = Portfolio::from_alpaca_json_map(portfolio_json);

            queue.push(portfolio);
        });
    }

    pub fn from_alpaca_json_map(vecs: Vec<serde_json::Map<String, Value>>) -> Portfolio {
        let positions = vecs
            .iter()
            .map(move |m| Position::from_alpaca_json_map(m.deref().clone()))
            .collect();
        Portfolio { positions }
    }
}

impl Position {
    pub fn from_alpaca_json_map(map: serde_json::Map<String, Value>) -> Position {
        Position {
            symbol: map.get("symbol").unwrap().to_string(),
            asset_class: map.get("asset_class").unwrap().to_string(),
            avg_entry_price: map
                .get("avg_entry_price")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            qty: map
                .get("qty")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            side: map
                .get("side")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            market_value: map
                .get("market_value")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            cost_basis: map
                .get("cost_basis")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            unrealized_pl: map
                .get("unrealized_pl")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            unrealized_plpc: map
                .get("unrealized_plpc")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            unrealized_intraday_pl: map
                .get("unrealized_intraday_pl")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            unrealized_intraday_plpc: map
                .get("unrealized_intraday_plpc")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            current_price: map
                .get("current_price")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            lastday_price: map
                .get("lastday_price")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            change_today: map
                .get("change_today")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
        }
    }

    pub fn from_alpaca_json(str: &str) -> Position {
        let map = serde_json::from_str(str).unwrap();
        Position::from_alpaca_json_map(map)
    }
}

#[test]
#[ignore]
fn test_deserialize_position() {
    let str = r##"
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
"##;
    let value: f64 = "100.0".parse().unwrap();
    println!("{:?}", value);

    //let map2 = serde_json::Map<String, Value>();

    //map2.insert("test", Value::from("100.0"));
    //println!("{:?}", position);

    let map: Map<String, Value> = serde_json::from_str(str).unwrap();
    let position = Position::from_alpaca_json_map(map);
    println!("{:?}", position);
}

#[test]
fn test_deserialize_portfolio() {
    let str = r##"
    [
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
]
"##;

    let positions: Vec<Map<String, Value>> = serde_json::from_str(str).unwrap();
    Portfolio::from_alpaca_json_map(positions.clone());
    let portfolio = Portfolio::from_alpaca_json_map(positions);
    println!("positions after deserialization {:?}", portfolio);
}
