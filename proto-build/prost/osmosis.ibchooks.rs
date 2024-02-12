// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, repeated, tag="1")]
    pub allowed_async_ack_contracts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEmitIbcAck {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub packet_sequence: u64,
    #[prost(string, tag="3")]
    pub channel: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEmitIbcAckResponse {
    #[prost(string, tag="1")]
    pub contract_result: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub ibc_ack: ::prost::alloc::string::String,
}
include!("osmosis.ibchooks.tonic.rs");
// @@protoc_insertion_point(module)