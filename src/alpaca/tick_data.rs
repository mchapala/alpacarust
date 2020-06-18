use std::fs::{File, OpenOptions};
use std::io::Write;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

use chrono::prelude::*;
use crossbeam::queue::ArrayQueue;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::appconfig::AppConfig;
use crate::kafkautil::producer::{KafkaMsg, KafkaProducer};
use crate::kafkautil::KafkaConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TickData {
    pub ticker_name: String,
    pub price: f64,
    pub size: i64,
    pub trade_time_stamp: u64,
    pub received_time_stamp: i64,

    #[serde(default)]
    pub source: String,
}

impl TickData {
    pub fn to_json_str(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn from_polygon_json(map: serde_json::Map<String, Value>) -> TickData {
        let symbol = map.get("symbol").unwrap().to_string();
        let last: &Value = &*map.get("last").unwrap();
        //let start = SystemTime::now();
        let now = Utc::now();

        TickData {
            ticker_name: symbol,
            price: last.get("price").unwrap().as_f64().unwrap(),
            size: last.get("size").unwrap().as_i64().unwrap(),
            trade_time_stamp: last.get("timestamp").unwrap().as_u64().unwrap(),
            received_time_stamp: now.timestamp_millis(),
            source: "polygon".to_string(),
        }
    }

    pub fn write_to_file(&self, mut f: File) -> std::result::Result<(), std::io::Error> {
        let j = serde_json::to_string_pretty(&self).expect("error");
        f.write_all(&j.as_bytes())
    }

    pub fn init_kafka_writer_channel(receiver: Receiver<KafkaMsg>, kafka_config: KafkaConfig) {
        thread::spawn(move || {
            let mut producer = KafkaProducer::new(kafka_config);
            loop {
                let result = receiver.recv();
                if result.is_ok() {
                    let msg: KafkaMsg = result.unwrap();
                    info!(" {:?} ", msg);
                    let resp = producer.publish_message(msg);
                    info!(" {:?} ", resp);
                }
            }
        });
    }

    pub fn init_file_writer(receiver: Receiver<TickData>, file_path: String) {
        let file = OpenOptions::new().append(true).open(file_path).expect("");

        thread::spawn(move || {
            let msg: TickData = receiver.recv().unwrap();
            info!(" {:?} ", msg);
            msg.write_to_file(file);
        });
    }
}

#[test]
#[ignore]
fn test_kafka_publisher() {
    use std::{thread, time};

    let (tx, rx): (Sender<KafkaMsg>, Receiver<KafkaMsg>) = std::mpsc::channel();
    let ac = AppConfig::new();
    TickData::init_kafka_writer_channel(rx, ac.kafka_config);

    let msg = KafkaMsg {
        topic: "test".to_string(),
        key: "key".to_string(),
        message: "{}".to_string(),
    };
    let resp = tx.send(msg).unwrap();
    let ten_millis = time::Duration::from_millis(10000);

    thread::sleep(ten_millis);
    println!("Test response: {:?}", resp)
}

#[test]
#[ignore]
fn test_file_writing() {
    let path = "lines.txt";
    let output = OpenOptions::new()
        .read(false)
        .write(true)
        .open(path)
        .expect("not able to open file");

    let data = TickData {
        ticker_name: "".to_string(),
        price: 0.0,
        size: 0,
        trade_time_stamp: 0,
        received_time_stamp: 0,
        source: "".to_string(),
    };

    let file = OpenOptions::new().append(true).open(path).expect("");
    data.write_to_file(file).expect("");
    data.write_to_file(output).expect("");
}

#[test]
#[ignore]
fn test_tick_data_deserialization() {
    let t_json = r#"    
{
  "last": {
    "cond1": 14,
    "cond2": 12,
    "cond3": 41,
    "exchange": 11,
    "price": 282.95,
    "size": 100,
    "timestamp": 1587763781260
  },
  "status": "success",
  "symbol": "SPY"
}
    "#;
    let map = serde_json::from_str(t_json).unwrap();
    let td = TickData::from_polygon_json(map);
    println!("{:?}", td)
}
