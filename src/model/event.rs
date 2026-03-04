use crate::model::tick::Tick;

pub enum CandleEvent{
    Tick(Tick),
    MinuteClose(u64)
}