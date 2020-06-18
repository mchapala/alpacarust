use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::mpsc::{Receiver, Sender};
//use std::sync::{Arc, RwLock};
use std::thread;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use serde::Deserialize;
use serde::Serialize;

pub(crate) const APCA_API_KEY: &str = "apca-api-key-id";
pub(crate) const APCA_SECRET_KEY: &str = "apca-api-secret-key";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Account {
    account_number: String,
    buying_power: String,
    regt_buying_power: String,
    daytrading_buying_power: String,
    cash: String,
    portfolio_value: String,
    pattern_day_trader: bool,
    shorting_enabled: bool,
    equity: String,
    long_market_value: String,
    short_market_value: String,
    initial_margin: String,
    sma: String,
    daytrade_count: u8,
}

impl Account {
    pub fn to_json_str(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn from_json_str(str: &str) -> Account {
        serde_json::from_str(str).unwrap()
    }

    pub fn write_to_file(&self, mut f: File) -> std::result::Result<(), std::io::Error> {
        let j = serde_json::to_string_pretty(&self).expect("error");
        f.write_all(&j.as_bytes())
    }

    pub fn init_file_writer(receiver: Receiver<Account>, file_path: String) {
        let file = OpenOptions::new().append(true).open(file_path).expect("");

        thread::spawn(move || {
            let msg: Account = receiver.recv().unwrap();
            info!(" {:?} ", msg);
            msg.write_to_file(file);
        });
    }
}

#[test]
#[ignore]
fn test_deserialize() {
    let j = r#"{
  "id": "632b4e47-db03-4d67-8b91-08724e8b6a19",
  "account_number": "PA2MGKBZ7YOU",
  "status": "ACTIVE",
  "currency": "USD",
  "buying_power": "400000",
  "regt_buying_power": "200000",
  "daytrading_buying_power": "400000",
  "cash": "100000",
  "portfolio_value": "100000",
  "pattern_day_trader": false,
  "trading_blocked": false,
  "transfers_blocked": false,
  "account_blocked": false,
  "created_at": "2020-04-02T22:24:15.579155Z",
  "trade_suspended_by_user": false,
  "multiplier": "4",
  "shorting_enabled": true,
  "equity": "100000",
  "last_equity": "100000",
  "long_market_value": "0",
  "short_market_value": "0",
  "initial_margin": "0",
  "maintenance_margin": "0",
  "last_maintenance_margin": "0",
  "sma": "0",
  "daytrade_count": 0
}
"#;

    let account: Account = serde_json::from_str(j).unwrap();
    println!("{:?}", account);
    //let account = Account::get_account();
}
