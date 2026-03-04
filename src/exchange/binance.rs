use crate::model::tick::{AggressorSide, Tick};
use serde::Deserialize;
use std::time::SystemTime;

#[derive(Deserialize, Debug)]
struct AggTradeEvent {
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "a")]
    agg_trade_id: u64,
    #[serde(rename = "p")]
    price: String,
    #[serde(rename = "q")]
    quantity: String,
    #[serde(rename = "T")]
    trade_time: u64,
    #[serde(rename = "m")]
    is_buyer_maker: bool,
}

pub struct BinanceParser;

impl super::traits::ExchangeParser for BinanceParser {
    fn parse(&self, text: &str) -> anyhow::Result<Option<Tick>> {
        let event: AggTradeEvent = match serde_json::from_str(text) {
            Ok(v) => v,
            Err(_) => return Ok(None),
        };

        if event.event_type != "aggTrade" {
            return Ok(None);
        }

        let side = if event.is_buyer_maker {
            AggressorSide::Sell
        } else {
            AggressorSide::Buy
        };

        Ok(Some(Tick {
            symbol: event.symbol,
            price: event.price.parse()?,
            quantity: event.quantity.parse()?,
            trade_id: event.agg_trade_id,
            side,
            exchange_ts: event.trade_time,
            received_ts: SystemTime::now(),
        }))
    }
}
