use crate::alpaca::tick_data::TickData;
use crate::kafkautil::producer::KafkaMsg;
use std::sync::mpsc::{Receiver, RecvError, SendError, Sender};

pub(crate) struct MapRecv<T, U, F: Fn(T) -> U> {
    pub(crate) recv: Receiver<T>,
    pub(crate) f: F,
}

pub(crate) struct MapSender<T, U, F: Fn(T) -> U> {
    pub(crate) sender: Sender<T>,
    pub(crate) f: F,
}

impl<T, U, F: Fn(T) -> U> MapRecv<T, U, F> {
    pub fn recv(&self) -> Result<U, RecvError> {
        self.recv.recv().map(|v| (self.f)(v))
    }
}
