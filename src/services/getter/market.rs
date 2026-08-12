use std::collections::HashMap;
use serde_json::Value;
use std::fmt::Write;

pub async fn get_price(_price: &str) -> String {
    let url = format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=USD", _price);
    let response = match reqwest::get(&url).await {
        Ok(resp) => resp,
        Err(_) => return format!("Failed to reach price service for {}", _price),
    };

    let mut report = String::new();
    match response.json::<HashMap<String, f64>>().await {
        Ok(num) => {
            if let Some(price_usd) = num.get("USD") {
                write!(&mut report, "The price of {} is ${}", _price, price_usd).unwrap();
            } else {
                write!(&mut report, "The price of {} is not supported yet", _price).unwrap();
            }
            report
        }
        Err(_) => {
            write!(&mut report, "The price of {} is not supported yet", _price).unwrap();
            report
        }
    }
}

pub async fn get_marketcap(coin: &str) -> String {
    let token = format!("https://min-api.cryptocompare.com/data/pricemultifull?fsyms={}&tsyms=USD", coin);
    let response = match reqwest::get(&token).await {
        Ok(resp) => resp,
        Err(_) => return format!("Error fetching market data for {}", coin),
    };

    let market_cap: Value = match response.json().await {
        Ok(v) => v,
        Err(_) => return "Error parsing market data JSON.".to_string(),
    };

    let mut report = String::new();
    if market_cap.get("DISPLAY").is_some() {
        if let Some(mktcap) = market_cap
            .get("DISPLAY")
            .and_then(|d| d.get(coin))
            .and_then(|c| c.get("USD"))
            .and_then(|u| u.get("MKTCAP"))
            .and_then(|m| m.as_str())
        {
            write!(&mut report, "The market capitalization of {} is {}", coin, mktcap).unwrap();
            return report;
        }
    }

    write!(&mut report, "Error fetching data!").unwrap();
    report
}