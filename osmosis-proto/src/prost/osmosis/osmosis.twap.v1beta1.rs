// @generated
/// A TWAP record should be indexed in state by pool_id, (asset pair), timestamp
/// The asset pair assets should be lexicographically sorted.
/// Technically (pool_id, asset_0_denom, asset_1_denom, height) do not need to
/// appear in the struct however we view this as the wrong performance tradeoff
/// given SDK today. Would rather we optimize for readability and correctness,
/// than an optimal state storage format. The system bottleneck is elsewhere for
/// now.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TwapRecord {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    /// Lexicographically smaller denom of the pair
    #[prost(string, tag = "2")]
    pub asset0_denom: ::prost::alloc::string::String,
    /// Lexicographically larger denom of the pair
    #[prost(string, tag = "3")]
    pub asset1_denom: ::prost::alloc::string::String,
    /// height this record corresponds to, for debugging purposes
    #[prost(int64, tag = "4")]
    pub height: i64,
    /// This field should only exist until we have a global registry in the state
    /// machine, mapping prior block heights within {TIME RANGE} to times.
    #[prost(message, optional, tag = "5")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// We store the last spot prices in the struct, so that we can interpolate
    /// accumulator values for times between when accumulator records are stored.
    #[prost(string, tag = "6")]
    pub p0_last_spot_price: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub p1_last_spot_price: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub p0_arithmetic_twap_accumulator: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub p1_arithmetic_twap_accumulator: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub geometric_twap_accumulator: ::prost::alloc::string::String,
    /// This field contains the time in which the last spot price error occurred.
    /// It is used to alert the caller if they are getting a potentially erroneous
    /// TWAP, due to an unforeseen underlying error.
    #[prost(message, optional, tag = "11")]
    pub last_error_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Params holds parameters for the twap module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag = "1")]
    pub prune_epoch_identifier: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub record_history_keep_period: ::core::option::Option<::prost_types::Duration>,
}
/// GenesisState defines the twap module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// twaps is the collection of all twap records.
    #[prost(message, repeated, tag = "1")]
    pub twaps: ::prost::alloc::vec::Vec<TwapRecord>,
    /// params is the container of twap parameters.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapResponse {
    #[prost(string, tag = "1")]
    pub arithmetic_twap: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapToNowRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapToNowResponse {
    #[prost(string, tag = "1")]
    pub arithmetic_twap: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeometricTwapRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeometricTwapResponse {
    #[prost(string, tag = "1")]
    pub geometric_twap: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeometricTwapToNowRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeometricTwapToNowResponse {
    #[prost(string, tag = "1")]
    pub geometric_twap: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
// @@protoc_insertion_point(module)
