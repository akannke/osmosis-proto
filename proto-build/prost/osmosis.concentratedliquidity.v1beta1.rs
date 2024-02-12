// @generated
/// Position contains position's id, address, pool id, lower tick, upper tick
/// join time, and liquidity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    #[prost(uint64, tag="1")]
    pub position_id: u64,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub pool_id: u64,
    #[prost(int64, tag="4")]
    pub lower_tick: i64,
    #[prost(int64, tag="5")]
    pub upper_tick: i64,
    #[prost(message, optional, tag="6")]
    pub join_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="7")]
    pub liquidity: ::prost::alloc::string::String,
}
/// FullPositionBreakdown returns:
/// - the position itself
/// - the amount the position translates in terms of asset0 and asset1
/// - the amount of claimable fees
/// - the amount of claimable incentives
/// - the amount of incentives that would be forfeited if the position was closed
/// now
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullPositionBreakdown {
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<Position>,
    #[prost(message, optional, tag="2")]
    pub asset0: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="3")]
    pub asset1: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag="4")]
    pub claimable_spread_rewards: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag="5")]
    pub claimable_incentives: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag="6")]
    pub forfeited_incentives: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionWithPeriodLock {
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<Position>,
    #[prost(message, optional, tag="2")]
    pub locks: ::core::option::Option<super::super::lockup::PeriodLock>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickInfo {
    #[prost(string, tag="1")]
    pub liquidity_gross: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub liquidity_net: ::prost::alloc::string::String,
    /// Total spread rewards accumulated in the opposite direction that the tick
    /// was last crossed. i.e. if the current tick is to the right of this tick
    /// (meaning its currently a greater price), then this is the total spread
    /// rewards accumulated below the tick. If the current tick is to the left of
    /// this tick (meaning its currently at a lower price), then this is the total
    /// spread rewards accumulated above the tick.
    ///
    /// Note: the way this value is used depends on the direction of spread rewards
    /// we are calculating for. If we are calculating spread rewards below the
    /// lower tick and the lower tick is the active tick, then this is the
    /// spreadRewardGrowthGlobal - the lower tick's
    /// spreadRewardGrowthOppositeDirectionOfLastTraversal. If we are calculating
    /// spread rewards above the upper tick and the upper tick is the active tick,
    /// then this is just the tick's
    /// spreadRewardGrowthOppositeDirectionOfLastTraversal value.
    #[prost(message, repeated, tag="3")]
    pub spread_reward_growth_opposite_direction_of_last_traversal: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin>,
    /// uptime_trackers is a container encapsulating the uptime trackers.
    /// We use a container instead of a "repeated UptimeTracker" directly
    /// because we need the ability to serialize and deserialize the
    /// container easily for events when crossing a tick.
    #[prost(message, optional, tag="4")]
    pub uptime_trackers: ::core::option::Option<UptimeTrackers>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UptimeTrackers {
    #[prost(message, repeated, tag="1")]
    pub list: ::prost::alloc::vec::Vec<UptimeTracker>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UptimeTracker {
    #[prost(message, repeated, tag="1")]
    pub uptime_growth_outside: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin>,
}
/// IncentiveRecord is the high-level struct we use to deal with an independent
/// incentive being distributed on a pool. Note that PoolId, Denom, and MinUptime
/// are included in the key so we avoid storing them in state, hence the
/// distinction between IncentiveRecord and IncentiveRecordBody.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncentiveRecord {
    /// incentive_id is the id uniquely identifying this incentive record.
    #[prost(uint64, tag="1")]
    pub incentive_id: u64,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    /// incentive record body holds necessary
    #[prost(message, optional, tag="4")]
    pub incentive_record_body: ::core::option::Option<IncentiveRecordBody>,
    /// min_uptime is the minimum uptime required for liquidity to qualify for this
    /// incentive. It should be always be one of the supported uptimes in
    /// types.SupportedUptimes
    #[prost(message, optional, tag="5")]
    pub min_uptime: ::core::option::Option<::prost_types::Duration>,
}
/// IncentiveRecordBody represents the body stored in state for each individual
/// record.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncentiveRecordBody {
    /// remaining_coin is the total amount of incentives to be distributed
    #[prost(message, optional, tag="1")]
    pub remaining_coin: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin>,
    /// emission_rate is the incentive emission rate per second
    #[prost(string, tag="2")]
    pub emission_rate: ::prost::alloc::string::String,
    /// start_time is the time when the incentive starts distributing
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// FullTick contains tick index and pool id along with other tick model
/// information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullTick {
    /// pool id associated with the tick.
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    /// tick's index.
    #[prost(int64, tag="2")]
    pub tick_index: i64,
    /// tick's info.
    #[prost(message, optional, tag="3")]
    pub info: ::core::option::Option<TickInfo>,
}
/// PoolData represents a serialized pool along with its ticks
/// for genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolData {
    /// pool struct
    #[prost(message, optional, tag="1")]
    pub pool: ::core::option::Option<::prost_types::Any>,
    /// pool's ticks
    #[prost(message, repeated, tag="2")]
    pub ticks: ::prost::alloc::vec::Vec<FullTick>,
    #[prost(message, optional, tag="3")]
    pub spread_reward_accumulator: ::core::option::Option<AccumObject>,
    #[prost(message, repeated, tag="4")]
    pub incentives_accumulators: ::prost::alloc::vec::Vec<AccumObject>,
    /// incentive records to be set
    #[prost(message, repeated, tag="5")]
    pub incentive_records: ::prost::alloc::vec::Vec<IncentiveRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionData {
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<Position>,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
    #[prost(message, optional, tag="3")]
    pub spread_reward_accum_record: ::core::option::Option<super::super::accum::v1beta1::Record>,
    #[prost(message, repeated, tag="4")]
    pub uptime_accum_records: ::prost::alloc::vec::Vec<super::super::accum::v1beta1::Record>,
}
/// GenesisState defines the concentrated liquidity module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params are all the parameters of the module
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<super::Params>,
    /// pool data containing serialized pool struct and ticks.
    #[prost(message, repeated, tag="2")]
    pub pool_data: ::prost::alloc::vec::Vec<PoolData>,
    #[prost(message, repeated, tag="3")]
    pub position_data: ::prost::alloc::vec::Vec<PositionData>,
    #[prost(uint64, tag="4")]
    pub next_position_id: u64,
    #[prost(uint64, tag="5")]
    pub next_incentive_record_id: u64,
}
/// In original struct of Accum object, store.KVStore is stored together.
/// For handling genesis, we do not need to include store.KVStore since we use
/// CL module's KVStore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccumObject {
    /// Accumulator's name (pulled from AccumulatorContent)
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub accum_content: ::core::option::Option<super::super::accum::v1beta1::AccumulatorContent>,
}
/// CreateConcentratedLiquidityPoolsProposal is a gov Content type for creating
/// concentrated liquidity pools. If a CreateConcentratedLiquidityPoolsProposal
/// passes, the pools are created via pool manager module account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConcentratedLiquidityPoolsProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub pool_records: ::prost::alloc::vec::Vec<PoolRecord>,
}
/// TickSpacingDecreaseProposal is a gov Content type for proposing a tick
/// spacing decrease for a pool. The proposal will fail if one of the pools do
/// not exist, or if the new tick spacing is not less than the current tick
/// spacing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickSpacingDecreaseProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub pool_id_to_tick_spacing_records: ::prost::alloc::vec::Vec<PoolIdToTickSpacingRecord>,
}
/// PoolIdToTickSpacingRecord is a struct that contains a pool id to new tick
/// spacing pair.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolIdToTickSpacingRecord {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    #[prost(uint64, tag="2")]
    pub new_tick_spacing: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolRecord {
    #[prost(string, tag="1")]
    pub denom0: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom1: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub tick_spacing: u64,
    #[prost(string, tag="5")]
    pub spread_factor: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    /// pool's address holding all liquidity tokens.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// address holding the incentives liquidity.
    #[prost(string, tag="2")]
    pub incentives_address: ::prost::alloc::string::String,
    /// address holding spread rewards from swaps.
    #[prost(string, tag="3")]
    pub spread_rewards_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub id: u64,
    /// Amount of total liquidity
    #[prost(string, tag="5")]
    pub current_tick_liquidity: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub token0: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub token1: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub current_sqrt_price: ::prost::alloc::string::String,
    #[prost(int64, tag="9")]
    pub current_tick: i64,
    /// tick_spacing must be one of the authorized_tick_spacing values set in the
    /// concentrated-liquidity parameters
    #[prost(uint64, tag="10")]
    pub tick_spacing: u64,
    #[prost(int64, tag="11")]
    pub exponent_at_price_one: i64,
    /// spread_factor is the ratio that is charged on the amount of token in.
    #[prost(string, tag="12")]
    pub spread_factor: ::prost::alloc::string::String,
    /// last_liquidity_update is the last time either the pool liquidity or the
    /// active tick changed
    #[prost(message, optional, tag="13")]
    pub last_liquidity_update: ::core::option::Option<::prost_types::Timestamp>,
}
/// =============================== UserPositions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPositionsRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(message, optional, tag="3")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPositionsResponse {
    #[prost(message, repeated, tag="1")]
    pub positions: ::prost::alloc::vec::Vec<FullPositionBreakdown>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// =============================== PositionById
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionByIdRequest {
    #[prost(uint64, tag="1")]
    pub position_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionByIdResponse {
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<FullPositionBreakdown>,
}
/// =============================== Pools
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolsResponse {
    #[prost(message, repeated, tag="1")]
    pub pools: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// =============================== ModuleParams
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsResponse {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<super::Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickLiquidityNet {
    #[prost(string, tag="1")]
    pub liquidity_net: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub tick_index: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityDepthWithRange {
    #[prost(string, tag="1")]
    pub liquidity_amount: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub lower_tick: i64,
    #[prost(int64, tag="3")]
    pub upper_tick: i64,
}
/// =============================== LiquidityNetInDirection
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityNetInDirectionRequest {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    #[prost(string, tag="2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub start_tick: i64,
    #[prost(bool, tag="4")]
    pub use_cur_tick: bool,
    #[prost(int64, tag="5")]
    pub bound_tick: i64,
    #[prost(bool, tag="6")]
    pub use_no_bound: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityNetInDirectionResponse {
    #[prost(message, repeated, tag="1")]
    pub liquidity_depths: ::prost::alloc::vec::Vec<TickLiquidityNet>,
    #[prost(int64, tag="2")]
    pub current_tick: i64,
    #[prost(string, tag="3")]
    pub current_liquidity: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub current_sqrt_price: ::prost::alloc::string::String,
}
/// =============================== LiquidityPerTickRange
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityPerTickRangeRequest {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityPerTickRangeResponse {
    #[prost(message, repeated, tag="1")]
    pub liquidity: ::prost::alloc::vec::Vec<LiquidityDepthWithRange>,
    #[prost(int64, tag="2")]
    pub bucket_index: i64,
}
/// ===================== QueryClaimableSpreadRewards
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimableSpreadRewardsRequest {
    #[prost(uint64, tag="1")]
    pub position_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimableSpreadRewardsResponse {
    #[prost(message, repeated, tag="1")]
    pub claimable_spread_rewards: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// ===================== QueryClaimableIncentives
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimableIncentivesRequest {
    #[prost(uint64, tag="1")]
    pub position_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimableIncentivesResponse {
    #[prost(message, repeated, tag="1")]
    pub claimable_incentives: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag="2")]
    pub forfeited_incentives: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// ===================== QueryPoolAccumulatorRewards
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolAccumulatorRewardsRequest {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolAccumulatorRewardsResponse {
    #[prost(message, repeated, tag="1")]
    pub spread_reward_growth_global: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin>,
    #[prost(message, repeated, tag="2")]
    pub uptime_growth_global: ::prost::alloc::vec::Vec<UptimeTracker>,
}
/// ===================== QueryTickAccumulatorTrackers
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickAccumulatorTrackersRequest {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    #[prost(int64, tag="2")]
    pub tick_index: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickAccumulatorTrackersResponse {
    #[prost(message, repeated, tag="1")]
    pub spread_reward_growth_opposite_direction_of_last_traversal: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin>,
    #[prost(message, repeated, tag="2")]
    pub uptime_trackers: ::prost::alloc::vec::Vec<UptimeTracker>,
}
/// ===================== QueryIncentiveRecords
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncentiveRecordsRequest {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncentiveRecordsResponse {
    #[prost(message, repeated, tag="1")]
    pub incentive_records: ::prost::alloc::vec::Vec<IncentiveRecord>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// =============================== CFMMPoolIdLinkFromConcentratedPoolId
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CfmmPoolIdLinkFromConcentratedPoolIdRequest {
    #[prost(uint64, tag="1")]
    pub concentrated_pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CfmmPoolIdLinkFromConcentratedPoolIdResponse {
    #[prost(uint64, tag="1")]
    pub cfmm_pool_id: u64,
}
/// =============================== UserUnbondingPositions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserUnbondingPositionsRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserUnbondingPositionsResponse {
    #[prost(message, repeated, tag="1")]
    pub positions_with_period_lock: ::prost::alloc::vec::Vec<PositionWithPeriodLock>,
}
/// =============================== GetTotalLiquidity
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTotalLiquidityRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTotalLiquidityResponse {
    #[prost(message, repeated, tag="1")]
    pub total_liquidity: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// =============================== NumNextInitializedTicks
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumNextInitializedTicksRequest {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    #[prost(string, tag="2")]
    pub token_in_denom: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub num_next_initialized_ticks: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumNextInitializedTicksResponse {
    #[prost(message, repeated, tag="1")]
    pub liquidity_depths: ::prost::alloc::vec::Vec<TickLiquidityNet>,
    #[prost(int64, tag="2")]
    pub current_tick: i64,
    #[prost(string, tag="3")]
    pub current_liquidity: ::prost::alloc::string::String,
}
/// ===================== MsgCreatePosition
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePosition {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub lower_tick: i64,
    #[prost(int64, tag="4")]
    pub upper_tick: i64,
    /// tokens_provided is the amount of tokens provided for the position.
    /// It must at a minimum be of length 1 (for a single sided position)
    /// and at a maximum be of length 2 (for a position that straddles the current
    /// tick).
    #[prost(message, repeated, tag="5")]
    pub tokens_provided: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="6")]
    pub token_min_amount0: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub token_min_amount1: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePositionResponse {
    #[prost(uint64, tag="1")]
    pub position_id: u64,
    #[prost(string, tag="2")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub liquidity_created: ::prost::alloc::string::String,
    /// the lower and upper tick are in the response because there are
    /// instances in which multiple ticks represent the same price, so
    /// we may move their provided tick to the canonical tick that represents
    /// the same price.
    #[prost(int64, tag="6")]
    pub lower_tick: i64,
    #[prost(int64, tag="7")]
    pub upper_tick: i64,
}
/// ===================== MsgAddToPosition
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToPosition {
    #[prost(uint64, tag="1")]
    pub position_id: u64,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    /// amount0 represents the amount of token0 willing to put in.
    #[prost(string, tag="3")]
    pub amount0: ::prost::alloc::string::String,
    /// amount1 represents the amount of token1 willing to put in.
    #[prost(string, tag="4")]
    pub amount1: ::prost::alloc::string::String,
    /// token_min_amount0 represents the minimum amount of token0 desired from the
    /// new position being created. Note that this field indicates the min amount0
    /// corresponding to the liquidity that is being added, not the total
    /// liquidity of the position.
    #[prost(string, tag="5")]
    pub token_min_amount0: ::prost::alloc::string::String,
    /// token_min_amount1 represents the minimum amount of token1 desired from the
    /// new position being created. Note that this field indicates the min amount1
    /// corresponding to the liquidity that is being added, not the total
    /// liquidity of the position.
    #[prost(string, tag="6")]
    pub token_min_amount1: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToPositionResponse {
    #[prost(uint64, tag="1")]
    pub position_id: u64,
    #[prost(string, tag="2")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub amount1: ::prost::alloc::string::String,
}
/// ===================== MsgWithdrawPosition
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawPosition {
    #[prost(uint64, tag="1")]
    pub position_id: u64,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub liquidity_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawPositionResponse {
    #[prost(string, tag="1")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub amount1: ::prost::alloc::string::String,
}
/// ===================== MsgCollectSpreadRewards
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCollectSpreadRewards {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub position_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCollectSpreadRewardsResponse {
    #[prost(message, repeated, tag="1")]
    pub collected_spread_rewards: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// ===================== MsgCollectIncentives
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCollectIncentives {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub position_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCollectIncentivesResponse {
    #[prost(message, repeated, tag="1")]
    pub collected_incentives: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag="2")]
    pub forfeited_incentives: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// ===================== MsgFungifyChargedPositions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFungifyChargedPositions {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub position_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFungifyChargedPositionsResponse {
    #[prost(uint64, tag="1")]
    pub new_position_id: u64,
}
/// ===================== MsgTransferPositions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferPositions {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub position_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferPositionsResponse {
}
include!("osmosis.concentratedliquidity.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)