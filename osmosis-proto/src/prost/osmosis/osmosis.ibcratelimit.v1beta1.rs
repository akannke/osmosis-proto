// @generated
/// Params defines the parameters for the ibc-rate-limit module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
/// GenesisState defines the ibc-rate-limit module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params are all the parameters of the module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// ParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsRequest {}
/// aramsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
include!("osmosis.ibcratelimit.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
