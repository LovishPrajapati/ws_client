use crate::model::tick::Tick;

pub trait ExchangeParser: Send + Sync {
    fn parse(&self, text:&str) -> anyhow::Result<Option<Tick>>;
}