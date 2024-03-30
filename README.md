# binance-rust

e.g.:

```rust
use std::fs;
use std::{thread, time};
use binance_rust;

fn main() -> anyhow::Result<()> {
    let price = binance_rust::binance_price("BTCUSDT".to_string())?;
    println!("{}", price);
    Ok(())
}
```