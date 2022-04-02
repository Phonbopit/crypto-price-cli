use clap::Parser;
use serde::{Deserialize, Serialize};
use reqwest as request;
use reqwest::Error;

#[derive(Parser)]
#[clap(name="CryptoPrice")]
#[clap(author="Chai Phonbopit")]
#[clap(version="1.0.0")]
#[clap(about = "CLI to get crypto price from ftx spot market.")]
struct Args {
    // Name of the market.
    #[clap(short, long, default_value="BTC/USD")]
    market: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Market {
    name: String,
    price: f32,

    #[serde(rename = "volumeUsd24h")]
    volume_usd24h: f32,

    #[serde(rename = "type")]
    market_type: String
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    success: bool,
    result: Market
}

#[tokio::main]
async fn fetch(market: &str) -> Result<APIResponse, Error> {
    let url = format!("https://ftx.com/api/markets/{market}", market=market);

    println!("{}", url);

    let response: APIResponse = request::get(url)
        .await?.json().await?;

    Ok(response)
}

fn main() {
    let args = Args::parse();

    let data = fetch(&args.market);
    println!("{:#?}", data);
}
