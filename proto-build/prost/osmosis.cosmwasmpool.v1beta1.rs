// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// code_ide_whitelist contains the list of code ids that are allowed to be
    /// instantiated.
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub code_id_whitelist: ::prost::alloc::vec::Vec<u64>,
    /// pool_migration_limit is the maximum number of pools that can be migrated
    /// at once via governance proposal. This is to have a constant bound on the
    /// number of pools that can be migrated at once and remove the possibility
    /// of an unlikely scenario of causing a chain halt due to a large migration.
    #[prost(uint64, tag="2")]
    pub pool_migration_limit: u64,
}
/// GenesisState defines the cosmwasmpool module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params is the container of cosmwasmpool parameters.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag="2")]
    pub pools: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// UploadCosmWasmPoolCodeAndWhiteListProposal is a gov Content type for
/// uploading coswasm pool code and adding it to internal whitelist. Only the
/// code ids created by this message are eligible for being x/cosmwasmpool pools.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadCosmWasmPoolCodeAndWhiteListProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes="vec", tag="3")]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
}
/// MigratePoolContractsProposal is a gov Content type for
/// migrating  given pools to the new contract code and adding to internal
/// whitelist if needed. It has two options to perform the migration:
///
/// 1. If the codeID is non-zero, it will migrate the pool contracts to a given
/// codeID assuming that it has already been uploaded. uploadByteCode must be
/// empty in such a case. Fails if codeID does not exist. Fails if uploadByteCode
/// is not empty.
///
/// 2. If the codeID is zero, it will upload the given uploadByteCode and use the
/// new resulting code id to migrate the pool to. Errors if uploadByteCode is
/// empty or invalid.
///
/// In both cases, if one of the pools specified by the given poolID does not
/// exist, the proposal fails.
///
/// The reason for having poolIDs be a slice of ids is to account for the
/// potential need for emergency migration of all old code ids associated with
/// particular pools to new code ids, or simply having the flexibility of
/// migrating multiple older pool contracts to a new one at once when there is a
/// release.
///
/// poolD count to be submitted at once is gated by a governance paramets (20 at
/// launch). The proposal fails if more. Note that 20 was chosen arbitrarily to
/// have a constant bound on the number of pools migrated at once. This size will
/// be configured by a module parameter so it can be changed by a constant.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigratePoolContractsProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// pool_ids are the pool ids of the contracts to be migrated
    /// either to the new_code_id that is already uploaded to chain or to
    /// the given wasm_byte_code.
    #[prost(uint64, repeated, tag="3")]
    pub pool_ids: ::prost::alloc::vec::Vec<u64>,
    /// new_code_id is the code id of the contract code to migrate to.
    /// Assumes that the code is already uploaded to chain. Only one of
    /// new_code_id and wasm_byte_code should be set.
    #[prost(uint64, tag="4")]
    pub new_code_id: u64,
    /// WASMByteCode can be raw or gzip compressed. Assumes that the code id
    /// has not been uploaded yet so uploads the given code and migrates to it.
    /// Only one of new_code_id and wasm_byte_code should be set.
    #[prost(bytes="vec", tag="5")]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    /// MigrateMsg migrate message to be used for migrating the pool contracts.
    #[prost(bytes="vec", tag="6")]
    pub migrate_msg: ::prost::alloc::vec::Vec<u8>,
}
/// ===================== InstantiateMsg
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstantiateMsg {
    /// pool_asset_denoms is the list of asset denoms that are initialized
    /// at pool creation time.
    #[prost(string, repeated, tag="1")]
    pub pool_asset_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ===================== CalcOutAmtGivenIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalcOutAmtGivenIn {
    /// token_in is the token to be sent to the pool.
    #[prost(message, optional, tag="1")]
    pub token_in: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// token_out_denom is the token denom to be received from the pool.
    #[prost(string, tag="2")]
    pub token_out_denom: ::prost::alloc::string::String,
    /// swap_fee is the swap fee for this swap estimate.
    #[prost(string, tag="3")]
    pub swap_fee: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalcOutAmtGivenInRequest {
    /// calc_out_amt_given_in is the structure containing all the request
    /// information for this query.
    #[prost(message, optional, tag="1")]
    pub calc_out_amt_given_in: ::core::option::Option<CalcOutAmtGivenIn>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalcOutAmtGivenInResponse {
    /// token_out is the token out computed from this swap estimate call.
    #[prost(message, optional, tag="1")]
    pub token_out: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// ===================== CalcInAmtGivenOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalcInAmtGivenOut {
    /// token_out is the token out to be receoved from the pool.
    #[prost(message, optional, tag="1")]
    pub token_out: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// token_in_denom is the token denom to be sentt to the pool.
    #[prost(string, tag="2")]
    pub token_in_denom: ::prost::alloc::string::String,
    /// swap_fee is the swap fee for this swap estimate.
    #[prost(string, tag="3")]
    pub swap_fee: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalcInAmtGivenOutRequest {
    /// calc_in_amt_given_out is the structure containing all the request
    /// information for this query.
    #[prost(message, optional, tag="1")]
    pub calc_in_amt_given_out: ::core::option::Option<CalcInAmtGivenOut>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalcInAmtGivenOutResponse {
    /// token_in is the token in computed from this swap estimate call.
    #[prost(message, optional, tag="1")]
    pub token_in: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// ===================== SwapExactAmountIn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapExactAmountIn {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// token_in is the token to be sent to the pool.
    #[prost(message, optional, tag="2")]
    pub token_in: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// token_out_denom is the token denom to be received from the pool.
    #[prost(string, tag="3")]
    pub token_out_denom: ::prost::alloc::string::String,
    /// token_out_min_amount is the minimum amount of token_out to be received from
    /// the pool.
    #[prost(string, tag="4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
    /// swap_fee is the swap fee for this swap estimate.
    #[prost(string, tag="5")]
    pub swap_fee: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapExactAmountInSudoMsg {
    /// swap_exact_amount_in is the structure containing all the request
    /// information for this message.
    #[prost(message, optional, tag="1")]
    pub swap_exact_amount_in: ::core::option::Option<SwapExactAmountIn>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapExactAmountInSudoMsgResponse {
    /// token_out_amount is the token out computed from this swap estimate call.
    #[prost(string, tag="1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== SwapExactAmountOut
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapExactAmountOut {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// token_out is the token to be sent out of the pool.
    #[prost(message, optional, tag="2")]
    pub token_out: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// token_in_denom is the token denom to be sent too the pool.
    #[prost(string, tag="3")]
    pub token_in_denom: ::prost::alloc::string::String,
    /// token_in_max_amount is the maximum amount of token_in to be sent to the
    /// pool.
    #[prost(string, tag="4")]
    pub token_in_max_amount: ::prost::alloc::string::String,
    /// swap_fee is the swap fee for this swap estimate.
    #[prost(string, tag="5")]
    pub swap_fee: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapExactAmountOutSudoMsg {
    /// swap_exact_amount_out is the structure containing all the request
    /// information for this message.
    #[prost(message, optional, tag="1")]
    pub swap_exact_amount_out: ::core::option::Option<SwapExactAmountOut>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapExactAmountOutSudoMsgResponse {
    /// token_in_amount is the token in computed from this swap estimate call.
    #[prost(string, tag="1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// CosmWasmPool represents the data serialized into state for each CW pool.
///
/// Note: CW Pool has 2 pool models:
/// - CosmWasmPool which is a proto-generated store model used for serialization
/// into state.
/// - Pool struct that encapsulates the CosmWasmPool and wasmKeeper for calling
/// the contract.
///
/// CosmWasmPool implements the poolmanager.PoolI interface but it panics on all
/// methods. The reason is that access to wasmKeeper is required to call the
/// contract.
///
/// Instead, all interactions and poolmanager.PoolI methods are to be performed
/// on the Pool struct. The reason why we cannot have a Pool struct only is
/// because it cannot be serialized into state due to having a non-serializable
/// wasmKeeper field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmWasmPool {
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
    #[prost(uint64, tag="3")]
    pub code_id: u64,
    #[prost(bytes="vec", tag="4")]
    pub instantiate_msg: ::prost::alloc::vec::Vec<u8>,
}
/// ===================== GetSwapFeeQueryMsg
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSwapFeeQueryMsg {
    /// get_swap_fee is the query structure to get swap fee.
    #[prost(message, optional, tag="1")]
    pub get_swap_fee: ::core::option::Option<EmptyStruct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSwapFeeQueryMsgResponse {
    /// swap_fee is the swap fee for this swap estimate.
    #[prost(string, tag="3")]
    pub swap_fee: ::prost::alloc::string::String,
}
/// ===================== SpotPriceQueryMsg
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotPrice {
    /// quote_asset_denom is the quote asset of the spot query.
    #[prost(string, tag="1")]
    pub quote_asset_denom: ::prost::alloc::string::String,
    /// base_asset_denom is the base asset of the spot query.
    #[prost(string, tag="2")]
    pub base_asset_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotPriceQueryMsg {
    /// spot_price is the structure containing request field of the spot price
    /// query message.
    #[prost(message, optional, tag="1")]
    pub spot_price: ::core::option::Option<SpotPrice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotPriceQueryMsgResponse {
    /// spot_price is the spot price returned.
    #[prost(string, tag="1")]
    pub spot_price: ::prost::alloc::string::String,
}
/// ===================== GetTotalPoolLiquidityQueryMsg
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyStruct {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTotalPoolLiquidityQueryMsg {
    /// get_total_pool_liquidity is the structure containing request field of the
    /// total pool liquidity query message.
    #[prost(message, optional, tag="1")]
    pub get_total_pool_liquidity: ::core::option::Option<EmptyStruct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTotalPoolLiquidityQueryMsgResponse {
    ///   total_pool_liquidity is the total liquidity in the pool denominated in
    ///   coins.
    #[prost(message, repeated, tag="1")]
    pub total_pool_liquidity: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// ===================== GetTotalSharesQueryMsg
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTotalSharesQueryMsg {
    /// get_total_shares is the structure containing request field of the
    /// total shares query message.
    #[prost(message, optional, tag="1")]
    pub get_total_shares: ::core::option::Option<EmptyStruct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTotalSharesQueryMsgResponse {
    /// total_shares is the amount of shares returned.
    #[prost(string, tag="1")]
    pub total_shares: ::prost::alloc::string::String,
}
/// ===================== JoinPoolExecuteMsg
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinPoolExecuteMsgRequest {
    /// join_pool is the structure containing all request fields of the join pool
    /// execute message.
    #[prost(message, optional, tag="1")]
    pub join_pool: ::core::option::Option<EmptyRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinPoolExecuteMsgResponse {
}
/// ===================== ExitPoolExecuteMsg
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExitPoolExecuteMsgRequest {
    /// exit_pool is the structure containing all request fields of the exit pool
    /// execute message.
    #[prost(message, optional, tag="1")]
    pub exit_pool: ::core::option::Option<EmptyRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExitPoolExecuteMsgResponse {
}
/// ===================== MsgCreateCosmwasmPool
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateCosmWasmPool {
    #[prost(uint64, tag="1")]
    pub code_id: u64,
    #[prost(bytes="vec", tag="2")]
    pub instantiate_msg: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub sender: ::prost::alloc::string::String,
}
/// Returns a unique poolID to identify the pool with.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateCosmWasmPoolResponse {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}
/// =============================== ContractInfoByPoolId
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsResponse {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
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
/// =============================== ContractInfoByPoolId
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractInfoByPoolIdRequest {
    /// pool_id is the pool id of the requested pool.
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractInfoByPoolIdResponse {
    /// contract_address is the pool address and contract address
    /// of the requested pool id.
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// code_id is the code id of the requested pool id.
    #[prost(uint64, tag="2")]
    pub code_id: u64,
}
include!("osmosis.cosmwasmpool.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)