use crate::alpaca::tick_data::TickData;
use std::time::Duration;

impl TickData {
    pub fn read_alpaca_data(
        queue: Arc<ArrayQueue<TickData>>,
        symbols: Vec<String>,
        app_config: AppConfig,
    ) {
        let client: reqwest::blocking::Client =
            reqwest::blocking::Client::builder().build().unwrap();
        let polygon_url: String = app_config.polygon_config.url;
        let polygon_key: String = app_config.polygon_config.key;
        thread::spawn(move || {
            let done = false; // mut done: bool
            while !done {
                for symbol in symbols.clone() {
                    let url = format!(
                        "{}{}{}{}{}",
                        polygon_url, "/v1/last/stocks/", symbol, "?apiKey=", polygon_key
                    );
                    println!("url: {:?}", url);
                    let res = client.get(&url).send().unwrap();
                    let tick_data_json: Map<String, Value> = res.json().unwrap();
                    println!("Tick data: {:?}", tick_data_json);

                    let tick_data: TickData = TickData::from_polygon_json(tick_data_json);
                    println!("Tick data: {:?}", tick_data);
                    queue.push(tick_data.clone());
                }
                thread::sleep(Duration::from_secs(1));
            }
        });
    }
}
