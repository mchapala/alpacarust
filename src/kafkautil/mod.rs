use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, Read, Write};
use std::iter::FromIterator;
use std::result;
use std::str::from_utf8;
use std::time::Duration;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage, Message, MessageSets};

pub mod consumer;
pub mod producer;

#[derive(Debug, Clone)]
pub struct KafkaConfig {
    pub(crate) broker_with_ip: String,
}

pub struct KafkaTopic {
    brokerWithIp: String,
    kafkaConfig: KafkaConfig,
}

pub fn init() {}
