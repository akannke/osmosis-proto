// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapAmountInRoute {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_out_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapAmountOutRoute {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_in_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapAmountInSplitRoute {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    #[prost(string, tag = "2")]
    pub token_in_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapAmountOutSplitRoute {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "2")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ModuleRouter defines a route encapsulating pool type.
/// It is used as the value of a mapping from pool id to the pool type,
/// allowing the pool manager to know which module to route swaps to given the
/// pool id.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleRoute {
    /// pool_type specifies the type of the pool
    #[prost(enumeration = "PoolType", tag = "1")]
    pub pool_type: i32,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
}
/// PoolType is an enumeration of all supported pool types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PoolType {
    /// Balancer is the standard xy=k curve. Its pool model is defined in x/gamm.
    Balancer = 0,
    /// Stableswap is the Solidly cfmm stable swap curve. Its pool model is defined
    /// in x/gamm.
    Stableswap = 1,
    /// Concentrated is the pool model specific to concentrated liquidity. It is
    /// defined in x/concentrated-liquidity.
    Concentrated = 2,
    /// CosmWasm is the pool model specific to CosmWasm. It is defined in
    /// x/cosmwasmpool.
    CosmWasm = 3,
}
impl PoolType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PoolType::Balancer => "Balancer",
            PoolType::Stableswap => "Stableswap",
            PoolType::Concentrated => "Concentrated",
            PoolType::CosmWasm => "CosmWasm",
        }
    }
}
/// ===================== MsgSwapExactAmountIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    #[prost(message, optional, tag = "3")]
    pub token_in: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSplitRouteSwapExactAmountIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSplitRouteSwapExactAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInSplitRoute>,
    #[prost(string, tag = "3")]
    pub token_in_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSplitRouteSwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSwapExactAmountOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "3")]
    pub token_in_max_amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub token_out: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSplitRouteSwapExactAmountOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSplitRouteSwapExactAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutSplitRoute>,
    #[prost(string, tag = "3")]
    pub token_out_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_in_max_amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSplitRouteSwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSetDenomPairTakerFee
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetDenomPairTakerFee {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub denom_pair_taker_fee: ::prost::alloc::vec::Vec<DenomPairTakerFee>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetDenomPairTakerFeeResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomPairTakerFee {
    /// denom0 and denom1 get automatically lexigographically sorted
    /// when being stored, so the order of input here does not matter.
    #[prost(string, tag = "1")]
    pub denom0: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom1: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub taker_fee: ::prost::alloc::string::String,
}
/// Params holds parameters for the poolmanager module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub pool_creation_fee:
        ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// taker_fee_params is the container of taker fee parameters.
    #[prost(message, optional, tag = "2")]
    pub taker_fee_params: ::core::option::Option<TakerFeeParams>,
    /// authorized_quote_denoms is a list of quote denoms that can be used as
    /// token1 when creating a concentrated pool. We limit the quote assets to a
    /// small set for the purposes of having convenient price increments stemming
    /// from tick to price conversion. These increments are in a human readable
    /// magnitude only for token1 as a quote. For limit orders in the future, this
    /// will be a desirable property in terms of UX as to allow users to set limit
    /// orders at prices in terms of token1 (quote asset) that are easy to reason
    /// about.
    #[prost(string, repeated, tag = "3")]
    pub authorized_quote_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GenesisState defines the poolmanager module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// the next_pool_id
    #[prost(uint64, tag = "1")]
    pub next_pool_id: u64,
    /// params is the container of poolmanager parameters.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
    /// pool_routes is the container of the mappings from pool id to pool type.
    #[prost(message, repeated, tag = "3")]
    pub pool_routes: ::prost::alloc::vec::Vec<ModuleRoute>,
    /// KVStore state
    #[prost(message, optional, tag = "4")]
    pub taker_fees_tracker: ::core::option::Option<TakerFeesTracker>,
    #[prost(message, repeated, tag = "5")]
    pub pool_volumes: ::prost::alloc::vec::Vec<PoolVolume>,
    #[prost(message, repeated, tag = "6")]
    pub denom_pair_taker_fee_store: ::prost::alloc::vec::Vec<DenomPairTakerFee>,
}
/// TakerFeeParams consolidates the taker fee parameters for the poolmanager.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakerFeeParams {
    /// default_taker_fee is the fee used when creating a new pool that doesn't
    /// fall under a custom pool taker fee or stableswap taker fee category.
    #[prost(string, tag = "1")]
    pub default_taker_fee: ::prost::alloc::string::String,
    /// osmo_taker_fee_distribution defines the distribution of taker fees
    /// generated in OSMO. As of this writing, it has two categories:
    /// - staking_rewards: the percent of the taker fee that gets distributed to
    ///    stakers.
    /// - community_pool: the percent of the taker fee that gets sent to the
    ///    community pool.
    #[prost(message, optional, tag = "2")]
    pub osmo_taker_fee_distribution: ::core::option::Option<TakerFeeDistributionPercentage>,
    /// non_osmo_taker_fee_distribution defines the distribution of taker fees
    /// generated in non-OSMO. As of this writing, it has two categories:
    /// - staking_rewards: the percent of the taker fee that gets swapped to OSMO
    ///    and then distributed to stakers.
    /// - community_pool: the percent of the taker fee that gets sent to the
    ///    community pool. Note: If the non-OSMO asset is an authorized_quote_denom,
    ///    that denom is sent directly to the community pool. Otherwise, it is
    ///    swapped to the community_pool_denom_to_swap_non_whitelisted_assets_to and
    ///    then sent to the community pool as that denom.
    #[prost(message, optional, tag = "3")]
    pub non_osmo_taker_fee_distribution: ::core::option::Option<TakerFeeDistributionPercentage>,
    /// admin_addresses is a list of addresses that are allowed to set and remove
    /// custom taker fees for denom pairs. Governance also has the ability to set
    /// and remove custom taker fees for denom pairs, but with the normal
    /// governance delay.
    #[prost(string, repeated, tag = "4")]
    pub admin_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// community_pool_denom_to_swap_non_whitelisted_assets_to is the denom that
    /// non-whitelisted taker fees will be swapped to before being sent to
    /// the community pool.
    #[prost(string, tag = "5")]
    pub community_pool_denom_to_swap_non_whitelisted_assets_to: ::prost::alloc::string::String,
    /// reduced_fee_whitelist is a list of addresses that are
    /// allowed to pay a reduce taker fee when performing a swap
    /// (i.e. swap without paying the taker fee).
    /// It is intended to be used for integrators who meet qualifying factors
    /// that are approved by governance.
    /// Initially, the taker fee is allowed to be bypassed completely. However
    /// In the future, we will charge a reduced taker fee instead of no fee at all.
    #[prost(string, repeated, tag = "6")]
    pub reduced_fee_whitelist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// TakerFeeDistributionPercentage defines what percent of the taker fee category
/// gets distributed to the available categories.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakerFeeDistributionPercentage {
    #[prost(string, tag = "1")]
    pub staking_rewards: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub community_pool: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakerFeesTracker {
    #[prost(message, repeated, tag = "1")]
    pub taker_fees_to_stakers:
        ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "2")]
    pub taker_fees_to_community_pool:
        ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "3")]
    pub height_accounting_starts_from: i64,
}
/// PoolVolume stores the KVStore entries for each pool's volume, which
/// is used in export/import genesis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolVolume {
    /// pool_id is the id of the pool.
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    /// pool_volume is the cumulative volume of the pool.
    #[prost(message, repeated, tag = "2")]
    pub pool_volume: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// DenomPairTakerFeeProposal is a type for adding/removing a custom taker fee(s)
/// for one or more denom pairs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomPairTakerFeeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub denom_pair_taker_fee: ::prost::alloc::vec::Vec<DenomPairTakerFee>,
}
/// =============================== Params
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// =============================== EstimateSwapExactAmountIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSwapExactAmountInRequest {
    #[deprecated]
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSwapExactAmountInWithPrimitiveTypesRequest {
    #[deprecated]
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub routes_pool_id: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, repeated, tag = "4")]
    pub routes_token_out_denom: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSinglePoolSwapExactAmountInRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_out_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// =============================== EstimateSwapExactAmountOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSwapExactAmountOutRequest {
    #[deprecated]
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(message, repeated, tag = "3")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "4")]
    pub token_out: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSwapExactAmountOutWithPrimitiveTypesRequest {
    #[deprecated]
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    pub routes_pool_id: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, repeated, tag = "3")]
    pub routes_token_in_denom: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub token_out: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSinglePoolSwapExactAmountOutRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_in_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_out: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// =============================== NumPools
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumPoolsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumPoolsResponse {
    #[prost(uint64, tag = "1")]
    pub num_pools: u64,
}
/// =============================== Pool
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolResponse {
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<::prost_types::Any>,
}
/// =============================== AllPools
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllPoolsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// =======================================================
/// ListPoolsByDenomRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoolsByDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoolsByDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// ==========================================================
/// SpotPriceRequest defines the gRPC request structure for a SpotPrice
/// query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotPriceRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset_denom: ::prost::alloc::string::String,
}
/// SpotPriceResponse defines the gRPC response structure for a SpotPrice
/// query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotPriceResponse {
    /// String of the Dec. Ex) 10.203uatom
    #[prost(string, tag = "1")]
    pub spot_price: ::prost::alloc::string::String,
}
/// =============================== TotalPoolLiquidity
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalPoolLiquidityRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalPoolLiquidityResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidity: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// =============================== TotalLiquidity
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalLiquidityRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalLiquidityResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidity: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// =============================== TotalVolumeForPool
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalVolumeForPoolRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalVolumeForPoolResponse {
    #[prost(message, repeated, tag = "1")]
    pub volume: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// =============================== TradingPairTakerFee
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingPairTakerFeeRequest {
    #[prost(string, tag = "1")]
    pub denom_0: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom_1: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingPairTakerFeeResponse {
    #[prost(string, tag = "1")]
    pub taker_fee: ::prost::alloc::string::String,
}
// =============================== EstimateTradeBasedOnPriceImpact

/// EstimateTradeBasedOnPriceImpactRequest represents a request to estimate a
/// trade for Balancer/StableSwap/Concentrated liquidity pool types based on the
/// given parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateTradeBasedOnPriceImpactRequest {
    /// from_coin is the total amount of tokens that the user wants to sell.
    #[prost(message, optional, tag = "1")]
    pub from_coin: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// to_coin_denom is the denom identifier of the token that the user wants to
    /// buy.
    #[prost(string, tag = "2")]
    pub to_coin_denom: ::prost::alloc::string::String,
    /// pool_id is the identifier of the liquidity pool that the trade will occur
    /// on.
    #[prost(uint64, tag = "3")]
    pub pool_id: u64,
    /// max_price_impact is the maximum percentage that the user is willing
    /// to affect the price of the liquidity pool.
    #[prost(string, tag = "4")]
    pub max_price_impact: ::prost::alloc::string::String,
    /// external_price is an optional external price that the user can enter.
    /// It adjusts the MaxPriceImpact as the SpotPrice of a pool can be changed at
    /// any time.
    #[prost(string, tag = "5")]
    pub external_price: ::prost::alloc::string::String,
}
/// EstimateTradeBasedOnPriceImpactResponse represents the response data
/// for an estimated trade based on price impact. If a trade fails to be
/// estimated the response would be 0,0 for input_coin and output_coin and will
/// not error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateTradeBasedOnPriceImpactResponse {
    /// input_coin is the actual input amount that would be tradeable
    /// under the specified price impact.
    #[prost(message, optional, tag = "1")]
    pub input_coin: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// output_coin is the amount of tokens of the ToCoinDenom type
    /// that will be received for the actual InputCoin trade.
    #[prost(message, optional, tag = "2")]
    pub output_coin: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackedVolume {
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
include!("osmosis.poolmanager.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
