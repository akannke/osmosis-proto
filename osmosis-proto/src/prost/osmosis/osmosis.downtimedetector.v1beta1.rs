// @generated
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Downtime {
    Duration30s = 0,
    Duration1m = 1,
    Duration2m = 2,
    Duration3m = 3,
    Duration4m = 4,
    Duration5m = 5,
    Duration10m = 6,
    Duration20m = 7,
    Duration30m = 8,
    Duration40m = 9,
    Duration50m = 10,
    Duration1h = 11,
    Duration15h = 12,
    Duration2h = 13,
    Duration25h = 14,
    Duration3h = 15,
    Duration4h = 16,
    Duration5h = 17,
    Duration6h = 18,
    Duration9h = 19,
    Duration12h = 20,
    Duration18h = 21,
    Duration24h = 22,
    Duration36h = 23,
    Duration48h = 24,
}
impl Downtime {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Downtime::Duration30s => "DURATION_30S",
            Downtime::Duration1m => "DURATION_1M",
            Downtime::Duration2m => "DURATION_2M",
            Downtime::Duration3m => "DURATION_3M",
            Downtime::Duration4m => "DURATION_4M",
            Downtime::Duration5m => "DURATION_5M",
            Downtime::Duration10m => "DURATION_10M",
            Downtime::Duration20m => "DURATION_20M",
            Downtime::Duration30m => "DURATION_30M",
            Downtime::Duration40m => "DURATION_40M",
            Downtime::Duration50m => "DURATION_50M",
            Downtime::Duration1h => "DURATION_1H",
            Downtime::Duration15h => "DURATION_1_5H",
            Downtime::Duration2h => "DURATION_2H",
            Downtime::Duration25h => "DURATION_2_5H",
            Downtime::Duration3h => "DURATION_3H",
            Downtime::Duration4h => "DURATION_4H",
            Downtime::Duration5h => "DURATION_5H",
            Downtime::Duration6h => "DURATION_6H",
            Downtime::Duration9h => "DURATION_9H",
            Downtime::Duration12h => "DURATION_12H",
            Downtime::Duration18h => "DURATION_18H",
            Downtime::Duration24h => "DURATION_24H",
            Downtime::Duration36h => "DURATION_36H",
            Downtime::Duration48h => "DURATION_48H",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisDowntimeEntry {
    #[prost(enumeration = "Downtime", tag = "1")]
    pub duration: i32,
    #[prost(message, optional, tag = "2")]
    pub last_downtime: ::core::option::Option<::prost_types::Timestamp>,
}
/// GenesisState defines the twap module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub downtimes: ::prost::alloc::vec::Vec<GenesisDowntimeEntry>,
    #[prost(message, optional, tag = "2")]
    pub last_block_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Query for has it been at least $RECOVERY_DURATION units of time,
/// since the chain has been down for $DOWNTIME_DURATION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoveredSinceDowntimeOfLengthRequest {
    #[prost(enumeration = "Downtime", tag = "1")]
    pub downtime: i32,
    #[prost(message, optional, tag = "2")]
    pub recovery: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoveredSinceDowntimeOfLengthResponse {
    #[prost(bool, tag = "1")]
    pub succesfully_recovered: bool,
}
include!("osmosis.downtimedetector.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
