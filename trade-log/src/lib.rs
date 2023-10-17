#[macro_export]
macro_rules! trade_log {
    (
        $trader_id:tt,
        $account_id:tt,
        $process_id:tt,
        $operation_id:tt,
        $message:tt,
        $telemetry:expr,
        $( $key:tt = $value:expr ),*
    ) => {
        let mut trade_log_data = HashMap::new();
        $(
            trade_log_data.insert($key.to_string(), trade_log::serde_json::to_string($value).unwrap());
        )*

        trade_log_core::TRADE_LOG.write($trader_id, $account_id, Some($process_id), $operation_id, $message, Some($telemetry), Some(trade_log_data)).await;
    };
    (
        $trader_id:tt,
        $account_id:tt,
        $process_id:tt,
        $operation_id:tt,
        $message:tt,
        $( $key:tt = $value:expr ),*
    ) => {
        let mut trade_log_data = HashMap::new();

        $(
            trade_log_data.insert($key.to_string(), trade_log::serde_json::to_string($value).unwrap());
        )*

        trade_log_core::TRADE_LOG.write($trader_id, $account_id, Some($process_id), $operation_id, $message, None, Some(trade_log_data)).await;
    };
    (
        $trader_id:tt,
        $account_id:tt,
        $operation_id:tt,
        $message:tt,
        $telemetry:expr,
        $( $key:tt = $value:expr ),*
    ) => {
        let mut trade_log_data = HashMap::new();

        $(
            trade_log_data.insert($key.to_string(), trade_log::serde_json::to_string($value).unwrap());
        )*

        let none: Option<String> = None;

        trade_log_core::TRADE_LOG.write($trader_id, $account_id, none, $operation_id, $message, Some($telemetry), Some(trade_log_data)).await;
    };
    (
        $trader_id:tt,
        $account_id:tt,
        $operation_id:tt,
        $message:tt,
        $( $key:tt = $value:expr ),*
    ) => {
        let mut trade_log_data = HashMap::new();

        $(
            trade_log_data.insert($key.to_string(), trade_log::serde_json::to_string($value).unwrap());
        )*

        let none: Option<String> = None;

        trade_log_core::TRADE_LOG.write($trader_id, $account_id, none, $operation_id, $message, None, Some(trade_log_data)).await;
    };
}

pub extern crate serde_json;
pub extern crate trade_log_contracts as contracts;