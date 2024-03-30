use std::io::Read;
use serde::Deserialize;

#[derive(Deserialize)]
struct FetchPrice {
    symbol: String,
    price: String
}

pub(crate) fn fetch_price_from_json(body: &str) -> serde_json::Result<(String)> {
    let fp: FetchPrice = serde_json::from_str(body)?;
    Ok((fp.price).parse().unwrap())
}

pub(crate) fn fetch_price() -> anyhow::Result<String> {
    let mut res = reqwest::blocking::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    Ok(body)
}
