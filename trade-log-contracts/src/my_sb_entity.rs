use yft_service_sdk::external::my_service_bus_sdk::macros::my_sb_entity_protobuf_model;
use yft_service_sdk::external::my_service_bus_sdk;
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "trade-log")]
pub struct TradeLogSbModel {
    #[prost(int64, tag = "1")]
    pub date_time_unix_micros: i64,
    #[prost(string, tag = "2")]
    pub trader_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub component: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub process_id: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub operation_id: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "8")]
    pub data: Vec<TradeLogSbModelDataItem>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeLogSbModelDataItem {
    #[prost(string, tag = "1")]
    pub key: String,
    #[prost(string, tag = "2")]
    pub value: String,
}
