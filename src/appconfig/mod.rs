use super::alpaca::*;
use super::kafkautil::*;
use config::{Config, File, FileFormat};
use std::env;
use std::rc::Rc;
use std::slice::RChunks;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub kafka_config: KafkaConfig,
    pub alpaca_config: Rc<AlpacaConfig>,
    pub polygon_config: PolygonConfig,
}

#[derive(Debug, Clone)]
pub struct PolygonConfig {
    pub(crate) url: String,
    pub(crate) key: String,
}

#[derive(Debug, Clone)]
pub struct AlpacaConfig {
    pub url: String,
    pub key: String,
    pub secret: String,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        let mut conf = Config::new();
        conf.merge(File::new("app", FileFormat::Json).required(true))
            .unwrap();

        debug!("Done loading  config file app.json");

        let name = format!("app_{}", env::var("env").unwrap_or("dev".into()));
        conf.merge(File::new(&name, FileFormat::Json).required(true))
            .unwrap();

        let kafka_config = KafkaConfig {
            broker_with_ip: conf.get_str("kafka.hosts").unwrap(),
        };

        let alpaca_config = Rc::new(AlpacaConfig {
            url: conf.get_str("alpaca.url").unwrap(),
            key: conf.get_str("alpaca.key").unwrap(),
            secret: conf.get_str("alpaca.secret").unwrap(),
        });

        let polygon_config = PolygonConfig {
            url: conf.get_str("polygon.url").unwrap(),
            key: conf.get_str("polygon.key").unwrap(),
        };

        debug!("Done reading config");

        return AppConfig {
            kafka_config,
            alpaca_config,
            polygon_config,
        };
    }
}
