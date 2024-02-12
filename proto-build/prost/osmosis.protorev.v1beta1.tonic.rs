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
                "/osmosis.protorev.v1beta1.Query/Params",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.protorev.v1beta1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_number_of_trades(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryGetProtoRevNumberOfTradesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevNumberOfTradesResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevNumberOfTrades",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetProtoRevNumberOfTrades",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_profits_by_denom(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryGetProtoRevProfitsByDenomRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevProfitsByDenomResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevProfitsByDenom",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetProtoRevProfitsByDenom",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_all_profits(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetProtoRevAllProfitsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevAllProfitsResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevAllProfits",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetProtoRevAllProfits",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_statistics_by_route(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryGetProtoRevStatisticsByRouteRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevStatisticsByRouteResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevStatisticsByRoute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetProtoRevStatisticsByRoute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_all_route_statistics(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryGetProtoRevAllRouteStatisticsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevAllRouteStatisticsResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevAllRouteStatistics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetProtoRevAllRouteStatistics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_token_pair_arb_routes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryGetProtoRevTokenPairArbRoutesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevTokenPairArbRoutesResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevTokenPairArbRoutes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetProtoRevTokenPairArbRoutes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_admin_account(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetProtoRevAdminAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevAdminAccountResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevAdminAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetProtoRevAdminAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_developer_account(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryGetProtoRevDeveloperAccountRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevDeveloperAccountResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevDeveloperAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetProtoRevDeveloperAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_info_by_pool_type(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryGetProtoRevInfoByPoolTypeRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevInfoByPoolTypeResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevInfoByPoolType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetProtoRevInfoByPoolType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_max_pool_points_per_tx(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryGetProtoRevMaxPoolPointsPerTxRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevMaxPoolPointsPerTxResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevMaxPoolPointsPerTx",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetProtoRevMaxPoolPointsPerTx",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_max_pool_points_per_block(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryGetProtoRevMaxPoolPointsPerBlockRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevMaxPoolPointsPerBlockResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevMaxPoolPointsPerBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetProtoRevMaxPoolPointsPerBlock",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_base_denoms(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetProtoRevBaseDenomsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevBaseDenomsResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevBaseDenoms",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetProtoRevBaseDenoms",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_enabled(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetProtoRevEnabledRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevEnabledResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevEnabled",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetProtoRevEnabled",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proto_rev_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetProtoRevPoolRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevPoolResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevPool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.protorev.v1beta1.Query", "GetProtoRevPool"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_all_protocol_revenue(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetAllProtocolRevenueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetAllProtocolRevenueResponse>,
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
                "/osmosis.protorev.v1beta1.Query/GetAllProtocolRevenue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Query",
                        "GetAllProtocolRevenue",
                    ),
                );
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
        async fn get_proto_rev_number_of_trades(
            &self,
            request: tonic::Request<super::QueryGetProtoRevNumberOfTradesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevNumberOfTradesResponse>,
            tonic::Status,
        >;
        async fn get_proto_rev_profits_by_denom(
            &self,
            request: tonic::Request<super::QueryGetProtoRevProfitsByDenomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevProfitsByDenomResponse>,
            tonic::Status,
        >;
        async fn get_proto_rev_all_profits(
            &self,
            request: tonic::Request<super::QueryGetProtoRevAllProfitsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevAllProfitsResponse>,
            tonic::Status,
        >;
        async fn get_proto_rev_statistics_by_route(
            &self,
            request: tonic::Request<super::QueryGetProtoRevStatisticsByRouteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevStatisticsByRouteResponse>,
            tonic::Status,
        >;
        async fn get_proto_rev_all_route_statistics(
            &self,
            request: tonic::Request<super::QueryGetProtoRevAllRouteStatisticsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevAllRouteStatisticsResponse>,
            tonic::Status,
        >;
        async fn get_proto_rev_token_pair_arb_routes(
            &self,
            request: tonic::Request<super::QueryGetProtoRevTokenPairArbRoutesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevTokenPairArbRoutesResponse>,
            tonic::Status,
        >;
        async fn get_proto_rev_admin_account(
            &self,
            request: tonic::Request<super::QueryGetProtoRevAdminAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevAdminAccountResponse>,
            tonic::Status,
        >;
        async fn get_proto_rev_developer_account(
            &self,
            request: tonic::Request<super::QueryGetProtoRevDeveloperAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevDeveloperAccountResponse>,
            tonic::Status,
        >;
        async fn get_proto_rev_info_by_pool_type(
            &self,
            request: tonic::Request<super::QueryGetProtoRevInfoByPoolTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevInfoByPoolTypeResponse>,
            tonic::Status,
        >;
        async fn get_proto_rev_max_pool_points_per_tx(
            &self,
            request: tonic::Request<super::QueryGetProtoRevMaxPoolPointsPerTxRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevMaxPoolPointsPerTxResponse>,
            tonic::Status,
        >;
        async fn get_proto_rev_max_pool_points_per_block(
            &self,
            request: tonic::Request<super::QueryGetProtoRevMaxPoolPointsPerBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevMaxPoolPointsPerBlockResponse>,
            tonic::Status,
        >;
        async fn get_proto_rev_base_denoms(
            &self,
            request: tonic::Request<super::QueryGetProtoRevBaseDenomsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevBaseDenomsResponse>,
            tonic::Status,
        >;
        async fn get_proto_rev_enabled(
            &self,
            request: tonic::Request<super::QueryGetProtoRevEnabledRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevEnabledResponse>,
            tonic::Status,
        >;
        async fn get_proto_rev_pool(
            &self,
            request: tonic::Request<super::QueryGetProtoRevPoolRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetProtoRevPoolResponse>,
            tonic::Status,
        >;
        async fn get_all_protocol_revenue(
            &self,
            request: tonic::Request<super::QueryGetAllProtocolRevenueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetAllProtocolRevenueResponse>,
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
                "/osmosis.protorev.v1beta1.Query/Params" => {
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevNumberOfTrades" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevNumberOfTradesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGetProtoRevNumberOfTradesRequest,
                    > for GetProtoRevNumberOfTradesSvc<T> {
                        type Response = super::QueryGetProtoRevNumberOfTradesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetProtoRevNumberOfTradesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proto_rev_number_of_trades(request).await
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
                        let method = GetProtoRevNumberOfTradesSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevProfitsByDenom" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevProfitsByDenomSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGetProtoRevProfitsByDenomRequest,
                    > for GetProtoRevProfitsByDenomSvc<T> {
                        type Response = super::QueryGetProtoRevProfitsByDenomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetProtoRevProfitsByDenomRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proto_rev_profits_by_denom(request).await
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
                        let method = GetProtoRevProfitsByDenomSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevAllProfits" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevAllProfitsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGetProtoRevAllProfitsRequest,
                    > for GetProtoRevAllProfitsSvc<T> {
                        type Response = super::QueryGetProtoRevAllProfitsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetProtoRevAllProfitsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proto_rev_all_profits(request).await
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
                        let method = GetProtoRevAllProfitsSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevStatisticsByRoute" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevStatisticsByRouteSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGetProtoRevStatisticsByRouteRequest,
                    > for GetProtoRevStatisticsByRouteSvc<T> {
                        type Response = super::QueryGetProtoRevStatisticsByRouteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetProtoRevStatisticsByRouteRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proto_rev_statistics_by_route(request).await
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
                        let method = GetProtoRevStatisticsByRouteSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevAllRouteStatistics" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevAllRouteStatisticsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGetProtoRevAllRouteStatisticsRequest,
                    > for GetProtoRevAllRouteStatisticsSvc<T> {
                        type Response = super::QueryGetProtoRevAllRouteStatisticsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetProtoRevAllRouteStatisticsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proto_rev_all_route_statistics(request).await
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
                        let method = GetProtoRevAllRouteStatisticsSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevTokenPairArbRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevTokenPairArbRoutesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGetProtoRevTokenPairArbRoutesRequest,
                    > for GetProtoRevTokenPairArbRoutesSvc<T> {
                        type Response = super::QueryGetProtoRevTokenPairArbRoutesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetProtoRevTokenPairArbRoutesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proto_rev_token_pair_arb_routes(request).await
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
                        let method = GetProtoRevTokenPairArbRoutesSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevAdminAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevAdminAccountSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGetProtoRevAdminAccountRequest,
                    > for GetProtoRevAdminAccountSvc<T> {
                        type Response = super::QueryGetProtoRevAdminAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetProtoRevAdminAccountRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proto_rev_admin_account(request).await
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
                        let method = GetProtoRevAdminAccountSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevDeveloperAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevDeveloperAccountSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGetProtoRevDeveloperAccountRequest,
                    > for GetProtoRevDeveloperAccountSvc<T> {
                        type Response = super::QueryGetProtoRevDeveloperAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetProtoRevDeveloperAccountRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proto_rev_developer_account(request).await
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
                        let method = GetProtoRevDeveloperAccountSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevInfoByPoolType" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevInfoByPoolTypeSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGetProtoRevInfoByPoolTypeRequest,
                    > for GetProtoRevInfoByPoolTypeSvc<T> {
                        type Response = super::QueryGetProtoRevInfoByPoolTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetProtoRevInfoByPoolTypeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proto_rev_info_by_pool_type(request).await
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
                        let method = GetProtoRevInfoByPoolTypeSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevMaxPoolPointsPerTx" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevMaxPoolPointsPerTxSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGetProtoRevMaxPoolPointsPerTxRequest,
                    > for GetProtoRevMaxPoolPointsPerTxSvc<T> {
                        type Response = super::QueryGetProtoRevMaxPoolPointsPerTxResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetProtoRevMaxPoolPointsPerTxRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proto_rev_max_pool_points_per_tx(request).await
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
                        let method = GetProtoRevMaxPoolPointsPerTxSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevMaxPoolPointsPerBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevMaxPoolPointsPerBlockSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGetProtoRevMaxPoolPointsPerBlockRequest,
                    > for GetProtoRevMaxPoolPointsPerBlockSvc<T> {
                        type Response = super::QueryGetProtoRevMaxPoolPointsPerBlockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetProtoRevMaxPoolPointsPerBlockRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_proto_rev_max_pool_points_per_block(request)
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
                        let method = GetProtoRevMaxPoolPointsPerBlockSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevBaseDenoms" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevBaseDenomsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGetProtoRevBaseDenomsRequest,
                    > for GetProtoRevBaseDenomsSvc<T> {
                        type Response = super::QueryGetProtoRevBaseDenomsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetProtoRevBaseDenomsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proto_rev_base_denoms(request).await
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
                        let method = GetProtoRevBaseDenomsSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevEnabled" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevEnabledSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryGetProtoRevEnabledRequest>
                    for GetProtoRevEnabledSvc<T> {
                        type Response = super::QueryGetProtoRevEnabledResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetProtoRevEnabledRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proto_rev_enabled(request).await
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
                        let method = GetProtoRevEnabledSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetProtoRevPool" => {
                    #[allow(non_camel_case_types)]
                    struct GetProtoRevPoolSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryGetProtoRevPoolRequest>
                    for GetProtoRevPoolSvc<T> {
                        type Response = super::QueryGetProtoRevPoolResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetProtoRevPoolRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proto_rev_pool(request).await
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
                        let method = GetProtoRevPoolSvc(inner);
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
                "/osmosis.protorev.v1beta1.Query/GetAllProtocolRevenue" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllProtocolRevenueSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryGetAllProtocolRevenueRequest,
                    > for GetAllProtocolRevenueSvc<T> {
                        type Response = super::QueryGetAllProtocolRevenueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetAllProtocolRevenueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_all_protocol_revenue(request).await
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
                        let method = GetAllProtocolRevenueSvc(inner);
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
        const NAME: &'static str = "osmosis.protorev.v1beta1.Query";
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
        pub async fn set_hot_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetHotRoutes>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetHotRoutesResponse>,
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
                "/osmosis.protorev.v1beta1.Msg/SetHotRoutes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.protorev.v1beta1.Msg", "SetHotRoutes"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_developer_account(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetDeveloperAccount>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetDeveloperAccountResponse>,
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
                "/osmosis.protorev.v1beta1.Msg/SetDeveloperAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Msg",
                        "SetDeveloperAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_max_pool_points_per_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetMaxPoolPointsPerTx>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetMaxPoolPointsPerTxResponse>,
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
                "/osmosis.protorev.v1beta1.Msg/SetMaxPoolPointsPerTx",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Msg",
                        "SetMaxPoolPointsPerTx",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_max_pool_points_per_block(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetMaxPoolPointsPerBlock>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetMaxPoolPointsPerBlockResponse>,
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
                "/osmosis.protorev.v1beta1.Msg/SetMaxPoolPointsPerBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.protorev.v1beta1.Msg",
                        "SetMaxPoolPointsPerBlock",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_info_by_pool_type(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetInfoByPoolType>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetInfoByPoolTypeResponse>,
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
                "/osmosis.protorev.v1beta1.Msg/SetInfoByPoolType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.protorev.v1beta1.Msg", "SetInfoByPoolType"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_base_denoms(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetBaseDenoms>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetBaseDenomsResponse>,
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
                "/osmosis.protorev.v1beta1.Msg/SetBaseDenoms",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.protorev.v1beta1.Msg", "SetBaseDenoms"),
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
        async fn set_hot_routes(
            &self,
            request: tonic::Request<super::MsgSetHotRoutes>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetHotRoutesResponse>,
            tonic::Status,
        >;
        async fn set_developer_account(
            &self,
            request: tonic::Request<super::MsgSetDeveloperAccount>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetDeveloperAccountResponse>,
            tonic::Status,
        >;
        async fn set_max_pool_points_per_tx(
            &self,
            request: tonic::Request<super::MsgSetMaxPoolPointsPerTx>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetMaxPoolPointsPerTxResponse>,
            tonic::Status,
        >;
        async fn set_max_pool_points_per_block(
            &self,
            request: tonic::Request<super::MsgSetMaxPoolPointsPerBlock>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetMaxPoolPointsPerBlockResponse>,
            tonic::Status,
        >;
        async fn set_info_by_pool_type(
            &self,
            request: tonic::Request<super::MsgSetInfoByPoolType>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetInfoByPoolTypeResponse>,
            tonic::Status,
        >;
        async fn set_base_denoms(
            &self,
            request: tonic::Request<super::MsgSetBaseDenoms>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetBaseDenomsResponse>,
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
                "/osmosis.protorev.v1beta1.Msg/SetHotRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct SetHotRoutesSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSetHotRoutes>
                    for SetHotRoutesSvc<T> {
                        type Response = super::MsgSetHotRoutesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetHotRoutes>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_hot_routes(request).await
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
                        let method = SetHotRoutesSvc(inner);
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
                "/osmosis.protorev.v1beta1.Msg/SetDeveloperAccount" => {
                    #[allow(non_camel_case_types)]
                    struct SetDeveloperAccountSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgSetDeveloperAccount>
                    for SetDeveloperAccountSvc<T> {
                        type Response = super::MsgSetDeveloperAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetDeveloperAccount>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_developer_account(request).await
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
                        let method = SetDeveloperAccountSvc(inner);
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
                "/osmosis.protorev.v1beta1.Msg/SetMaxPoolPointsPerTx" => {
                    #[allow(non_camel_case_types)]
                    struct SetMaxPoolPointsPerTxSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgSetMaxPoolPointsPerTx>
                    for SetMaxPoolPointsPerTxSvc<T> {
                        type Response = super::MsgSetMaxPoolPointsPerTxResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetMaxPoolPointsPerTx>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_max_pool_points_per_tx(request).await
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
                        let method = SetMaxPoolPointsPerTxSvc(inner);
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
                "/osmosis.protorev.v1beta1.Msg/SetMaxPoolPointsPerBlock" => {
                    #[allow(non_camel_case_types)]
                    struct SetMaxPoolPointsPerBlockSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgSetMaxPoolPointsPerBlock>
                    for SetMaxPoolPointsPerBlockSvc<T> {
                        type Response = super::MsgSetMaxPoolPointsPerBlockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetMaxPoolPointsPerBlock>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_max_pool_points_per_block(request).await
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
                        let method = SetMaxPoolPointsPerBlockSvc(inner);
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
                "/osmosis.protorev.v1beta1.Msg/SetInfoByPoolType" => {
                    #[allow(non_camel_case_types)]
                    struct SetInfoByPoolTypeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSetInfoByPoolType>
                    for SetInfoByPoolTypeSvc<T> {
                        type Response = super::MsgSetInfoByPoolTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetInfoByPoolType>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_info_by_pool_type(request).await
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
                        let method = SetInfoByPoolTypeSvc(inner);
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
                "/osmosis.protorev.v1beta1.Msg/SetBaseDenoms" => {
                    #[allow(non_camel_case_types)]
                    struct SetBaseDenomsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSetBaseDenoms>
                    for SetBaseDenomsSvc<T> {
                        type Response = super::MsgSetBaseDenomsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetBaseDenoms>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_base_denoms(request).await
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
                        let method = SetBaseDenomsSvc(inner);
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
        const NAME: &'static str = "osmosis.protorev.v1beta1.Msg";
    }
}
