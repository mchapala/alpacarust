use super::KafkaConfig;
use std::collections::HashMap;

use std::ascii::AsciiExt;
use std::iter::FromIterator;
use std::time::Duration;

use std::fs::File;
//use std::io::prelude::*;
use std::result;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage, Message, MessageSets};
use std::io::{Error, Read, Write};
use std::str::from_utf8;

struct KafkaConsumer {
    kafka_config: KafkaConfig,
    consumer_name: String,
    consumer: Box<Consumer>,
}

struct KafkaConsumerMessage {
    pub offset: i64,

    pub key: Box<String>,

    pub value: Box<String>,
}

impl KafkaConsumer {
    pub fn new(
        _consumer_group_name: &str,
        topic_name: &str,
        brokers_with_port: &str,
    ) -> KafkaConsumer {
        let brokers: Vec<&str> = brokers_with_port.split(',').collect();

        let _v1 = Vec::from_iter(brokers.iter().map(|x| x.to_string()));

        let consumer = Consumer::from_hosts(vec!["localhost:9092".to_owned()])
            .with_topic_partitions(topic_name.to_owned(), &[0, 1])
            .with_fallback_offset(FetchOffset::Earliest)
            .with_group("my-group".to_owned())
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
            .unwrap();

        KafkaConsumer {
            kafka_config: KafkaConfig {
                broker_with_ip: "".to_string(),
            },
            consumer_name: "".to_string(),
            consumer: Box::new(consumer),
        }
    }

    pub fn read_without_commit<'a>(&mut self) -> Vec<KafkaConsumerMessage> {
        let mut vec = Vec::new();
        for ms in self.consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let km = KafkaConsumerMessage {
                    offset: m.offset,
                    key: Box::new(from_utf8(m.key).unwrap().to_owned()),
                    value: Box::new(from_utf8(m.value).unwrap().to_owned()),
                };
                vec.push(km)
            }
            self.consumer.consume_messageset(ms);
        }
        return vec;
    }

    pub fn commit(&mut self) {
        self.consumer.commit_consumed();
    }
}
