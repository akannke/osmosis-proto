// @generated
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
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
            Self {
                inner,
            }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self {
                inner,
            }
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
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
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
        pub async fn module_to_distribute_coins(
            &mut self,
            request: impl tonic::IntoRequest<super::ModuleToDistributeCoinsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ModuleToDistributeCoinsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/ModuleToDistributeCoins",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.incentives.Query", "ModuleToDistributeCoins"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn gauge_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GaugeByIdRequest>,
        ) -> std::result::Result<tonic::Response<super::GaugeByIdResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/GaugeByID");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("osmosis.incentives.Query", "GaugeByID"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn gauges(
            &mut self,
            request: impl tonic::IntoRequest<super::GaugesRequest>,
        ) -> std::result::Result<tonic::Response<super::GaugesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/Gauges");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("osmosis.incentives.Query", "Gauges"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn active_gauges(
            &mut self,
            request: impl tonic::IntoRequest<super::ActiveGaugesRequest>,
        ) -> std::result::Result<tonic::Response<super::ActiveGaugesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/ActiveGauges");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.incentives.Query", "ActiveGauges"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn active_gauges_per_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::ActiveGaugesPerDenomRequest>,
        ) -> std::result::Result<tonic::Response<super::ActiveGaugesPerDenomResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/ActiveGaugesPerDenom",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.incentives.Query", "ActiveGaugesPerDenom"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn upcoming_gauges(
            &mut self,
            request: impl tonic::IntoRequest<super::UpcomingGaugesRequest>,
        ) -> std::result::Result<tonic::Response<super::UpcomingGaugesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/UpcomingGauges");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.incentives.Query", "UpcomingGauges"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn upcoming_gauges_per_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::UpcomingGaugesPerDenomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpcomingGaugesPerDenomResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/UpcomingGaugesPerDenom",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.incentives.Query", "UpcomingGaugesPerDenom"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn rewards_est(
            &mut self,
            request: impl tonic::IntoRequest<super::RewardsEstRequest>,
        ) -> std::result::Result<tonic::Response<super::RewardsEstResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/RewardsEst");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("osmosis.incentives.Query", "RewardsEst"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn lockable_durations(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLockableDurationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryLockableDurationsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/LockableDurations");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.incentives.Query", "LockableDurations"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn all_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllGroupsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllGroupsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/AllGroups");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("osmosis.incentives.Query", "AllGroups"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn all_groups_gauges(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllGroupsGaugesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllGroupsGaugesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.incentives.Query/AllGroupsGauges");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.incentives.Query", "AllGroupsGauges"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn all_groups_with_gauge(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllGroupsWithGaugeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAllGroupsWithGaugeResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/AllGroupsWithGauge",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.incentives.Query", "AllGroupsWithGauge"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn group_by_group_gauge_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGroupByGroupGaugeIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupByGroupGaugeIdResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/GroupByGroupGaugeID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.incentives.Query", "GroupByGroupGaugeID"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn current_weight_by_group_gauge_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCurrentWeightByGroupGaugeIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCurrentWeightByGroupGaugeIdResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.incentives.Query/CurrentWeightByGroupGaugeID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.incentives.Query", "CurrentWeightByGroupGaugeID"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        async fn module_to_distribute_coins(
            &self,
            request: tonic::Request<super::ModuleToDistributeCoinsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ModuleToDistributeCoinsResponse>,
            tonic::Status,
        >;
        async fn gauge_by_id(
            &self,
            request: tonic::Request<super::GaugeByIdRequest>,
        ) -> std::result::Result<tonic::Response<super::GaugeByIdResponse>, tonic::Status>;
        async fn gauges(
            &self,
            request: tonic::Request<super::GaugesRequest>,
        ) -> std::result::Result<tonic::Response<super::GaugesResponse>, tonic::Status>;
        async fn active_gauges(
            &self,
            request: tonic::Request<super::ActiveGaugesRequest>,
        ) -> std::result::Result<tonic::Response<super::ActiveGaugesResponse>, tonic::Status>;
        async fn active_gauges_per_denom(
            &self,
            request: tonic::Request<super::ActiveGaugesPerDenomRequest>,
        ) -> std::result::Result<tonic::Response<super::ActiveGaugesPerDenomResponse>, tonic::Status>;
        async fn upcoming_gauges(
            &self,
            request: tonic::Request<super::UpcomingGaugesRequest>,
        ) -> std::result::Result<tonic::Response<super::UpcomingGaugesResponse>, tonic::Status>;
        async fn upcoming_gauges_per_denom(
            &self,
            request: tonic::Request<super::UpcomingGaugesPerDenomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpcomingGaugesPerDenomResponse>,
            tonic::Status,
        >;
        async fn rewards_est(
            &self,
            request: tonic::Request<super::RewardsEstRequest>,
        ) -> std::result::Result<tonic::Response<super::RewardsEstResponse>, tonic::Status>;
        async fn lockable_durations(
            &self,
            request: tonic::Request<super::QueryLockableDurationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryLockableDurationsResponse>,
            tonic::Status,
        >;
        async fn all_groups(
            &self,
            request: tonic::Request<super::QueryAllGroupsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllGroupsResponse>, tonic::Status>;
        async fn all_groups_gauges(
            &self,
            request: tonic::Request<super::QueryAllGroupsGaugesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllGroupsGaugesResponse>, tonic::Status>;
        async fn all_groups_with_gauge(
            &self,
            request: tonic::Request<super::QueryAllGroupsWithGaugeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAllGroupsWithGaugeResponse>,
            tonic::Status,
        >;
        async fn group_by_group_gauge_id(
            &self,
            request: tonic::Request<super::QueryGroupByGroupGaugeIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGroupByGroupGaugeIdResponse>,
            tonic::Status,
        >;
        async fn current_weight_by_group_gauge_id(
            &self,
            request: tonic::Request<super::QueryCurrentWeightByGroupGaugeIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCurrentWeightByGroupGaugeIdResponse>,
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
                "/osmosis.incentives.Query/ModuleToDistributeCoins" => {
                    #[allow(non_camel_case_types)]
                    struct ModuleToDistributeCoinsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::ModuleToDistributeCoinsRequest>
                        for ModuleToDistributeCoinsSvc<T>
                    {
                        type Response = super::ModuleToDistributeCoinsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModuleToDistributeCoinsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).module_to_distribute_coins(request).await };
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
                        let method = ModuleToDistributeCoinsSvc(inner);
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
                },
                "/osmosis.incentives.Query/GaugeByID" => {
                    #[allow(non_camel_case_types)]
                    struct GaugeByIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::GaugeByIdRequest> for GaugeByIDSvc<T> {
                        type Response = super::GaugeByIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GaugeByIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).gauge_by_id(request).await };
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
                        let method = GaugeByIDSvc(inner);
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
                },
                "/osmosis.incentives.Query/Gauges" => {
                    #[allow(non_camel_case_types)]
                    struct GaugesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::GaugesRequest> for GaugesSvc<T> {
                        type Response = super::GaugesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GaugesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).gauges(request).await };
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
                        let method = GaugesSvc(inner);
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
                },
                "/osmosis.incentives.Query/ActiveGauges" => {
                    #[allow(non_camel_case_types)]
                    struct ActiveGaugesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ActiveGaugesRequest> for ActiveGaugesSvc<T> {
                        type Response = super::ActiveGaugesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ActiveGaugesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).active_gauges(request).await };
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
                        let method = ActiveGaugesSvc(inner);
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
                },
                "/osmosis.incentives.Query/ActiveGaugesPerDenom" => {
                    #[allow(non_camel_case_types)]
                    struct ActiveGaugesPerDenomSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ActiveGaugesPerDenomRequest>
                        for ActiveGaugesPerDenomSvc<T>
                    {
                        type Response = super::ActiveGaugesPerDenomResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ActiveGaugesPerDenomRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).active_gauges_per_denom(request).await };
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
                        let method = ActiveGaugesPerDenomSvc(inner);
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
                },
                "/osmosis.incentives.Query/UpcomingGauges" => {
                    #[allow(non_camel_case_types)]
                    struct UpcomingGaugesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::UpcomingGaugesRequest> for UpcomingGaugesSvc<T> {
                        type Response = super::UpcomingGaugesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpcomingGaugesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).upcoming_gauges(request).await };
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
                        let method = UpcomingGaugesSvc(inner);
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
                },
                "/osmosis.incentives.Query/UpcomingGaugesPerDenom" => {
                    #[allow(non_camel_case_types)]
                    struct UpcomingGaugesPerDenomSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::UpcomingGaugesPerDenomRequest>
                        for UpcomingGaugesPerDenomSvc<T>
                    {
                        type Response = super::UpcomingGaugesPerDenomResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpcomingGaugesPerDenomRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).upcoming_gauges_per_denom(request).await };
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
                        let method = UpcomingGaugesPerDenomSvc(inner);
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
                },
                "/osmosis.incentives.Query/RewardsEst" => {
                    #[allow(non_camel_case_types)]
                    struct RewardsEstSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::RewardsEstRequest> for RewardsEstSvc<T> {
                        type Response = super::RewardsEstResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RewardsEstRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).rewards_est(request).await };
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
                        let method = RewardsEstSvc(inner);
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
                },
                "/osmosis.incentives.Query/LockableDurations" => {
                    #[allow(non_camel_case_types)]
                    struct LockableDurationsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryLockableDurationsRequest>
                        for LockableDurationsSvc<T>
                    {
                        type Response = super::QueryLockableDurationsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLockableDurationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).lockable_durations(request).await };
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
                        let method = LockableDurationsSvc(inner);
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
                },
                "/osmosis.incentives.Query/AllGroups" => {
                    #[allow(non_camel_case_types)]
                    struct AllGroupsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllGroupsRequest> for AllGroupsSvc<T> {
                        type Response = super::QueryAllGroupsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllGroupsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).all_groups(request).await };
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
                        let method = AllGroupsSvc(inner);
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
                },
                "/osmosis.incentives.Query/AllGroupsGauges" => {
                    #[allow(non_camel_case_types)]
                    struct AllGroupsGaugesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllGroupsGaugesRequest>
                        for AllGroupsGaugesSvc<T>
                    {
                        type Response = super::QueryAllGroupsGaugesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllGroupsGaugesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).all_groups_gauges(request).await };
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
                        let method = AllGroupsGaugesSvc(inner);
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
                },
                "/osmosis.incentives.Query/AllGroupsWithGauge" => {
                    #[allow(non_camel_case_types)]
                    struct AllGroupsWithGaugeSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryAllGroupsWithGaugeRequest>
                        for AllGroupsWithGaugeSvc<T>
                    {
                        type Response = super::QueryAllGroupsWithGaugeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllGroupsWithGaugeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).all_groups_with_gauge(request).await };
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
                        let method = AllGroupsWithGaugeSvc(inner);
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
                },
                "/osmosis.incentives.Query/GroupByGroupGaugeID" => {
                    #[allow(non_camel_case_types)]
                    struct GroupByGroupGaugeIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryGroupByGroupGaugeIdRequest>
                        for GroupByGroupGaugeIDSvc<T>
                    {
                        type Response = super::QueryGroupByGroupGaugeIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGroupByGroupGaugeIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).group_by_group_gauge_id(request).await };
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
                        let method = GroupByGroupGaugeIDSvc(inner);
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
                },
                "/osmosis.incentives.Query/CurrentWeightByGroupGaugeID" => {
                    #[allow(non_camel_case_types)]
                    struct CurrentWeightByGroupGaugeIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryCurrentWeightByGroupGaugeIdRequest>
                        for CurrentWeightByGroupGaugeIDSvc<T>
                    {
                        type Response = super::QueryCurrentWeightByGroupGaugeIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryCurrentWeightByGroupGaugeIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).current_weight_by_group_gauge_id(request).await
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
                        let method = CurrentWeightByGroupGaugeIDSvc(inner);
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
                },
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
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
        const NAME: &'static str = "osmosis.incentives.Query";
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
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
            Self {
                inner,
            }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self {
                inner,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
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
        pub async fn create_gauge(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateGauge>,
        ) -> std::result::Result<tonic::Response<super::MsgCreateGaugeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.incentives.Msg/CreateGauge");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("osmosis.incentives.Msg", "CreateGauge"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_to_gauge(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddToGauge>,
        ) -> std::result::Result<tonic::Response<super::MsgAddToGaugeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.incentives.Msg/AddToGauge");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("osmosis.incentives.Msg", "AddToGauge"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_group(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateGroup>,
        ) -> std::result::Result<tonic::Response<super::MsgCreateGroupResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.incentives.Msg/CreateGroup");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("osmosis.incentives.Msg", "CreateGroup"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        async fn create_gauge(
            &self,
            request: tonic::Request<super::MsgCreateGauge>,
        ) -> std::result::Result<tonic::Response<super::MsgCreateGaugeResponse>, tonic::Status>;
        async fn add_to_gauge(
            &self,
            request: tonic::Request<super::MsgAddToGauge>,
        ) -> std::result::Result<tonic::Response<super::MsgAddToGaugeResponse>, tonic::Status>;
        async fn create_group(
            &self,
            request: tonic::Request<super::MsgCreateGroup>,
        ) -> std::result::Result<tonic::Response<super::MsgCreateGroupResponse>, tonic::Status>;
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
                "/osmosis.incentives.Msg/CreateGauge" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGaugeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateGauge> for CreateGaugeSvc<T> {
                        type Response = super::MsgCreateGaugeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateGauge>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_gauge(request).await };
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
                        let method = CreateGaugeSvc(inner);
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
                },
                "/osmosis.incentives.Msg/AddToGauge" => {
                    #[allow(non_camel_case_types)]
                    struct AddToGaugeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAddToGauge> for AddToGaugeSvc<T> {
                        type Response = super::MsgAddToGaugeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddToGauge>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).add_to_gauge(request).await };
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
                        let method = AddToGaugeSvc(inner);
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
                },
                "/osmosis.incentives.Msg/CreateGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGroupSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateGroup> for CreateGroupSvc<T> {
                        type Response = super::MsgCreateGroupResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateGroup>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_group(request).await };
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
                        let method = CreateGroupSvc(inner);
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
                },
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
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
        const NAME: &'static str = "osmosis.incentives.Msg";
    }
}
