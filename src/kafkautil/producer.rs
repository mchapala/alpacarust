use super::KafkaConfig;

use super::KafkaTopic;
use crate::alpaca::account::Account;
use crate::alpaca::positions::Portfolio;
use crate::alpaca::tick_data::TickData;
use chrono::Utc;
use kafka::producer::{Producer, Record, RequiredAcks};
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub struct KafkaProducer {
    producer: Box<Producer>,
}

#[derive(Debug, Clone)]
pub struct KafkaMsg {
    pub(crate) topic: String,
    pub(crate) key: String,
    pub(crate) message: String,
}

impl From<TickData> for KafkaMsg {
    fn from(item: TickData) -> Self {
        KafkaMsg {
            topic: "tick_data_last".to_string(),
            key: format!(
                "{}_{}_{}_{}_{}",
                item.source, item.ticker_name, item.price, item.size, item.received_time_stamp
            ),
            message: item.to_json_str(),
        }
    }
}

impl From<Portfolio> for KafkaMsg {
    fn from(item: Portfolio) -> Self {
        KafkaMsg {
            topic: "portfolio".to_string(),
            key: Utc::now().timestamp().to_string(),
            message: item.to_json_str(),
        }
    }
}

impl From<&Account> for KafkaMsg {
    fn from(item: &Account) -> Self {
        KafkaMsg {
            topic: "account".to_string(),
            key: Utc::now().timestamp().to_string(),
            message: item.to_json_str(),
        }
    }
}

impl KafkaProducer {
    pub fn new(kc: KafkaConfig) -> KafkaProducer {
        info!(
            "####################### Connecting to broker ##################### {:?}",
            kc.broker_with_ip
        );
        let producer = Producer::from_hosts(vec![kc.broker_with_ip])
            .with_ack_timeout(Duration::from_secs(5))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

        KafkaProducer {
            producer: Box::new(producer),
        }
    }

    pub fn publish_message(&mut self, msg: KafkaMsg) {
        let record = Record {
            key: msg.key.as_str(),
            value: msg.message.as_str(),
            topic: msg.topic.as_str(),
            partition: 1,
        };
        println!("Publishing: {:?}", record);

        let response = self.producer.send(&record);
        info!("Message Sent: {:?}, Response: {:?}", record, response);
        println!("Message Sent: {:?}, Response: {:?}", record, response);
    }
}
