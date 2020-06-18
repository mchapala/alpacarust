use crate::alpaca::tick_data::TickData;
use crate::kafkautil::producer::KafkaMsg;

//use crate::util::transform::*;

use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex, RwLock};

use crate::alpaca::account::Account;
use crate::alpaca::positions::Portfolio;
use crossbeam::queue::{ArrayQueue, PopError, PushError};
use std::ops::Deref;
use std::thread;
use std::time::Duration;

pub fn distribute_tick_data(
    tick_array_queue: Arc<ArrayQueue<TickData>>,
    portfolio: Arc<ArrayQueue<Portfolio>>,
    rwlock_account: Arc<RwLock<Account>>,
    sender: Sender<KafkaMsg>,
) {
    thread::spawn(move || loop {
        if tick_array_queue.len() > 0 {
            let msg: Result<TickData, PopError> = tick_array_queue.pop();
            println!("msg: {:?}", msg);
            sender.send(KafkaMsg::from(msg.unwrap()));
        }
        if portfolio.len() > 0 {
            let msg: Result<Portfolio, PopError> = portfolio.pop();
            println!("msg: {:?}", msg);
            sender.send(KafkaMsg::from(msg.unwrap()));
        }
        let account: &Account = &*rwlock_account.read().unwrap();
        let msg = KafkaMsg::from(account);
        sender.send(msg);

        if tick_array_queue.len() == 0 && portfolio.len() == 0 {
            thread::sleep(Duration::from_millis(100));
        }
    });
}
