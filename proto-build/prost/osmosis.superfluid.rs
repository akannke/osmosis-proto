// @generated
/// SuperfluidAsset stores the pair of superfluid asset type and denom pair
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidAsset {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// AssetType indicates whether the superfluid asset is a native token or an lp
    /// share
    #[prost(enumeration="SuperfluidAssetType", tag="2")]
    pub asset_type: i32,
}
/// SuperfluidIntermediaryAccount takes the role of intermediary between LP token
/// and OSMO tokens for superfluid staking. The intermediary account is the
/// actual account responsible for delegation, not the validator account itself.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidIntermediaryAccount {
    /// Denom indicates the denom of the superfluid asset.
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub val_addr: ::prost::alloc::string::String,
    /// perpetual gauge for rewards distribution
    #[prost(uint64, tag="3")]
    pub gauge_id: u64,
}
/// The Osmo-Equivalent-Multiplier Record for epoch N refers to the osmo worth we
/// treat an LP share as having, for all of epoch N. Eventually this is intended
/// to be set as the Time-weighted-average-osmo-backing for the entire duration
/// of epoch N-1. (Thereby locking what's in use for epoch N as based on the
/// prior epochs rewards) However for now, this is not the TWAP but instead the
/// spot price at the boundary. For different types of assets in the future, it
/// could change.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsmoEquivalentMultiplierRecord {
    #[prost(int64, tag="1")]
    pub epoch_number: i64,
    /// superfluid asset denom, can be LP token or native token
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub multiplier: ::prost::alloc::string::String,
}
/// SuperfluidDelegationRecord is a struct used to indicate superfluid
/// delegations of an account in the state machine in a user friendly form.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationRecord {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub delegation_amount: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="4")]
    pub equivalent_staked_amount: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// LockIdIntermediaryAccountConnection is a struct used to indicate the
/// relationship between the underlying lock id and superfluid delegation done
/// via lp shares.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockIdIntermediaryAccountConnection {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
    #[prost(string, tag="2")]
    pub intermediary_account: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnpoolWhitelistedPools {
    #[prost(uint64, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConcentratedPoolUserPositionRecord {
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub position_id: u64,
    #[prost(uint64, tag="3")]
    pub lock_id: u64,
    #[prost(message, optional, tag="4")]
    pub synthetic_lock: ::core::option::Option<super::lockup::SyntheticLock>,
    #[prost(message, optional, tag="5")]
    pub delegation_amount: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="6")]
    pub equivalent_staked_amount: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// SuperfluidAssetType indicates whether the superfluid asset is
/// a native token, lp share of a pool, or concentrated share of a pool
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SuperfluidAssetType {
    Native = 0,
    LpShare = 1,
    /// SuperfluidAssetTypeLendingShare = 3; // for now not exist
    ConcentratedShare = 2,
}
impl SuperfluidAssetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SuperfluidAssetType::Native => "SuperfluidAssetTypeNative",
            SuperfluidAssetType::LpShare => "SuperfluidAssetTypeLPShare",
            SuperfluidAssetType::ConcentratedShare => "SuperfluidAssetTypeConcentratedShare",
        }
    }
}
/// Params holds parameters for the superfluid module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// minimum_risk_factor is to be cut on OSMO equivalent value of lp tokens for
    /// superfluid staking, default: 5%. The minimum risk factor works
    /// to counter-balance the staked amount on chain's exposure to various asset
    /// volatilities, and have base staking be 'resistant' to volatility.
    #[prost(string, tag="1")]
    pub minimum_risk_factor: ::prost::alloc::string::String,
}
/// GenesisState defines the module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// superfluid_assets defines the registered superfluid assets that have been
    /// registered via governance.
    #[prost(message, repeated, tag="2")]
    pub superfluid_assets: ::prost::alloc::vec::Vec<SuperfluidAsset>,
    /// osmo_equivalent_multipliers is the records of osmo equivalent amount of
    /// each superfluid registered pool, updated every epoch.
    #[prost(message, repeated, tag="3")]
    pub osmo_equivalent_multipliers: ::prost::alloc::vec::Vec<OsmoEquivalentMultiplierRecord>,
    /// intermediary_accounts is a secondary account for superfluid staking that
    /// plays an intermediary role between validators and the delegators.
    #[prost(message, repeated, tag="4")]
    pub intermediary_accounts: ::prost::alloc::vec::Vec<SuperfluidIntermediaryAccount>,
    #[prost(message, repeated, tag="5")]
    pub intemediary_account_connections: ::prost::alloc::vec::Vec<LockIdIntermediaryAccountConnection>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetTypeRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetTypeResponse {
    #[prost(enumeration="SuperfluidAssetType", tag="1")]
    pub asset_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllAssetsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllAssetsResponse {
    #[prost(message, repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<SuperfluidAsset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetMultiplierRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetMultiplierResponse {
    #[prost(message, optional, tag="1")]
    pub osmo_equivalent_multiplier: ::core::option::Option<OsmoEquivalentMultiplierRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidIntermediaryAccountInfo {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub val_addr: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub gauge_id: u64,
    #[prost(string, tag="4")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllIntermediaryAccountsRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllIntermediaryAccountsResponse {
    #[prost(message, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<SuperfluidIntermediaryAccountInfo>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectedIntermediaryAccountRequest {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectedIntermediaryAccountResponse {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<SuperfluidIntermediaryAccountInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalDelegationByValidatorForDenomRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalDelegationByValidatorForDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<Delegations>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Delegations {
    #[prost(string, tag="1")]
    pub val_addr: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub amount_sfsd: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub osmo_equivalent: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalSuperfluidDelegationsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalSuperfluidDelegationsResponse {
    #[prost(string, tag="1")]
    pub total_delegations: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationAmountRequest {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationAmountResponse {
    #[prost(message, repeated, tag="1")]
    pub amount: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationsByDelegatorRequest {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationsByDelegatorResponse {
    #[prost(message, repeated, tag="1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
    #[prost(message, repeated, tag="2")]
    pub total_delegated_coins: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="3")]
    pub total_equivalent_staked_amount: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidUndelegationsByDelegatorRequest {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidUndelegationsByDelegatorResponse {
    #[prost(message, repeated, tag="1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
    #[prost(message, repeated, tag="2")]
    pub total_undelegated_coins: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag="3")]
    pub synthetic_locks: ::prost::alloc::vec::Vec<super::lockup::SyntheticLock>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationsByValidatorDenomRequest {
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperfluidDelegationsByValidatorDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSuperfluidDelegatedAmountByValidatorDenomRequest {
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateSuperfluidDelegatedAmountByValidatorDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub total_delegated_coins: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalDelegationByDelegatorRequest {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalDelegationByDelegatorResponse {
    #[prost(message, repeated, tag="1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
    #[prost(message, repeated, tag="2")]
    pub delegation_response: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::staking::v1beta1::DelegationResponse>,
    #[prost(message, repeated, tag="3")]
    pub total_delegated_coins: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="4")]
    pub total_equivalent_staked_amount: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnpoolWhitelistRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnpoolWhitelistResponse {
    #[prost(uint64, repeated, tag="1")]
    pub pool_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserConcentratedSuperfluidPositionsDelegatedRequest {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserConcentratedSuperfluidPositionsDelegatedResponse {
    #[prost(message, repeated, tag="1")]
    pub cl_pool_user_position_records: ::prost::alloc::vec::Vec<ConcentratedPoolUserPositionRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserConcentratedSuperfluidPositionsUndelegatingRequest {
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserConcentratedSuperfluidPositionsUndelegatingResponse {
    #[prost(message, repeated, tag="1")]
    pub cl_pool_user_position_records: ::prost::alloc::vec::Vec<ConcentratedPoolUserPositionRecord>,
}
/// THIS QUERY IS TEMPORARY
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRestSupplyRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRestSupplyResponse {
    /// amount is the supply of the coin.
    #[prost(message, optional, tag="1")]
    pub amount: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidDelegate {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
    #[prost(string, tag="3")]
    pub val_addr: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidDelegateResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUndelegate {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUndelegateResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUnbondLock {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUnbondLockResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUndelegateAndUnbondLock {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub lock_id: u64,
    /// Amount of unlocking coin.
    #[prost(message, optional, tag="3")]
    pub coin: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSuperfluidUndelegateAndUnbondLockResponse {
    /// lock id of the new lock created for the remaining amount.
    /// returns the original lockid if the unlocked amount is equal to the
    /// original lock's amount.
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
}
// message MsgSuperfluidRedelegate {
//    string sender = 1 [ (gogoproto.moretags) = "yaml:\"sender\"" ];
//    uint64 lock_id = 2;
//    string new_val_addr = 3;
// }
// message MsgSuperfluidRedelegateResponse {}

/// MsgLockAndSuperfluidDelegate locks coins with the unbonding period duration,
/// and then does a superfluid lock from the newly created lockup, to the
/// specified validator addr.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockAndSuperfluidDelegate {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub coins: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="3")]
    pub val_addr: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockAndSuperfluidDelegateResponse {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
/// MsgCreateFullRangePositionAndSuperfluidDelegate creates a full range position
/// in a concentrated liquidity pool, then superfluid delegates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateFullRangePositionAndSuperfluidDelegate {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub coins: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="3")]
    pub val_addr: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateFullRangePositionAndSuperfluidDelegateResponse {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
    #[prost(uint64, tag="2")]
    pub position_id: u64,
}
/// MsgUnPoolWhitelistedPool Unpools every lock the sender has, that is
/// associated with pool pool_id. If pool_id is not approved for unpooling by
/// governance, this is a no-op. Unpooling takes the locked gamm shares, and runs
/// "ExitPool" on it, to get the constituent tokens. e.g. z gamm/pool/1 tokens
/// ExitPools into constituent tokens x uatom, y uosmo. Then it creates a new
/// lock for every constituent token, with the duration associated with the lock.
/// If the lock was unbonding, the new lockup durations should be the time left
/// until unbond completion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnPoolWhitelistedPool {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnPoolWhitelistedPoolResponse {
    #[prost(uint64, repeated, tag="1")]
    pub exited_lock_ids: ::prost::alloc::vec::Vec<u64>,
}
/// =====================
/// MsgUnlockAndMigrateSharesToFullRangeConcentratedPosition
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnlockAndMigrateSharesToFullRangeConcentratedPosition {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub lock_id: i64,
    #[prost(message, optional, tag="3")]
    pub shares_to_migrate: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// token_out_mins indicates minimum token to exit Balancer pool with.
    #[prost(message, repeated, tag="4")]
    pub token_out_mins: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnlockAndMigrateSharesToFullRangeConcentratedPositionResponse {
    #[prost(string, tag="1")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub liquidity_created: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub join_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// ===================== MsgAddToConcentratedLiquiditySuperfluidPosition
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToConcentratedLiquiditySuperfluidPosition {
    #[prost(uint64, tag="1")]
    pub position_id: u64,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub token_desired0: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="4")]
    pub token_desired1: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToConcentratedLiquiditySuperfluidPositionResponse {
    #[prost(uint64, tag="1")]
    pub position_id: u64,
    #[prost(string, tag="2")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub amount1: ::prost::alloc::string::String,
    /// new_liquidity is the final liquidity after the add.
    /// It includes the liquidity that existed before in the position
    /// and the new liquidity that was added to the position.
    #[prost(string, tag="5")]
    pub new_liquidity: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub lock_id: u64,
}
/// ===================== MsgUnbondConvertAndStake
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnbondConvertAndStake {
    /// lock ID to convert and stake.
    /// lock id with 0 should be provided if converting liquid gamm shares to stake
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    /// validator address to delegate to.
    /// If provided empty string, we use the validators returned from
    /// valset-preference module.
    #[prost(string, tag="3")]
    pub val_addr: ::prost::alloc::string::String,
    /// min_amt_to_stake indicates the minimum amount to stake after conversion
    #[prost(string, tag="4")]
    pub min_amt_to_stake: ::prost::alloc::string::String,
    /// shares_to_convert indicates shares wanted to stake.
    /// Note that this field is only used for liquid(unlocked) gamm shares.
    /// For all other cases, this field would be disregarded.
    #[prost(message, optional, tag="5")]
    pub shares_to_convert: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnbondConvertAndStakeResponse {
    #[prost(string, tag="1")]
    pub total_amt_staked: ::prost::alloc::string::String,
}
include!("osmosis.superfluid.tonic.rs");
// @@protoc_insertion_point(module)