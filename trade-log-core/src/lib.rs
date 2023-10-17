mod trade_log;
mod trade_log_inner;

use std::sync::Arc;
pub use trade_log::*;
pub use trade_log_inner::*;

lazy_static::lazy_static! {
    pub static ref TRADE_LOG: Arc<TradeLog> = {
        Arc::new(TradeLog::new())
    };
}

