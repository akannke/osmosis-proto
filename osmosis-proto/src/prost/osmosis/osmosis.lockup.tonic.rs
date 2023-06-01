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
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
        pub async fn module_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::ModuleBalanceRequest>,
        ) -> Result<tonic::Response<super::ModuleBalanceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/ModuleBalance");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn module_locked_amount(
            &mut self,
            request: impl tonic::IntoRequest<super::ModuleLockedAmountRequest>,
        ) -> Result<tonic::Response<super::ModuleLockedAmountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/ModuleLockedAmount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn account_unlockable_coins(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountUnlockableCoinsRequest>,
        ) -> Result<tonic::Response<super::AccountUnlockableCoinsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/AccountUnlockableCoins",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn account_unlocking_coins(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountUnlockingCoinsRequest>,
        ) -> Result<tonic::Response<super::AccountUnlockingCoinsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/AccountUnlockingCoins");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn account_locked_coins(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedCoinsRequest>,
        ) -> Result<tonic::Response<super::AccountLockedCoinsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/AccountLockedCoins");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn account_locked_past_time(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedPastTimeRequest>,
        ) -> Result<tonic::Response<super::AccountLockedPastTimeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/AccountLockedPastTime");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn account_locked_past_time_not_unlocking_only(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedPastTimeNotUnlockingOnlyRequest>,
        ) -> Result<
            tonic::Response<super::AccountLockedPastTimeNotUnlockingOnlyResponse>,
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
                "/osmosis.lockup.Query/AccountLockedPastTimeNotUnlockingOnly",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn account_unlocked_before_time(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountUnlockedBeforeTimeRequest>,
        ) -> Result<tonic::Response<super::AccountUnlockedBeforeTimeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/AccountUnlockedBeforeTime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn account_locked_past_time_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedPastTimeDenomRequest>,
        ) -> Result<tonic::Response<super::AccountLockedPastTimeDenomResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/AccountLockedPastTimeDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn locked_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::LockedDenomRequest>,
        ) -> Result<tonic::Response<super::LockedDenomResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/LockedDenom");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn locked_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::LockedRequest>,
        ) -> Result<tonic::Response<super::LockedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/LockedByID");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn next_lock_id(
            &mut self,
            request: impl tonic::IntoRequest<super::NextLockIdRequest>,
        ) -> Result<tonic::Response<super::NextLockIdResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/NextLockID");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn synthetic_lockups_by_lockup_id(
            &mut self,
            request: impl tonic::IntoRequest<super::SyntheticLockupsByLockupIdRequest>,
        ) -> Result<tonic::Response<super::SyntheticLockupsByLockupIdResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/SyntheticLockupsByLockupID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn account_locked_longer_duration(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedLongerDurationRequest>,
        ) -> Result<tonic::Response<super::AccountLockedLongerDurationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/AccountLockedLongerDuration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn account_locked_duration(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedDurationRequest>,
        ) -> Result<tonic::Response<super::AccountLockedDurationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/AccountLockedDuration");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn account_locked_longer_duration_not_unlocking_only(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedLongerDurationNotUnlockingOnlyRequest>,
        ) -> Result<
            tonic::Response<super::AccountLockedLongerDurationNotUnlockingOnlyResponse>,
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
                "/osmosis.lockup.Query/AccountLockedLongerDurationNotUnlockingOnly",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn account_locked_longer_duration_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedLongerDurationDenomRequest>,
        ) -> Result<tonic::Response<super::AccountLockedLongerDurationDenomResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/AccountLockedLongerDurationDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        async fn module_balance(
            &self,
            request: tonic::Request<super::ModuleBalanceRequest>,
        ) -> Result<tonic::Response<super::ModuleBalanceResponse>, tonic::Status>;
        async fn module_locked_amount(
            &self,
            request: tonic::Request<super::ModuleLockedAmountRequest>,
        ) -> Result<tonic::Response<super::ModuleLockedAmountResponse>, tonic::Status>;
        async fn account_unlockable_coins(
            &self,
            request: tonic::Request<super::AccountUnlockableCoinsRequest>,
        ) -> Result<tonic::Response<super::AccountUnlockableCoinsResponse>, tonic::Status>;
        async fn account_unlocking_coins(
            &self,
            request: tonic::Request<super::AccountUnlockingCoinsRequest>,
        ) -> Result<tonic::Response<super::AccountUnlockingCoinsResponse>, tonic::Status>;
        async fn account_locked_coins(
            &self,
            request: tonic::Request<super::AccountLockedCoinsRequest>,
        ) -> Result<tonic::Response<super::AccountLockedCoinsResponse>, tonic::Status>;
        async fn account_locked_past_time(
            &self,
            request: tonic::Request<super::AccountLockedPastTimeRequest>,
        ) -> Result<tonic::Response<super::AccountLockedPastTimeResponse>, tonic::Status>;
        async fn account_locked_past_time_not_unlocking_only(
            &self,
            request: tonic::Request<super::AccountLockedPastTimeNotUnlockingOnlyRequest>,
        ) -> Result<
            tonic::Response<super::AccountLockedPastTimeNotUnlockingOnlyResponse>,
            tonic::Status,
        >;
        async fn account_unlocked_before_time(
            &self,
            request: tonic::Request<super::AccountUnlockedBeforeTimeRequest>,
        ) -> Result<tonic::Response<super::AccountUnlockedBeforeTimeResponse>, tonic::Status>;
        async fn account_locked_past_time_denom(
            &self,
            request: tonic::Request<super::AccountLockedPastTimeDenomRequest>,
        ) -> Result<tonic::Response<super::AccountLockedPastTimeDenomResponse>, tonic::Status>;
        async fn locked_denom(
            &self,
            request: tonic::Request<super::LockedDenomRequest>,
        ) -> Result<tonic::Response<super::LockedDenomResponse>, tonic::Status>;
        async fn locked_by_id(
            &self,
            request: tonic::Request<super::LockedRequest>,
        ) -> Result<tonic::Response<super::LockedResponse>, tonic::Status>;
        async fn next_lock_id(
            &self,
            request: tonic::Request<super::NextLockIdRequest>,
        ) -> Result<tonic::Response<super::NextLockIdResponse>, tonic::Status>;
        async fn synthetic_lockups_by_lockup_id(
            &self,
            request: tonic::Request<super::SyntheticLockupsByLockupIdRequest>,
        ) -> Result<tonic::Response<super::SyntheticLockupsByLockupIdResponse>, tonic::Status>;
        async fn account_locked_longer_duration(
            &self,
            request: tonic::Request<super::AccountLockedLongerDurationRequest>,
        ) -> Result<tonic::Response<super::AccountLockedLongerDurationResponse>, tonic::Status>;
        async fn account_locked_duration(
            &self,
            request: tonic::Request<super::AccountLockedDurationRequest>,
        ) -> Result<tonic::Response<super::AccountLockedDurationResponse>, tonic::Status>;
        async fn account_locked_longer_duration_not_unlocking_only(
            &self,
            request: tonic::Request<super::AccountLockedLongerDurationNotUnlockingOnlyRequest>,
        ) -> Result<
            tonic::Response<super::AccountLockedLongerDurationNotUnlockingOnlyResponse>,
            tonic::Status,
        >;
        async fn account_locked_longer_duration_denom(
            &self,
            request: tonic::Request<super::AccountLockedLongerDurationDenomRequest>,
        ) -> Result<tonic::Response<super::AccountLockedLongerDurationDenomResponse>, tonic::Status>;
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
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
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/osmosis.lockup.Query/ModuleBalance" => {
                    #[allow(non_camel_case_types)]
                    struct ModuleBalanceSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ModuleBalanceRequest> for ModuleBalanceSvc<T> {
                        type Response = super::ModuleBalanceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModuleBalanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).module_balance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ModuleBalanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/ModuleLockedAmount" => {
                    #[allow(non_camel_case_types)]
                    struct ModuleLockedAmountSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ModuleLockedAmountRequest>
                        for ModuleLockedAmountSvc<T>
                    {
                        type Response = super::ModuleLockedAmountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModuleLockedAmountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).module_locked_amount(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ModuleLockedAmountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/AccountUnlockableCoins" => {
                    #[allow(non_camel_case_types)]
                    struct AccountUnlockableCoinsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AccountUnlockableCoinsRequest>
                        for AccountUnlockableCoinsSvc<T>
                    {
                        type Response = super::AccountUnlockableCoinsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountUnlockableCoinsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).account_unlockable_coins(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountUnlockableCoinsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/AccountUnlockingCoins" => {
                    #[allow(non_camel_case_types)]
                    struct AccountUnlockingCoinsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AccountUnlockingCoinsRequest>
                        for AccountUnlockingCoinsSvc<T>
                    {
                        type Response = super::AccountUnlockingCoinsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountUnlockingCoinsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).account_unlocking_coins(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountUnlockingCoinsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/AccountLockedCoins" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedCoinsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AccountLockedCoinsRequest>
                        for AccountLockedCoinsSvc<T>
                    {
                        type Response = super::AccountLockedCoinsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountLockedCoinsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).account_locked_coins(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedCoinsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/AccountLockedPastTime" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedPastTimeSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AccountLockedPastTimeRequest>
                        for AccountLockedPastTimeSvc<T>
                    {
                        type Response = super::AccountLockedPastTimeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountLockedPastTimeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).account_locked_past_time(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedPastTimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/AccountLockedPastTimeNotUnlockingOnly" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedPastTimeNotUnlockingOnlySvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::AccountLockedPastTimeNotUnlockingOnlyRequest,
                        > for AccountLockedPastTimeNotUnlockingOnlySvc<T>
                    {
                        type Response = super::AccountLockedPastTimeNotUnlockingOnlyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AccountLockedPastTimeNotUnlockingOnlyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).account_locked_past_time_not_unlocking_only(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedPastTimeNotUnlockingOnlySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/AccountUnlockedBeforeTime" => {
                    #[allow(non_camel_case_types)]
                    struct AccountUnlockedBeforeTimeSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::AccountUnlockedBeforeTimeRequest>
                        for AccountUnlockedBeforeTimeSvc<T>
                    {
                        type Response = super::AccountUnlockedBeforeTimeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountUnlockedBeforeTimeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).account_unlocked_before_time(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountUnlockedBeforeTimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/AccountLockedPastTimeDenom" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedPastTimeDenomSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::AccountLockedPastTimeDenomRequest>
                        for AccountLockedPastTimeDenomSvc<T>
                    {
                        type Response = super::AccountLockedPastTimeDenomResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountLockedPastTimeDenomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).account_locked_past_time_denom(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedPastTimeDenomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/LockedDenom" => {
                    #[allow(non_camel_case_types)]
                    struct LockedDenomSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::LockedDenomRequest> for LockedDenomSvc<T> {
                        type Response = super::LockedDenomResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LockedDenomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).locked_denom(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LockedDenomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/LockedByID" => {
                    #[allow(non_camel_case_types)]
                    struct LockedByIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::LockedRequest> for LockedByIDSvc<T> {
                        type Response = super::LockedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LockedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).locked_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LockedByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/NextLockID" => {
                    #[allow(non_camel_case_types)]
                    struct NextLockIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::NextLockIdRequest> for NextLockIDSvc<T> {
                        type Response = super::NextLockIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NextLockIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).next_lock_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NextLockIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/SyntheticLockupsByLockupID" => {
                    #[allow(non_camel_case_types)]
                    struct SyntheticLockupsByLockupIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::SyntheticLockupsByLockupIdRequest>
                        for SyntheticLockupsByLockupIDSvc<T>
                    {
                        type Response = super::SyntheticLockupsByLockupIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SyntheticLockupsByLockupIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).synthetic_lockups_by_lockup_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SyntheticLockupsByLockupIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/AccountLockedLongerDuration" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedLongerDurationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::AccountLockedLongerDurationRequest>
                        for AccountLockedLongerDurationSvc<T>
                    {
                        type Response = super::AccountLockedLongerDurationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountLockedLongerDurationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).account_locked_longer_duration(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedLongerDurationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/AccountLockedDuration" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedDurationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AccountLockedDurationRequest>
                        for AccountLockedDurationSvc<T>
                    {
                        type Response = super::AccountLockedDurationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountLockedDurationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).account_locked_duration(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedDurationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/AccountLockedLongerDurationNotUnlockingOnly" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedLongerDurationNotUnlockingOnlySvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::AccountLockedLongerDurationNotUnlockingOnlyRequest,
                        > for AccountLockedLongerDurationNotUnlockingOnlySvc<T>
                    {
                        type Response = super::AccountLockedLongerDurationNotUnlockingOnlyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AccountLockedLongerDurationNotUnlockingOnlyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .account_locked_longer_duration_not_unlocking_only(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedLongerDurationNotUnlockingOnlySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/AccountLockedLongerDurationDenom" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedLongerDurationDenomSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::AccountLockedLongerDurationDenomRequest>
                        for AccountLockedLongerDurationDenomSvc<T>
                    {
                        type Response = super::AccountLockedLongerDurationDenomResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountLockedLongerDurationDenomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).account_locked_longer_duration_denom(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedLongerDurationDenomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest> for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
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
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "osmosis.lockup.Query";
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
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
        pub async fn lock_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgLockTokens>,
        ) -> Result<tonic::Response<super::MsgLockTokensResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Msg/LockTokens");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn begin_unlocking_all(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBeginUnlockingAll>,
        ) -> Result<tonic::Response<super::MsgBeginUnlockingAllResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.lockup.Msg/BeginUnlockingAll");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn begin_unlocking(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBeginUnlocking>,
        ) -> Result<tonic::Response<super::MsgBeginUnlockingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Msg/BeginUnlocking");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn extend_lockup(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExtendLockup>,
        ) -> Result<tonic::Response<super::MsgExtendLockupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Msg/ExtendLockup");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn force_unlock(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgForceUnlock>,
        ) -> Result<tonic::Response<super::MsgForceUnlockResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Msg/ForceUnlock");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        async fn lock_tokens(
            &self,
            request: tonic::Request<super::MsgLockTokens>,
        ) -> Result<tonic::Response<super::MsgLockTokensResponse>, tonic::Status>;
        async fn begin_unlocking_all(
            &self,
            request: tonic::Request<super::MsgBeginUnlockingAll>,
        ) -> Result<tonic::Response<super::MsgBeginUnlockingAllResponse>, tonic::Status>;
        async fn begin_unlocking(
            &self,
            request: tonic::Request<super::MsgBeginUnlocking>,
        ) -> Result<tonic::Response<super::MsgBeginUnlockingResponse>, tonic::Status>;
        async fn extend_lockup(
            &self,
            request: tonic::Request<super::MsgExtendLockup>,
        ) -> Result<tonic::Response<super::MsgExtendLockupResponse>, tonic::Status>;
        async fn force_unlock(
            &self,
            request: tonic::Request<super::MsgForceUnlock>,
        ) -> Result<tonic::Response<super::MsgForceUnlockResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
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
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/osmosis.lockup.Msg/LockTokens" => {
                    #[allow(non_camel_case_types)]
                    struct LockTokensSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgLockTokens> for LockTokensSvc<T> {
                        type Response = super::MsgLockTokensResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgLockTokens>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).lock_tokens(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LockTokensSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Msg/BeginUnlockingAll" => {
                    #[allow(non_camel_case_types)]
                    struct BeginUnlockingAllSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBeginUnlockingAll> for BeginUnlockingAllSvc<T> {
                        type Response = super::MsgBeginUnlockingAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBeginUnlockingAll>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).begin_unlocking_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BeginUnlockingAllSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Msg/BeginUnlocking" => {
                    #[allow(non_camel_case_types)]
                    struct BeginUnlockingSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBeginUnlocking> for BeginUnlockingSvc<T> {
                        type Response = super::MsgBeginUnlockingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBeginUnlocking>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).begin_unlocking(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BeginUnlockingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Msg/ExtendLockup" => {
                    #[allow(non_camel_case_types)]
                    struct ExtendLockupSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgExtendLockup> for ExtendLockupSvc<T> {
                        type Response = super::MsgExtendLockupResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgExtendLockup>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).extend_lockup(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExtendLockupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/osmosis.lockup.Msg/ForceUnlock" => {
                    #[allow(non_camel_case_types)]
                    struct ForceUnlockSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgForceUnlock> for ForceUnlockSvc<T> {
                        type Response = super::MsgForceUnlockResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgForceUnlock>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).force_unlock(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ForceUnlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
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
            }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "osmosis.lockup.Msg";
    }
}
