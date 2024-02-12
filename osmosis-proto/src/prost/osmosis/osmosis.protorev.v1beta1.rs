// @generated
/// TokenPairArbRoutes tracks all of the hot routes for a given pair of tokens
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenPairArbRoutes {
    /// Stores all of the possible hot paths for a given pair of tokens
    #[prost(message, repeated, tag = "1")]
    pub arb_routes: ::prost::alloc::vec::Vec<Route>,
    /// Token denomination of the first asset
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    /// Token denomination of the second asset
    #[prost(string, tag = "3")]
    pub token_out: ::prost::alloc::string::String,
}
/// Route is a hot route for a given pair of tokens
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    /// The pool IDs that are traversed in the directed cyclic graph (traversed
    /// left
    /// -> right)
    #[prost(message, repeated, tag = "1")]
    pub trades: ::prost::alloc::vec::Vec<Trade>,
    /// The step size that will be used to find the optimal swap amount in the
    /// binary search
    #[prost(string, tag = "2")]
    pub step_size: ::prost::alloc::string::String,
}
/// Trade is a single trade in a route
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    /// The pool id of the pool that is traded on
    #[prost(uint64, tag = "1")]
    pub pool: u64,
    /// The denom of the token that is traded
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    /// The denom of the token that is received
    #[prost(string, tag = "3")]
    pub token_out: ::prost::alloc::string::String,
}
/// RouteStatistics contains the number of trades the module has executed after a
/// swap on a given route and the profits from the trades
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteStatistics {
    /// profits is the total profit from all trades on this route
    #[prost(message, repeated, tag = "1")]
    pub profits: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// number_of_trades is the number of trades the module has executed using this
    /// route
    #[prost(string, tag = "2")]
    pub number_of_trades: ::prost::alloc::string::String,
    /// route is the route that was used (pool ids along the arbitrage route)
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub route: ::prost::alloc::vec::Vec<u64>,
}
/// PoolWeights contains the weights of all of the different pool types. This
/// distinction is made and necessary because the execution time ranges
/// significantly between the different pool types. Each weight roughly
/// corresponds to the amount of time (in ms) it takes to execute a swap on that
/// pool type.
///
/// DEPRECATED: This field is deprecated and will be removed in the next
/// release. It is replaced by the `info_by_pool_type` field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolWeights {
    /// The weight of a stableswap pool
    #[prost(uint64, tag = "1")]
    pub stable_weight: u64,
    /// The weight of a balancer pool
    #[prost(uint64, tag = "2")]
    pub balancer_weight: u64,
    /// The weight of a concentrated pool
    #[prost(uint64, tag = "3")]
    pub concentrated_weight: u64,
    /// The weight of a cosmwasm pool
    #[prost(uint64, tag = "4")]
    pub cosmwasm_weight: u64,
}
/// InfoByPoolType contains information pertaining to how expensive (in terms of
/// gas and time) it is to execute a swap on a given pool type. This distinction
/// is made and necessary because the execution time ranges significantly between
/// the different pool types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoByPoolType {
    /// The stable pool info
    #[prost(message, optional, tag = "1")]
    pub stable: ::core::option::Option<StablePoolInfo>,
    /// The balancer pool info
    #[prost(message, optional, tag = "2")]
    pub balancer: ::core::option::Option<BalancerPoolInfo>,
    /// The concentrated pool info
    #[prost(message, optional, tag = "3")]
    pub concentrated: ::core::option::Option<ConcentratedPoolInfo>,
    /// The cosmwasm pool info
    #[prost(message, optional, tag = "4")]
    pub cosmwasm: ::core::option::Option<CosmwasmPoolInfo>,
}
/// StablePoolInfo contains meta data pertaining to a stableswap pool type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StablePoolInfo {
    /// The weight of a stableswap pool
    #[prost(uint64, tag = "1")]
    pub weight: u64,
}
/// BalancerPoolInfo contains meta data pertaining to a balancer pool type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalancerPoolInfo {
    /// The weight of a balancer pool
    #[prost(uint64, tag = "1")]
    pub weight: u64,
}
/// ConcentratedPoolInfo contains meta data pertaining to a concentrated pool
/// type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConcentratedPoolInfo {
    /// The weight of a concentrated pool
    #[prost(uint64, tag = "1")]
    pub weight: u64,
    /// The maximum number of ticks we can move when rebalancing
    #[prost(uint64, tag = "2")]
    pub max_ticks_crossed: u64,
}
/// CosmwasmPoolInfo contains meta data pertaining to a cosmwasm pool type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmwasmPoolInfo {
    /// The weight of a cosmwasm pool (by contract address)
    #[prost(message, repeated, tag = "1")]
    pub weight_maps: ::prost::alloc::vec::Vec<WeightMap>,
}
/// WeightMap maps a contract address to a weight. The weight of an address
/// corresponds to the amount of ms required to execute a swap on that contract.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightMap {
    /// The weight of a cosmwasm pool (by contract address)
    #[prost(uint64, tag = "1")]
    pub weight: u64,
    /// The contract address
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}
/// BaseDenom represents a single base denom that the module uses for its
/// arbitrage trades. It contains the denom name alongside the step size of the
/// binary search that is used to find the optimal swap amount
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseDenom {
    /// The denom i.e. name of the base denom (ex. uosmo)
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// The step size of the binary search that is used to find the optimal swap
    /// amount
    #[prost(string, tag = "2")]
    pub step_size: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllProtocolRevenue {
    #[prost(message, optional, tag = "1")]
    pub taker_fees_tracker:
        ::core::option::Option<super::super::poolmanager::v1beta1::TakerFeesTracker>,
    #[prost(message, optional, tag = "3")]
    pub cyclic_arb_tracker: ::core::option::Option<CyclicArbTracker>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CyclicArbTracker {
    #[prost(message, repeated, tag = "1")]
    pub cyclic_arb: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "2")]
    pub height_accounting_starts_from: i64,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Boolean whether the protorev module is enabled.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// The admin account (settings manager) of the protorev module.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
}
/// GenesisState defines the protorev module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// Parameters for the protorev module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// Token pair arb routes for the protorev module (hot routes).
    #[prost(message, repeated, tag = "2")]
    pub token_pair_arb_routes: ::prost::alloc::vec::Vec<TokenPairArbRoutes>,
    /// The base denominations being used to create cyclic arbitrage routes via the
    /// highest liquidity method.
    #[prost(message, repeated, tag = "3")]
    pub base_denoms: ::prost::alloc::vec::Vec<BaseDenom>,
    /// The pool weights that are being used to calculate the weight (compute cost)
    /// of each route.
    ///
    /// DEPRECATED: This field is deprecated and will be removed in the next
    /// release. It is replaced by the `info_by_pool_type` field.
    #[prost(message, optional, tag = "4")]
    pub pool_weights: ::core::option::Option<PoolWeights>,
    /// The number of days since module genesis.
    #[prost(uint64, tag = "5")]
    pub days_since_module_genesis: u64,
    /// The fees the developer account has accumulated over time.
    #[prost(message, repeated, tag = "6")]
    pub developer_fees: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// The latest block height that the module has processed.
    #[prost(uint64, tag = "7")]
    pub latest_block_height: u64,
    /// The developer account address of the module.
    #[prost(string, tag = "8")]
    pub developer_address: ::prost::alloc::string::String,
    /// Max pool points per block i.e. the maximum compute time (in ms)
    /// that protorev can use per block.
    #[prost(uint64, tag = "9")]
    pub max_pool_points_per_block: u64,
    /// Max pool points per tx i.e. the maximum compute time (in ms) that
    /// protorev can use per tx.
    #[prost(uint64, tag = "10")]
    pub max_pool_points_per_tx: u64,
    /// The number of pool points that have been consumed in the current block.
    #[prost(uint64, tag = "11")]
    pub point_count_for_block: u64,
    /// All of the profits that have been accumulated by the module.
    #[prost(message, repeated, tag = "12")]
    pub profits: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Information that is used to estimate execution time / gas
    /// consumption of a swap on a given pool type.
    #[prost(message, optional, tag = "13")]
    pub info_by_pool_type: ::core::option::Option<InfoByPoolType>,
    #[prost(message, optional, tag = "14")]
    pub cyclic_arb_tracker: ::core::option::Option<CyclicArbTracker>,
}
/// SetProtoRevEnabledProposal is a gov Content type to update whether the
/// protorev module is enabled
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetProtoRevEnabledProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub enabled: bool,
}
/// SetProtoRevAdminAccountProposal is a gov Content type to set the admin
/// account that will receive permissions to alter hot routes and set the
/// developer address that will be receiving a share of profits from the module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetProtoRevAdminAccountProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryGetProtoRevNumberOfTradesRequest is request type for the
/// Query/GetProtoRevNumberOfTrades RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevNumberOfTradesRequest {}
/// QueryGetProtoRevNumberOfTradesResponse is response type for the
/// Query/GetProtoRevNumberOfTrades RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevNumberOfTradesResponse {
    /// number_of_trades is the number of trades the module has executed
    #[prost(string, tag = "1")]
    pub number_of_trades: ::prost::alloc::string::String,
}
/// QueryGetProtoRevProfitsByDenomRequest is request type for the
/// Query/GetProtoRevProfitsByDenom RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevProfitsByDenomRequest {
    /// denom is the denom to query profits by
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryGetProtoRevProfitsByDenomResponse is response type for the
/// Query/GetProtoRevProfitsByDenom RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevProfitsByDenomResponse {
    /// profit is the profits of the module by the selected denom
    #[prost(message, optional, tag = "1")]
    pub profit: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// QueryGetProtoRevAllProfitsRequest is request type for the
/// Query/GetProtoRevAllProfits RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevAllProfitsRequest {}
/// QueryGetProtoRevAllProfitsResponse is response type for the
/// Query/GetProtoRevAllProfits RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevAllProfitsResponse {
    /// profits is a list of all of the profits from the module
    #[prost(message, repeated, tag = "1")]
    pub profits: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// QueryGetProtoRevStatisticsByPoolRequest is request type for the
/// Query/GetProtoRevStatisticsByRoute RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevStatisticsByRouteRequest {
    /// route is the set of pool ids to query statistics by i.e. 1,2,3
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub route: ::prost::alloc::vec::Vec<u64>,
}
/// QueryGetProtoRevStatisticsByRouteResponse is response type for the
/// Query/GetProtoRevStatisticsByRoute RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevStatisticsByRouteResponse {
    /// statistics contains the number of trades the module has executed after a
    /// swap on a given pool and the profits from the trades
    #[prost(message, optional, tag = "1")]
    pub statistics: ::core::option::Option<RouteStatistics>,
}
/// QueryGetProtoRevAllRouteStatisticsRequest is request type for the
/// Query/GetProtoRevAllRouteStatistics RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevAllRouteStatisticsRequest {}
/// QueryGetProtoRevAllRouteStatisticsResponse is response type for the
/// Query/GetProtoRevAllRouteStatistics RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevAllRouteStatisticsResponse {
    /// statistics contains the number of trades/profits the module has executed on
    /// all routes it has successfully executed a trade on
    #[prost(message, repeated, tag = "1")]
    pub statistics: ::prost::alloc::vec::Vec<RouteStatistics>,
}
/// QueryGetProtoRevTokenPairArbRoutesRequest is request type for the
/// Query/GetProtoRevTokenPairArbRoutes RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevTokenPairArbRoutesRequest {}
/// QueryGetProtoRevTokenPairArbRoutesResponse is response type for the
/// Query/GetProtoRevTokenPairArbRoutes RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevTokenPairArbRoutesResponse {
    /// routes is a list of all of the hot routes that the module is currently
    /// arbitraging
    #[prost(message, repeated, tag = "1")]
    pub routes: ::prost::alloc::vec::Vec<TokenPairArbRoutes>,
}
/// QueryGetProtoRevAdminAccountRequest is request type for the
/// Query/GetProtoRevAdminAccount RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevAdminAccountRequest {}
/// QueryGetProtoRevAdminAccountResponse is response type for the
/// Query/GetProtoRevAdminAccount RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevAdminAccountResponse {
    /// admin_account is the admin account of the module
    #[prost(string, tag = "1")]
    pub admin_account: ::prost::alloc::string::String,
}
/// QueryGetProtoRevDeveloperAccountRequest is request type for the
/// Query/GetProtoRevDeveloperAccount RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevDeveloperAccountRequest {}
/// QueryGetProtoRevDeveloperAccountResponse is response type for the
/// Query/GetProtoRevDeveloperAccount RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevDeveloperAccountResponse {
    /// developer_account is the developer account of the module
    #[prost(string, tag = "1")]
    pub developer_account: ::prost::alloc::string::String,
}
/// QueryGetProtoRevInfoByPoolTypeRequest is request type for the
/// Query/GetProtoRevInfoByPoolType RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevInfoByPoolTypeRequest {}
/// QueryGetProtoRevInfoByPoolTypeResponse is response type for the
/// Query/GetProtoRevInfoByPoolType RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevInfoByPoolTypeResponse {
    /// InfoByPoolType contains all information pertaining to how different
    /// pool types are handled by the module.
    #[prost(message, optional, tag = "1")]
    pub info_by_pool_type: ::core::option::Option<InfoByPoolType>,
}
/// QueryGetProtoRevMaxPoolPointsPerBlockRequest is request type for the
/// Query/GetProtoRevMaxPoolPointsPerBlock RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevMaxPoolPointsPerBlockRequest {}
/// QueryGetProtoRevMaxPoolPointsPerBlockResponse is response type for the
/// Query/GetProtoRevMaxPoolPointsPerBlock RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevMaxPoolPointsPerBlockResponse {
    /// max_pool_points_per_block is the maximum number of pool points that can be
    /// consumed per block
    #[prost(uint64, tag = "1")]
    pub max_pool_points_per_block: u64,
}
/// QueryGetProtoRevMaxPoolPointsPerTxRequest is request type for the
/// Query/GetProtoRevMaxPoolPointsPerTx RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevMaxPoolPointsPerTxRequest {}
/// QueryGetProtoRevMaxPoolPointsPerTxResponse is response type for the
/// Query/GetProtoRevMaxPoolPointsPerTx RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevMaxPoolPointsPerTxResponse {
    /// max_pool_points_per_tx is the maximum number of pool points that can be
    /// consumed per transaction
    #[prost(uint64, tag = "1")]
    pub max_pool_points_per_tx: u64,
}
/// QueryGetProtoRevBaseDenomsRequest is request type for the
/// Query/GetProtoRevBaseDenoms RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevBaseDenomsRequest {}
/// QueryGetProtoRevBaseDenomsResponse is response type for the
/// Query/GetProtoRevBaseDenoms RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevBaseDenomsResponse {
    /// base_denoms is a list of all of the base denoms and step sizes
    #[prost(message, repeated, tag = "1")]
    pub base_denoms: ::prost::alloc::vec::Vec<BaseDenom>,
}
/// QueryGetProtoRevEnabledRequest is request type for the
/// Query/GetProtoRevEnabled RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevEnabledRequest {}
/// QueryGetProtoRevEnabledResponse is response type for the
/// Query/GetProtoRevEnabled RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevEnabledResponse {
    /// enabled is whether the module is enabled
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// QueryGetProtoRevPoolRequest is request type for the
/// Query/GetProtoRevPool RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevPoolRequest {
    /// base_denom is the base denom set in protorev for the denom pair to pool
    /// mapping
    #[prost(string, tag = "1")]
    pub base_denom: ::prost::alloc::string::String,
    /// other_denom is the other denom for the denom pair to pool mapping
    #[prost(string, tag = "2")]
    pub other_denom: ::prost::alloc::string::String,
}
/// QueryGetProtoRevPoolResponse is response type for the
/// Query/GetProtoRevPool RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetProtoRevPoolResponse {
    /// pool_id is the pool_id stored for the denom pair
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetAllProtocolRevenueRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetAllProtocolRevenueResponse {
    #[prost(message, optional, tag = "1")]
    pub all_protocol_revenue: ::core::option::Option<AllProtocolRevenue>,
}
/// MsgSetHotRoutes defines the Msg/SetHotRoutes request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetHotRoutes {
    /// admin is the account that is authorized to set the hot routes.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// hot_routes is the list of hot routes to set.
    #[prost(message, repeated, tag = "2")]
    pub hot_routes: ::prost::alloc::vec::Vec<TokenPairArbRoutes>,
}
/// MsgSetHotRoutesResponse defines the Msg/SetHotRoutes response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetHotRoutesResponse {}
/// MsgSetDeveloperAccount defines the Msg/SetDeveloperAccount request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetDeveloperAccount {
    /// admin is the account that is authorized to set the developer account.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// developer_account is the account that will receive a portion of the profits
    /// from the protorev module.
    #[prost(string, tag = "2")]
    pub developer_account: ::prost::alloc::string::String,
}
/// MsgSetDeveloperAccountResponse defines the Msg/SetDeveloperAccount response
/// type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetDeveloperAccountResponse {}
/// MsgSetInfoByPoolType defines the Msg/SetInfoByPoolType request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetInfoByPoolType {
    /// admin is the account that is authorized to set the pool weights.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// info_by_pool_type contains information about the pool types.
    #[prost(message, optional, tag = "2")]
    pub info_by_pool_type: ::core::option::Option<InfoByPoolType>,
}
/// MsgSetInfoByPoolTypeResponse defines the Msg/SetInfoByPoolType response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetInfoByPoolTypeResponse {}
/// MsgSetMaxPoolPointsPerTx defines the Msg/SetMaxPoolPointsPerTx request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetMaxPoolPointsPerTx {
    /// admin is the account that is authorized to set the max pool points per tx.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// max_pool_points_per_tx is the maximum number of pool points that can be
    /// consumed per transaction.
    #[prost(uint64, tag = "2")]
    pub max_pool_points_per_tx: u64,
}
/// MsgSetMaxPoolPointsPerTxResponse defines the Msg/SetMaxPoolPointsPerTx
/// response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetMaxPoolPointsPerTxResponse {}
/// MsgSetMaxPoolPointsPerBlock defines the Msg/SetMaxPoolPointsPerBlock request
/// type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetMaxPoolPointsPerBlock {
    /// admin is the account that is authorized to set the max pool points per
    /// block.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// max_pool_points_per_block is the maximum number of pool points that can be
    /// consumed per block.
    #[prost(uint64, tag = "2")]
    pub max_pool_points_per_block: u64,
}
/// MsgSetMaxPoolPointsPerBlockResponse defines the
/// Msg/SetMaxPoolPointsPerBlock response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetMaxPoolPointsPerBlockResponse {}
/// MsgSetBaseDenoms defines the Msg/SetBaseDenoms request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetBaseDenoms {
    /// admin is the account that is authorized to set the base denoms.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// base_denoms is the list of base denoms to set.
    #[prost(message, repeated, tag = "2")]
    pub base_denoms: ::prost::alloc::vec::Vec<BaseDenom>,
}
/// MsgSetBaseDenomsResponse defines the Msg/SetBaseDenoms response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetBaseDenomsResponse {}
include!("osmosis.protorev.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
