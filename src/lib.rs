mod util;

pub fn binance_price() -> anyhow::Result<String> {
    let body = util::fetch_price()?;
    let price = util::fetch_price_from_json(&body)?;
    Ok(price)
}
