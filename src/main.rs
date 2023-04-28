#![allow(warnings)]

mod config;

mod api {
    pub mod market_data;
    pub mod stream;
}

use api::market_data;
use api::stream;

use config::Environment;
use apca::Client;
use apca::ApiInfo;

// #[allow(unused_variables)]
#[tokio::main]
async fn main() {
    let paper_config = config::Config::load(Environment::Paper);
    //pairs trading algorithm goes here
    let api_info = ApiInfo::from_parts(
        paper_config.alpaca_base_url,
        paper_config.alpaca_api_key,
        paper_config.alpaca_api_secret
    ).unwrap();

    // get most recent mid prices for a few securities
    let client = Client::new(api_info);

    let tickers = vec![String::from("SPY"), String::from("QQQ")];

    stream::stream_quotes(&client, tickers).await;
}
