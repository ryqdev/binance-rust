mod util;

pub fn binance_price(symbol: String) -> anyhow::Result<String> {
    let body = util::fetch_price(symbol)?;
    let price = util::fetch_price_from_json(&body)?;
    Ok(price)
}
