// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// minted_denom is the denomination of the coin expected to be minted by the
    /// minting module. Pool-incentives module doesn’t actually mint the coin
    /// itself, but rather manages the distribution of coins that matches the
    /// defined minted_denom.
    #[prost(string, tag="1")]
    pub minted_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockableDurationsInfo {
    #[prost(message, repeated, tag="1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistrInfo {
    #[prost(string, tag="1")]
    pub total_weight: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistrRecord {
    #[prost(uint64, tag="1")]
    pub gauge_id: u64,
    #[prost(string, tag="2")]
    pub weight: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolToGauge {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    #[prost(uint64, tag="2")]
    pub gauge_id: u64,
    #[prost(message, optional, tag="3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyPoolToInternalGauges {
    #[prost(message, repeated, tag="2")]
    pub pool_to_gauge: ::prost::alloc::vec::Vec<PoolToGauge>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConcentratedPoolToNoLockGauges {
    #[prost(message, repeated, tag="1")]
    pub pool_to_gauge: ::prost::alloc::vec::Vec<PoolToGauge>,
}
/// GenesisState defines the pool incentives module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag="2")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
    #[prost(message, optional, tag="3")]
    pub distr_info: ::core::option::Option<DistrInfo>,
    /// any_pool_to_internal_gauges defines the gauges for any pool to internal
    /// pool. For every pool type (e.g. LP, Concentrated, etc), there is one such
    /// link
    #[prost(message, optional, tag="4")]
    pub any_pool_to_internal_gauges: ::core::option::Option<AnyPoolToInternalGauges>,
    /// concentrated_pool_to_no_lock_gauges defines the no lock gauges for
    /// concentrated pool. This only exists between concentrated pool and no lock
    /// gauges. Both external and internal gauges are included.
    #[prost(message, optional, tag="5")]
    pub concentrated_pool_to_no_lock_gauges: ::core::option::Option<ConcentratedPoolToNoLockGauges>,
}
/// ReplacePoolIncentivesProposal is a gov Content type for updating the pool
/// incentives. If a ReplacePoolIncentivesProposal passes, the proposal’s records
/// override the existing DistrRecords set in the module. Each record has a
/// specified gauge id and weight, and the incentives are distributed to each
/// gauge according to weight/total_weight. The incentives are put in the fee
/// pool and it is allocated to gauges and community pool by the DistrRecords
/// configuration. Note that gaugeId=0 represents the community pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplacePoolIncentivesProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
// UpdatePoolIncentivesProposal is a gov Content type for updating the pool
// incentives. If a UpdatePoolIncentivesProposal passes, all the DistrRecords
// in the proposals are edited. An existing DistrRecord is not overridden unless
// explicitly included in the proposal.
// This differs from an ReplacePoolIncentivesProposal because it only does an
// in place update of the DistrRecords for gauges explicitly mentioned in the
// proposal.

/// For example: if the existing DistrRecords were:
/// [(Gauge 0, 5), (Gauge 1, 6), (Gauge 2, 6)]
/// An UpdatePoolIncentivesProposal includes
/// [(Gauge 1, 0), (Gauge 2, 4), (Gauge 3, 10)]
/// This would delete Gauge 1, Edit Gauge 2, and Add Gauge 3
/// The result DistrRecords in state would be:
/// [(Gauge 0, 5), (Gauge 2, 4), (Gauge 3, 10)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePoolIncentivesProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGaugeIdsRequest {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGaugeIdsResponse {
    #[prost(message, repeated, tag="1")]
    pub gauge_ids_with_duration: ::prost::alloc::vec::Vec<query_gauge_ids_response::GaugeIdWithDuration>,
}
/// Nested message and enum types in `QueryGaugeIdsResponse`.
pub mod query_gauge_ids_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GaugeIdWithDuration {
        #[prost(uint64, tag="1")]
        pub gauge_id: u64,
        #[prost(message, optional, tag="2")]
        pub duration: ::core::option::Option<::prost_types::Duration>,
        #[prost(string, tag="3")]
        pub gauge_incentive_percentage: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDistrInfoRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDistrInfoResponse {
    #[prost(message, optional, tag="1")]
    pub distr_info: ::core::option::Option<DistrInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsResponse {
    #[prost(message, repeated, tag="1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIncentivizedPoolsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncentivizedPool {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    #[prost(message, optional, tag="2")]
    pub lockable_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(uint64, tag="3")]
    pub gauge_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIncentivizedPoolsResponse {
    #[prost(message, repeated, tag="1")]
    pub incentivized_pools: ::prost::alloc::vec::Vec<IncentivizedPool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalIncentiveGaugesRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalIncentiveGaugesResponse {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<super::super::incentives::Gauge>,
}
/// MigrationRecords contains all the links between balancer and concentrated
/// pools.
///
/// This is copied over from the gamm proto file in order to circumnavigate
/// the circular dependency between the two modules.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationRecords {
    #[prost(message, repeated, tag="1")]
    pub balancer_to_concentrated_pool_links: ::prost::alloc::vec::Vec<BalancerToConcentratedPoolLink>,
}
/// BalancerToConcentratedPoolLink defines a single link between a single
/// balancer pool and a single concentrated liquidity pool. This link is used to
/// allow a balancer pool to migrate to a single canonical full range
/// concentrated liquidity pool position
/// A balancer pool can be linked to a maximum of one cl pool, and a cl pool can
/// be linked to a maximum of one balancer pool.
///
/// This is copied over from the gamm proto file in order to circumnavigate
/// the circular dependency between the two modules.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalancerToConcentratedPoolLink {
    #[prost(uint64, tag="1")]
    pub balancer_pool_id: u64,
    #[prost(uint64, tag="2")]
    pub cl_pool_id: u64,
}
include!("osmosis.poolincentives.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)