// @generated
/// Gauge is an object that stores and distributes yields to recipients who
/// satisfy certain conditions. Currently gauges support conditions around the
/// duration for which a given denom is locked.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gauge {
    /// id is the unique ID of a Gauge
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// is_perpetual is a flag to show if it's a perpetual or non-perpetual gauge
    /// Non-perpetual gauges distribute their tokens equally per epoch while the
    /// gauge is in the active period. Perpetual gauges distribute all their tokens
    /// at a single time and only distribute their tokens again once the gauge is
    /// refilled, Intended for use with incentives that get refilled daily.
    #[prost(bool, tag = "2")]
    pub is_perpetual: bool,
    /// distribute_to is where the gauge rewards are distributed to.
    /// This is queried via lock duration or by timestamp
    #[prost(message, optional, tag = "3")]
    pub distribute_to: ::core::option::Option<super::lockup::QueryCondition>,
    /// coins is the total amount of coins that have been in the gauge
    /// Can distribute multiple coin denoms
    #[prost(message, repeated, tag = "4")]
    pub coins: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// start_time is the distribution start time
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// num_epochs_paid_over is the number of total epochs distribution will be
    /// completed over
    #[prost(uint64, tag = "6")]
    pub num_epochs_paid_over: u64,
    /// filled_epochs is the number of epochs distribution has been completed on
    /// already
    #[prost(uint64, tag = "7")]
    pub filled_epochs: u64,
    /// distributed_coins are coins that have been distributed already
    #[prost(message, repeated, tag = "8")]
    pub distributed_coins:
        ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockableDurationsInfo {
    /// List of incentivised durations that gauges will pay out to
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
/// Params holds parameters for the incentives module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// distr_epoch_identifier is what epoch type distribution will be triggered by
    /// (day, week, etc.)
    #[prost(string, tag = "1")]
    pub distr_epoch_identifier: ::prost::alloc::string::String,
    /// group_creation_fee is the fee required to create a new group
    /// It is only charged to all addresses other than incentive module account
    /// or addresses in the unrestricted_creator_whitelist
    #[prost(message, repeated, tag = "2")]
    pub group_creation_fee:
        ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// unrestricted_creator_whitelist is a list of addresses that are
    /// allowed to bypass restrictions on permissionless Group
    /// creation. In the future, we might expand these to creating gauges
    /// as well.
    /// The goal of this is to allow a subdao to manage incentives efficiently
    /// without being stopped by 5 day governance process or a high fee.
    /// At the same time, it prevents spam by having a fee for all
    /// other users.
    #[prost(string, repeated, tag = "3")]
    pub unrestricted_creator_whitelist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Note that while both InternalGaugeInfo and InternalGaugeRecord could
/// technically be replaced by DistrInfo and DistrRecord from the pool-incentives
/// module, we create separate types here to keep our abstractions clean and
/// readable (pool-incentives distribution abstractions are used in a very
/// specific way that does not directly relate to gauge logic). This also helps
/// us sidestep a refactor to avoid an import cycle.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalGaugeInfo {
    #[prost(string, tag = "1")]
    pub total_weight: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub gauge_records: ::prost::alloc::vec::Vec<InternalGaugeRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalGaugeRecord {
    #[prost(uint64, tag = "1")]
    pub gauge_id: u64,
    /// CurrentWeight is the current weight of this gauge being distributed to for
    /// this epoch. For instance, for volume splitting policy, this stores the
    /// volume generated in the last epoch of the linked pool.
    #[prost(string, tag = "2")]
    pub current_weight: ::prost::alloc::string::String,
    /// CumulativeWeight serves as a snapshot of the accumulator being tracked
    /// based on splitting policy. For instance, for volume splitting policy, this
    /// stores the cumulative volume for the linked pool at time of last update.
    #[prost(string, tag = "3")]
    pub cumulative_weight: ::prost::alloc::string::String,
}
/// Group is an object that stores a 1:1 mapped gauge ID, a list of pool gauge
/// info, and a splitting policy. These are grouped into a single abstraction to
/// allow for distribution of group incentives to internal gauges according to
/// the specified splitting policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    #[prost(uint64, tag = "1")]
    pub group_gauge_id: u64,
    #[prost(message, optional, tag = "2")]
    pub internal_gauge_info: ::core::option::Option<InternalGaugeInfo>,
    #[prost(enumeration = "SplittingPolicy", tag = "3")]
    pub splitting_policy: i32,
}
/// CreateGroup is called via governance to create a new group.
/// It takes an array of pool IDs to split the incentives across.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGroup {
    #[prost(uint64, repeated, tag = "1")]
    pub pool_ids: ::prost::alloc::vec::Vec<u64>,
}
/// GroupsWithGauge is a helper struct that stores a group and its
/// associated gauge.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupsWithGauge {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<Group>,
    #[prost(message, optional, tag = "2")]
    pub gauge: ::core::option::Option<Gauge>,
}
/// SplittingPolicy determines the way we want to split incentives in groupGauges
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SplittingPolicy {
    ByVolume = 0,
}
impl SplittingPolicy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SplittingPolicy::ByVolume => "ByVolume",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ByVolume" => Some(Self::ByVolume),
            _ => None,
        }
    }
}
/// GenesisState defines the incentives module's various parameters when first
/// initialized
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params are all the parameters of the module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// gauges are all gauges (not including group gauges) that should exist at
    /// genesis
    #[prost(message, repeated, tag = "2")]
    pub gauges: ::prost::alloc::vec::Vec<Gauge>,
    /// lockable_durations are all lockup durations that gauges can be locked for
    /// in order to receive incentives
    #[prost(message, repeated, tag = "3")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
    /// last_gauge_id is what the gauge number will increment from when creating
    /// the next gauge after genesis
    #[prost(uint64, tag = "4")]
    pub last_gauge_id: u64,
    /// gauges are all group gauges that should exist at genesis
    #[prost(message, repeated, tag = "5")]
    pub group_gauges: ::prost::alloc::vec::Vec<Gauge>,
    /// groups are all the groups that should exist at genesis
    #[prost(message, repeated, tag = "6")]
    pub groups: ::prost::alloc::vec::Vec<Group>,
}
/// CreateGroupsProposal is a type for creating one or more groups via
/// governance. This is useful for creating groups without having to pay
/// creation fees.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGroupsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub create_groups: ::prost::alloc::vec::Vec<CreateGroup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleToDistributeCoinsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleToDistributeCoinsResponse {
    /// Coins that have yet to be distributed
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugeByIdRequest {
    /// Gague ID being queried
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugeByIdResponse {
    /// Gauge that corresponds to provided gague ID
    #[prost(message, optional, tag = "1")]
    pub gauge: ::core::option::Option<Gauge>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugesRequest {
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugesResponse {
    /// Upcoming and active gauges
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesRequest {
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesResponse {
    /// Active gagues only
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesPerDenomRequest {
    /// Desired denom when querying active gagues
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesPerDenomResponse {
    /// Active gagues that match denom in query
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesRequest {
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesResponse {
    /// Gauges whose distribution is upcoming
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesPerDenomRequest {
    /// Filter for upcoming gagues that match specific denom
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesPerDenomResponse {
    /// Upcoming gagues that match denom in query
    #[prost(message, repeated, tag = "1")]
    pub upcoming_gauges: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsEstRequest {
    /// Address that is being queried for future estimated rewards
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// Lock IDs included in future reward estimation
    #[prost(uint64, repeated, tag = "2")]
    pub lock_ids: ::prost::alloc::vec::Vec<u64>,
    /// Upper time limit of reward estimation
    /// Lower limit is current epoch
    #[prost(int64, tag = "3")]
    pub end_epoch: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsEstResponse {
    /// Estimated coin rewards that will be received at provided address
    /// from specified locks between current time and end epoch
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsResponse {
    /// Time durations that users can lock coins for in order to receive rewards
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllGroupsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllGroupsResponse {
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<Group>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllGroupsGaugesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllGroupsGaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub gauges: ::prost::alloc::vec::Vec<Gauge>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllGroupsWithGaugeRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllGroupsWithGaugeResponse {
    #[prost(message, repeated, tag = "1")]
    pub groups_with_gauge: ::prost::alloc::vec::Vec<GroupsWithGauge>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupByGroupGaugeIdRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupByGroupGaugeIdResponse {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<Group>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentWeightByGroupGaugeIdRequest {
    #[prost(uint64, tag = "1")]
    pub group_gauge_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentWeightByGroupGaugeIdResponse {
    #[prost(message, repeated, tag = "1")]
    pub gauge_weight: ::prost::alloc::vec::Vec<GaugeWeight>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugeWeight {
    #[prost(uint64, tag = "1")]
    pub gauge_id: u64,
    #[prost(string, tag = "2")]
    pub weight_ratio: ::prost::alloc::string::String,
}
/// MsgCreateGauge creates a gague to distribute rewards to users
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGauge {
    /// is_perpetual shows if it's a perpetual or non-perpetual gauge
    /// Non-perpetual gauges distribute their tokens equally per epoch while the
    /// gauge is in the active period. Perpetual gauges distribute all their tokens
    /// at a single time and only distribute their tokens again once the gauge is
    /// refilled
    #[prost(bool, tag = "1")]
    pub is_perpetual: bool,
    /// owner is the address of gauge creator
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// distribute_to show which lock the gauge should distribute to by time
    /// duration or by timestamp
    #[prost(message, optional, tag = "3")]
    pub distribute_to: ::core::option::Option<super::lockup::QueryCondition>,
    /// coins are coin(s) to be distributed by the gauge
    #[prost(message, repeated, tag = "4")]
    pub coins: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// start_time is the distribution start time
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// num_epochs_paid_over is the number of epochs distribution will be completed
    /// over
    #[prost(uint64, tag = "6")]
    pub num_epochs_paid_over: u64,
    /// pool_id is the ID of the pool that the gauge is meant to be associated
    /// with. if pool_id is set, then the "QueryCondition.LockQueryType" must be
    /// "NoLock" with all other fields of the "QueryCondition.LockQueryType" struct
    /// unset, including "QueryCondition.Denom". However, note that, internally,
    /// the empty string in "QueryCondition.Denom" ends up being overwritten with
    /// incentivestypes.NoLockExternalGaugeDenom(<pool-id>) so that the gauges
    /// associated with a pool can be queried by this prefix if needed.
    #[prost(uint64, tag = "7")]
    pub pool_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGaugeResponse {}
/// MsgAddToGauge adds coins to a previously created gauge
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToGauge {
    /// owner is the gauge owner's address
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// gauge_id is the ID of gauge that rewards are getting added to
    #[prost(uint64, tag = "2")]
    pub gauge_id: u64,
    /// rewards are the coin(s) to add to gauge
    #[prost(message, repeated, tag = "3")]
    pub rewards: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToGaugeResponse {}
/// MsgCreateGroup creates a group to distribute rewards to a group of pools
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGroup {
    /// coins are the provided coins that the group will distribute
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// num_epochs_paid_over is the number of epochs distribution will be completed
    /// in. 0 means it's perpetual
    #[prost(uint64, tag = "2")]
    pub num_epochs_paid_over: u64,
    /// owner is the group owner's address
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
    /// pool_ids are the IDs of pools that the group is comprised of
    #[prost(uint64, repeated, tag = "4")]
    pub pool_ids: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGroupResponse {
    /// group_id is the ID of the group that is created from this msg
    #[prost(uint64, tag = "1")]
    pub group_id: u64,
}
// @@protoc_insertion_point(module)
