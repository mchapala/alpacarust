mod alpaca;
mod app;
mod appconfig;
mod http;
mod kafkautil;
mod models;
mod util;

extern crate config;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate env_logger;

use crate::appconfig::AppConfig;

use crate::alpaca::account::Account;
use crate::alpaca::tick_data::TickData;
use crate::kafkautil::producer::KafkaMsg;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, RwLock};

use crate::alpaca::positions::Portfolio;
use crossbeam::queue::ArrayQueue;
use std::{thread, time};

fn main() {
    println!("Hello ................? !");
    env_logger::init();
    let ac = AppConfig::new();

    info!("Debugging {:?}", ac);

    // global account
    let account = Account::default();
    let rwlock_account = Arc::new(RwLock::new(account));

    Account::read_alpaca_data(rwlock_account.clone(), ac.clone());

    let tick_array_queue = Arc::new(ArrayQueue::<TickData>::new(2000));
    let portfolio_queue = Arc::new(ArrayQueue::<Portfolio>::new(2000));

    // channel.  kafka_publisher_receiver will published messages
    // kafka_msg_sender will transform original message into reqd format KafkaMsg
    let (kafka_msg_sender, kafka_publisher_receiver): (Sender<KafkaMsg>, Receiver<KafkaMsg>) =
        mpsc::channel();

    // will keep publishing tick data to receivers.
    let mut symbols = Vec::<String>::new();
    symbols.push("SPY".parse().unwrap());
    symbols.push("BAC".parse().unwrap());

    TickData::read_alpaca_data(tick_array_queue.clone(), symbols, ac.clone());

    TickData::init_kafka_writer_channel(kafka_publisher_receiver, ac.clone().kafka_config);
    thread::sleep(time::Duration::from_secs(6000));

    Portfolio::read_alpaca_data(portfolio_queue.clone(), ac.clone());

    app::distribute_tick_data(
        tick_array_queue.clone(),
        portfolio_queue.clone(),
        rwlock_account.clone(),
        kafka_msg_sender,
    );
}
