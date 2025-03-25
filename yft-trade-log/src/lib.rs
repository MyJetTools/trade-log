#[macro_export]
macro_rules! trade_log {
    (
        $trader_id:expr,
        $account_id:expr,
        $process_id:expr,
        $operation_id:expr,
        $message:expr,
        $telemetry:expr,
        $( $key:tt = $value:expr ),*
    ) => {
        let mut trade_log_data = std::collections::HashMap::new();
        $(
            trade_log_data.insert($key.to_string(), trade_log::serde_json::to_string($value).unwrap());
        )*

        trade_log::core::TRADE_LOG.write($trader_id, $account_id, Some($process_id), $operation_id, $message, Some($telemetry), Some(trade_log_data)).await;
    };
    (
        $trader_id:expr,
        $account_id:expr,
        $process_id:expr,
        $operation_id:expr,
        $message:expr,
        $( $key:tt = $value:expr ),*
    ) => {
        let mut trade_log_data = std::collections::HashMap::new();

        $(
            trade_log_data.insert($key.to_string(), trade_log::serde_json::to_string($value).unwrap());
        )*

        trade_log::core::TRADE_LOG.write($trader_id, $account_id, Some($process_id), $operation_id, $message, None, Some(trade_log_data)).await;
    };
    (
        $trader_id:expr,
        $account_id:expr,
        $operation_id:expr,
        $message:expr,
        $telemetry:expr,
        $( $key:tt = $value:expr ),*
    ) => {
        let mut trade_log_data = std::collections::HashMap::new();

        $(
            trade_log_data.insert($key.to_string(), trade_log::serde_json::to_string($value).unwrap());
        )*

        let none: Option<String> = None;

        trade_log::core::TRADE_LOG.write($trader_id, $account_id, none, $operation_id, $message, Some($telemetry), Some(trade_log_data)).await;
    };
    (
        $trader_id:expr,
        $account_id:expr,
        $operation_id:expr,
        $message:expr,
        $( $key:tt = $value:expr ),*
    ) => {
        let mut trade_log_data = std::collections::HashMap::new();

        $(
            trade_log_data.insert($key.to_string(), trade_log::serde_json::to_string($value).unwrap());
        )*

        let none: Option<String> = None;

        trade_log::core::TRADE_LOG.write($trader_id, $account_id, none, $operation_id, $message, None, Some(trade_log_data)).await;
    };
}

pub extern crate serde_json;
pub extern crate trade_log_contracts as contracts;
pub extern crate trade_log_core as core;