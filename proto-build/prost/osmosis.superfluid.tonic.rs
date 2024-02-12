// @generated
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryParamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/Params",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.superfluid.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn asset_type(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AssetTypeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/AssetType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.superfluid.Query", "AssetType"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn all_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::AllAssetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllAssetsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/AllAssets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.superfluid.Query", "AllAssets"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn asset_multiplier(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetMultiplierRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AssetMultiplierResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/AssetMultiplier",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.superfluid.Query", "AssetMultiplier"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn all_intermediary_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::AllIntermediaryAccountsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllIntermediaryAccountsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/AllIntermediaryAccounts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Query",
                        "AllIntermediaryAccounts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn connected_intermediary_account(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectedIntermediaryAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConnectedIntermediaryAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/ConnectedIntermediaryAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Query",
                        "ConnectedIntermediaryAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn total_delegation_by_validator_for_denom(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryTotalDelegationByValidatorForDenomRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryTotalDelegationByValidatorForDenomResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/TotalDelegationByValidatorForDenom",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Query",
                        "TotalDelegationByValidatorForDenom",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn total_superfluid_delegations(
            &mut self,
            request: impl tonic::IntoRequest<super::TotalSuperfluidDelegationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TotalSuperfluidDelegationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/TotalSuperfluidDelegations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Query",
                        "TotalSuperfluidDelegations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn superfluid_delegation_amount(
            &mut self,
            request: impl tonic::IntoRequest<super::SuperfluidDelegationAmountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SuperfluidDelegationAmountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/SuperfluidDelegationAmount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Query",
                        "SuperfluidDelegationAmount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn superfluid_delegations_by_delegator(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SuperfluidDelegationsByDelegatorRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SuperfluidDelegationsByDelegatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/SuperfluidDelegationsByDelegator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Query",
                        "SuperfluidDelegationsByDelegator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn superfluid_undelegations_by_delegator(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SuperfluidUndelegationsByDelegatorRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SuperfluidUndelegationsByDelegatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/SuperfluidUndelegationsByDelegator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Query",
                        "SuperfluidUndelegationsByDelegator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn superfluid_delegations_by_validator_denom(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SuperfluidDelegationsByValidatorDenomRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SuperfluidDelegationsByValidatorDenomResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/SuperfluidDelegationsByValidatorDenom",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Query",
                        "SuperfluidDelegationsByValidatorDenom",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_superfluid_delegated_amount_by_validator_denom(
            &mut self,
            request: impl tonic::IntoRequest<
                super::EstimateSuperfluidDelegatedAmountByValidatorDenomRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::EstimateSuperfluidDelegatedAmountByValidatorDenomResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/EstimateSuperfluidDelegatedAmountByValidatorDenom",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Query",
                        "EstimateSuperfluidDelegatedAmountByValidatorDenom",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn total_delegation_by_delegator(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryTotalDelegationByDelegatorRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryTotalDelegationByDelegatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/TotalDelegationByDelegator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Query",
                        "TotalDelegationByDelegator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unpool_whitelist(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUnpoolWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUnpoolWhitelistResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/UnpoolWhitelist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.superfluid.Query", "UnpoolWhitelist"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn user_concentrated_superfluid_positions_delegated(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UserConcentratedSuperfluidPositionsDelegatedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UserConcentratedSuperfluidPositionsDelegatedResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/UserConcentratedSuperfluidPositionsDelegated",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Query",
                        "UserConcentratedSuperfluidPositionsDelegated",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn user_concentrated_superfluid_positions_undelegating(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UserConcentratedSuperfluidPositionsUndelegatingRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::UserConcentratedSuperfluidPositionsUndelegatingResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/UserConcentratedSuperfluidPositionsUndelegating",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Query",
                        "UserConcentratedSuperfluidPositionsUndelegating",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn rest_supply(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRestSupplyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryRestSupplyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Query/RestSupply",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.superfluid.Query", "RestSupply"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryParamsResponse>,
            tonic::Status,
        >;
        async fn asset_type(
            &self,
            request: tonic::Request<super::AssetTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AssetTypeResponse>,
            tonic::Status,
        >;
        async fn all_assets(
            &self,
            request: tonic::Request<super::AllAssetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllAssetsResponse>,
            tonic::Status,
        >;
        async fn asset_multiplier(
            &self,
            request: tonic::Request<super::AssetMultiplierRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AssetMultiplierResponse>,
            tonic::Status,
        >;
        async fn all_intermediary_accounts(
            &self,
            request: tonic::Request<super::AllIntermediaryAccountsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllIntermediaryAccountsResponse>,
            tonic::Status,
        >;
        async fn connected_intermediary_account(
            &self,
            request: tonic::Request<super::ConnectedIntermediaryAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConnectedIntermediaryAccountResponse>,
            tonic::Status,
        >;
        async fn total_delegation_by_validator_for_denom(
            &self,
            request: tonic::Request<
                super::QueryTotalDelegationByValidatorForDenomRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryTotalDelegationByValidatorForDenomResponse>,
            tonic::Status,
        >;
        async fn total_superfluid_delegations(
            &self,
            request: tonic::Request<super::TotalSuperfluidDelegationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TotalSuperfluidDelegationsResponse>,
            tonic::Status,
        >;
        async fn superfluid_delegation_amount(
            &self,
            request: tonic::Request<super::SuperfluidDelegationAmountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SuperfluidDelegationAmountResponse>,
            tonic::Status,
        >;
        async fn superfluid_delegations_by_delegator(
            &self,
            request: tonic::Request<super::SuperfluidDelegationsByDelegatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SuperfluidDelegationsByDelegatorResponse>,
            tonic::Status,
        >;
        async fn superfluid_undelegations_by_delegator(
            &self,
            request: tonic::Request<super::SuperfluidUndelegationsByDelegatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SuperfluidUndelegationsByDelegatorResponse>,
            tonic::Status,
        >;
        async fn superfluid_delegations_by_validator_denom(
            &self,
            request: tonic::Request<super::SuperfluidDelegationsByValidatorDenomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SuperfluidDelegationsByValidatorDenomResponse>,
            tonic::Status,
        >;
        async fn estimate_superfluid_delegated_amount_by_validator_denom(
            &self,
            request: tonic::Request<
                super::EstimateSuperfluidDelegatedAmountByValidatorDenomRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::EstimateSuperfluidDelegatedAmountByValidatorDenomResponse,
            >,
            tonic::Status,
        >;
        async fn total_delegation_by_delegator(
            &self,
            request: tonic::Request<super::QueryTotalDelegationByDelegatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTotalDelegationByDelegatorResponse>,
            tonic::Status,
        >;
        async fn unpool_whitelist(
            &self,
            request: tonic::Request<super::QueryUnpoolWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUnpoolWhitelistResponse>,
            tonic::Status,
        >;
        async fn user_concentrated_superfluid_positions_delegated(
            &self,
            request: tonic::Request<
                super::UserConcentratedSuperfluidPositionsDelegatedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UserConcentratedSuperfluidPositionsDelegatedResponse>,
            tonic::Status,
        >;
        async fn user_concentrated_superfluid_positions_undelegating(
            &self,
            request: tonic::Request<
                super::UserConcentratedSuperfluidPositionsUndelegatingRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::UserConcentratedSuperfluidPositionsUndelegatingResponse,
            >,
            tonic::Status,
        >;
        async fn rest_supply(
            &self,
            request: tonic::Request<super::QueryRestSupplyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryRestSupplyResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/osmosis.superfluid.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest>
                    for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/AssetType" => {
                    #[allow(non_camel_case_types)]
                    struct AssetTypeSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AssetTypeRequest>
                    for AssetTypeSvc<T> {
                        type Response = super::AssetTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssetTypeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).asset_type(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssetTypeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/AllAssets" => {
                    #[allow(non_camel_case_types)]
                    struct AllAssetsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AllAssetsRequest>
                    for AllAssetsSvc<T> {
                        type Response = super::AllAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AllAssetsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).all_assets(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AllAssetsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/AssetMultiplier" => {
                    #[allow(non_camel_case_types)]
                    struct AssetMultiplierSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::AssetMultiplierRequest>
                    for AssetMultiplierSvc<T> {
                        type Response = super::AssetMultiplierResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssetMultiplierRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).asset_multiplier(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssetMultiplierSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/AllIntermediaryAccounts" => {
                    #[allow(non_camel_case_types)]
                    struct AllIntermediaryAccountsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::AllIntermediaryAccountsRequest>
                    for AllIntermediaryAccountsSvc<T> {
                        type Response = super::AllIntermediaryAccountsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AllIntermediaryAccountsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).all_intermediary_accounts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AllIntermediaryAccountsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/ConnectedIntermediaryAccount" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectedIntermediaryAccountSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::ConnectedIntermediaryAccountRequest,
                    > for ConnectedIntermediaryAccountSvc<T> {
                        type Response = super::ConnectedIntermediaryAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ConnectedIntermediaryAccountRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).connected_intermediary_account(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConnectedIntermediaryAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/TotalDelegationByValidatorForDenom" => {
                    #[allow(non_camel_case_types)]
                    struct TotalDelegationByValidatorForDenomSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryTotalDelegationByValidatorForDenomRequest,
                    > for TotalDelegationByValidatorForDenomSvc<T> {
                        type Response = super::QueryTotalDelegationByValidatorForDenomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryTotalDelegationByValidatorForDenomRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .total_delegation_by_validator_for_denom(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TotalDelegationByValidatorForDenomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/TotalSuperfluidDelegations" => {
                    #[allow(non_camel_case_types)]
                    struct TotalSuperfluidDelegationsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::TotalSuperfluidDelegationsRequest,
                    > for TotalSuperfluidDelegationsSvc<T> {
                        type Response = super::TotalSuperfluidDelegationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::TotalSuperfluidDelegationsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).total_superfluid_delegations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TotalSuperfluidDelegationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/SuperfluidDelegationAmount" => {
                    #[allow(non_camel_case_types)]
                    struct SuperfluidDelegationAmountSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::SuperfluidDelegationAmountRequest,
                    > for SuperfluidDelegationAmountSvc<T> {
                        type Response = super::SuperfluidDelegationAmountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SuperfluidDelegationAmountRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).superfluid_delegation_amount(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SuperfluidDelegationAmountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/SuperfluidDelegationsByDelegator" => {
                    #[allow(non_camel_case_types)]
                    struct SuperfluidDelegationsByDelegatorSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::SuperfluidDelegationsByDelegatorRequest,
                    > for SuperfluidDelegationsByDelegatorSvc<T> {
                        type Response = super::SuperfluidDelegationsByDelegatorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SuperfluidDelegationsByDelegatorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).superfluid_delegations_by_delegator(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SuperfluidDelegationsByDelegatorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/SuperfluidUndelegationsByDelegator" => {
                    #[allow(non_camel_case_types)]
                    struct SuperfluidUndelegationsByDelegatorSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::SuperfluidUndelegationsByDelegatorRequest,
                    > for SuperfluidUndelegationsByDelegatorSvc<T> {
                        type Response = super::SuperfluidUndelegationsByDelegatorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SuperfluidUndelegationsByDelegatorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .superfluid_undelegations_by_delegator(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SuperfluidUndelegationsByDelegatorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/SuperfluidDelegationsByValidatorDenom" => {
                    #[allow(non_camel_case_types)]
                    struct SuperfluidDelegationsByValidatorDenomSvc<T: Query>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::SuperfluidDelegationsByValidatorDenomRequest,
                    > for SuperfluidDelegationsByValidatorDenomSvc<T> {
                        type Response = super::SuperfluidDelegationsByValidatorDenomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SuperfluidDelegationsByValidatorDenomRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .superfluid_delegations_by_validator_denom(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SuperfluidDelegationsByValidatorDenomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/EstimateSuperfluidDelegatedAmountByValidatorDenom" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateSuperfluidDelegatedAmountByValidatorDenomSvc<
                        T: Query,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::EstimateSuperfluidDelegatedAmountByValidatorDenomRequest,
                    > for EstimateSuperfluidDelegatedAmountByValidatorDenomSvc<T> {
                        type Response = super::EstimateSuperfluidDelegatedAmountByValidatorDenomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::EstimateSuperfluidDelegatedAmountByValidatorDenomRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .estimate_superfluid_delegated_amount_by_validator_denom(
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EstimateSuperfluidDelegatedAmountByValidatorDenomSvc(
                            inner,
                        );
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/TotalDelegationByDelegator" => {
                    #[allow(non_camel_case_types)]
                    struct TotalDelegationByDelegatorSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryTotalDelegationByDelegatorRequest,
                    > for TotalDelegationByDelegatorSvc<T> {
                        type Response = super::QueryTotalDelegationByDelegatorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryTotalDelegationByDelegatorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).total_delegation_by_delegator(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TotalDelegationByDelegatorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/UnpoolWhitelist" => {
                    #[allow(non_camel_case_types)]
                    struct UnpoolWhitelistSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryUnpoolWhitelistRequest>
                    for UnpoolWhitelistSvc<T> {
                        type Response = super::QueryUnpoolWhitelistResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryUnpoolWhitelistRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).unpool_whitelist(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnpoolWhitelistSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/UserConcentratedSuperfluidPositionsDelegated" => {
                    #[allow(non_camel_case_types)]
                    struct UserConcentratedSuperfluidPositionsDelegatedSvc<T: Query>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::UserConcentratedSuperfluidPositionsDelegatedRequest,
                    > for UserConcentratedSuperfluidPositionsDelegatedSvc<T> {
                        type Response = super::UserConcentratedSuperfluidPositionsDelegatedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UserConcentratedSuperfluidPositionsDelegatedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .user_concentrated_superfluid_positions_delegated(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UserConcentratedSuperfluidPositionsDelegatedSvc(
                            inner,
                        );
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/UserConcentratedSuperfluidPositionsUndelegating" => {
                    #[allow(non_camel_case_types)]
                    struct UserConcentratedSuperfluidPositionsUndelegatingSvc<T: Query>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::UserConcentratedSuperfluidPositionsUndelegatingRequest,
                    > for UserConcentratedSuperfluidPositionsUndelegatingSvc<T> {
                        type Response = super::UserConcentratedSuperfluidPositionsUndelegatingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UserConcentratedSuperfluidPositionsUndelegatingRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .user_concentrated_superfluid_positions_undelegating(
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UserConcentratedSuperfluidPositionsUndelegatingSvc(
                            inner,
                        );
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Query/RestSupply" => {
                    #[allow(non_camel_case_types)]
                    struct RestSupplySvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryRestSupplyRequest>
                    for RestSupplySvc<T> {
                        type Response = super::QueryRestSupplyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRestSupplyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).rest_supply(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RestSupplySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "osmosis.superfluid.Query";
    }
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn superfluid_delegate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSuperfluidDelegate>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSuperfluidDelegateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/SuperfluidDelegate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.superfluid.Msg", "SuperfluidDelegate"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn superfluid_undelegate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSuperfluidUndelegate>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSuperfluidUndelegateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/SuperfluidUndelegate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.superfluid.Msg", "SuperfluidUndelegate"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn superfluid_unbond_lock(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSuperfluidUnbondLock>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSuperfluidUnbondLockResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/SuperfluidUnbondLock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.superfluid.Msg", "SuperfluidUnbondLock"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn superfluid_undelegate_and_unbond_lock(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSuperfluidUndelegateAndUnbondLock>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSuperfluidUndelegateAndUnbondLockResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/SuperfluidUndelegateAndUnbondLock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Msg",
                        "SuperfluidUndelegateAndUnbondLock",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn lock_and_superfluid_delegate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgLockAndSuperfluidDelegate>,
        ) -> std::result::Result<
            tonic::Response<super::MsgLockAndSuperfluidDelegateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/LockAndSuperfluidDelegate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Msg",
                        "LockAndSuperfluidDelegate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_full_range_position_and_superfluid_delegate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::MsgCreateFullRangePositionAndSuperfluidDelegate,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::MsgCreateFullRangePositionAndSuperfluidDelegateResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/CreateFullRangePositionAndSuperfluidDelegate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Msg",
                        "CreateFullRangePositionAndSuperfluidDelegate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn un_pool_whitelisted_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnPoolWhitelistedPool>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUnPoolWhitelistedPoolResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/UnPoolWhitelistedPool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.superfluid.Msg", "UnPoolWhitelistedPool"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlock_and_migrate_shares_to_full_range_concentrated_position(
            &mut self,
            request: impl tonic::IntoRequest<
                super::MsgUnlockAndMigrateSharesToFullRangeConcentratedPosition,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::MsgUnlockAndMigrateSharesToFullRangeConcentratedPositionResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/UnlockAndMigrateSharesToFullRangeConcentratedPosition",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Msg",
                        "UnlockAndMigrateSharesToFullRangeConcentratedPosition",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_to_concentrated_liquidity_superfluid_position(
            &mut self,
            request: impl tonic::IntoRequest<
                super::MsgAddToConcentratedLiquiditySuperfluidPosition,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::MsgAddToConcentratedLiquiditySuperfluidPositionResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/AddToConcentratedLiquiditySuperfluidPosition",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.superfluid.Msg",
                        "AddToConcentratedLiquiditySuperfluidPosition",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unbond_convert_and_stake(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnbondConvertAndStake>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUnbondConvertAndStakeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.superfluid.Msg/UnbondConvertAndStake",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.superfluid.Msg", "UnbondConvertAndStake"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        async fn superfluid_delegate(
            &self,
            request: tonic::Request<super::MsgSuperfluidDelegate>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSuperfluidDelegateResponse>,
            tonic::Status,
        >;
        async fn superfluid_undelegate(
            &self,
            request: tonic::Request<super::MsgSuperfluidUndelegate>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSuperfluidUndelegateResponse>,
            tonic::Status,
        >;
        async fn superfluid_unbond_lock(
            &self,
            request: tonic::Request<super::MsgSuperfluidUnbondLock>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSuperfluidUnbondLockResponse>,
            tonic::Status,
        >;
        async fn superfluid_undelegate_and_unbond_lock(
            &self,
            request: tonic::Request<super::MsgSuperfluidUndelegateAndUnbondLock>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSuperfluidUndelegateAndUnbondLockResponse>,
            tonic::Status,
        >;
        async fn lock_and_superfluid_delegate(
            &self,
            request: tonic::Request<super::MsgLockAndSuperfluidDelegate>,
        ) -> std::result::Result<
            tonic::Response<super::MsgLockAndSuperfluidDelegateResponse>,
            tonic::Status,
        >;
        async fn create_full_range_position_and_superfluid_delegate(
            &self,
            request: tonic::Request<
                super::MsgCreateFullRangePositionAndSuperfluidDelegate,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::MsgCreateFullRangePositionAndSuperfluidDelegateResponse,
            >,
            tonic::Status,
        >;
        async fn un_pool_whitelisted_pool(
            &self,
            request: tonic::Request<super::MsgUnPoolWhitelistedPool>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUnPoolWhitelistedPoolResponse>,
            tonic::Status,
        >;
        async fn unlock_and_migrate_shares_to_full_range_concentrated_position(
            &self,
            request: tonic::Request<
                super::MsgUnlockAndMigrateSharesToFullRangeConcentratedPosition,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::MsgUnlockAndMigrateSharesToFullRangeConcentratedPositionResponse,
            >,
            tonic::Status,
        >;
        async fn add_to_concentrated_liquidity_superfluid_position(
            &self,
            request: tonic::Request<
                super::MsgAddToConcentratedLiquiditySuperfluidPosition,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::MsgAddToConcentratedLiquiditySuperfluidPositionResponse,
            >,
            tonic::Status,
        >;
        async fn unbond_convert_and_stake(
            &self,
            request: tonic::Request<super::MsgUnbondConvertAndStake>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUnbondConvertAndStakeResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/osmosis.superfluid.Msg/SuperfluidDelegate" => {
                    #[allow(non_camel_case_types)]
                    struct SuperfluidDelegateSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgSuperfluidDelegate>
                    for SuperfluidDelegateSvc<T> {
                        type Response = super::MsgSuperfluidDelegateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSuperfluidDelegate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).superfluid_delegate(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SuperfluidDelegateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Msg/SuperfluidUndelegate" => {
                    #[allow(non_camel_case_types)]
                    struct SuperfluidUndelegateSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgSuperfluidUndelegate>
                    for SuperfluidUndelegateSvc<T> {
                        type Response = super::MsgSuperfluidUndelegateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSuperfluidUndelegate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).superfluid_undelegate(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SuperfluidUndelegateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Msg/SuperfluidUnbondLock" => {
                    #[allow(non_camel_case_types)]
                    struct SuperfluidUnbondLockSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgSuperfluidUnbondLock>
                    for SuperfluidUnbondLockSvc<T> {
                        type Response = super::MsgSuperfluidUnbondLockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSuperfluidUnbondLock>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).superfluid_unbond_lock(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SuperfluidUnbondLockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Msg/SuperfluidUndelegateAndUnbondLock" => {
                    #[allow(non_camel_case_types)]
                    struct SuperfluidUndelegateAndUnbondLockSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<
                        super::MsgSuperfluidUndelegateAndUnbondLock,
                    > for SuperfluidUndelegateAndUnbondLockSvc<T> {
                        type Response = super::MsgSuperfluidUndelegateAndUnbondLockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgSuperfluidUndelegateAndUnbondLock,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .superfluid_undelegate_and_unbond_lock(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SuperfluidUndelegateAndUnbondLockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Msg/LockAndSuperfluidDelegate" => {
                    #[allow(non_camel_case_types)]
                    struct LockAndSuperfluidDelegateSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgLockAndSuperfluidDelegate>
                    for LockAndSuperfluidDelegateSvc<T> {
                        type Response = super::MsgLockAndSuperfluidDelegateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgLockAndSuperfluidDelegate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).lock_and_superfluid_delegate(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LockAndSuperfluidDelegateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Msg/CreateFullRangePositionAndSuperfluidDelegate" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFullRangePositionAndSuperfluidDelegateSvc<T: Msg>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<
                        super::MsgCreateFullRangePositionAndSuperfluidDelegate,
                    > for CreateFullRangePositionAndSuperfluidDelegateSvc<T> {
                        type Response = super::MsgCreateFullRangePositionAndSuperfluidDelegateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgCreateFullRangePositionAndSuperfluidDelegate,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .create_full_range_position_and_superfluid_delegate(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateFullRangePositionAndSuperfluidDelegateSvc(
                            inner,
                        );
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Msg/UnPoolWhitelistedPool" => {
                    #[allow(non_camel_case_types)]
                    struct UnPoolWhitelistedPoolSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgUnPoolWhitelistedPool>
                    for UnPoolWhitelistedPoolSvc<T> {
                        type Response = super::MsgUnPoolWhitelistedPoolResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUnPoolWhitelistedPool>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).un_pool_whitelisted_pool(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnPoolWhitelistedPoolSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Msg/UnlockAndMigrateSharesToFullRangeConcentratedPosition" => {
                    #[allow(non_camel_case_types)]
                    struct UnlockAndMigrateSharesToFullRangeConcentratedPositionSvc<
                        T: Msg,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<
                        super::MsgUnlockAndMigrateSharesToFullRangeConcentratedPosition,
                    > for UnlockAndMigrateSharesToFullRangeConcentratedPositionSvc<T> {
                        type Response = super::MsgUnlockAndMigrateSharesToFullRangeConcentratedPositionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgUnlockAndMigrateSharesToFullRangeConcentratedPosition,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .unlock_and_migrate_shares_to_full_range_concentrated_position(
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnlockAndMigrateSharesToFullRangeConcentratedPositionSvc(
                            inner,
                        );
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Msg/AddToConcentratedLiquiditySuperfluidPosition" => {
                    #[allow(non_camel_case_types)]
                    struct AddToConcentratedLiquiditySuperfluidPositionSvc<T: Msg>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<
                        super::MsgAddToConcentratedLiquiditySuperfluidPosition,
                    > for AddToConcentratedLiquiditySuperfluidPositionSvc<T> {
                        type Response = super::MsgAddToConcentratedLiquiditySuperfluidPositionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgAddToConcentratedLiquiditySuperfluidPosition,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .add_to_concentrated_liquidity_superfluid_position(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddToConcentratedLiquiditySuperfluidPositionSvc(
                            inner,
                        );
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/osmosis.superfluid.Msg/UnbondConvertAndStake" => {
                    #[allow(non_camel_case_types)]
                    struct UnbondConvertAndStakeSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgUnbondConvertAndStake>
                    for UnbondConvertAndStakeSvc<T> {
                        type Response = super::MsgUnbondConvertAndStakeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUnbondConvertAndStake>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).unbond_convert_and_stake(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnbondConvertAndStakeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "osmosis.superfluid.Msg";
    }
}
