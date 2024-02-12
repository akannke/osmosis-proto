// @generated
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
        pub async fn join_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgJoinPool>,
        ) -> std::result::Result<
            tonic::Response<super::MsgJoinPoolResponse>,
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
                "/osmosis.gamm.v1beta1.Msg/JoinPool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.gamm.v1beta1.Msg", "JoinPool"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn exit_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExitPool>,
        ) -> std::result::Result<
            tonic::Response<super::MsgExitPoolResponse>,
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
                "/osmosis.gamm.v1beta1.Msg/ExitPool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.gamm.v1beta1.Msg", "ExitPool"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn swap_exact_amount_in(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSwapExactAmountIn>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSwapExactAmountInResponse>,
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
                "/osmosis.gamm.v1beta1.Msg/SwapExactAmountIn",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.gamm.v1beta1.Msg", "SwapExactAmountIn"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn swap_exact_amount_out(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSwapExactAmountOut>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSwapExactAmountOutResponse>,
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
                "/osmosis.gamm.v1beta1.Msg/SwapExactAmountOut",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.gamm.v1beta1.Msg", "SwapExactAmountOut"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn join_swap_extern_amount_in(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgJoinSwapExternAmountIn>,
        ) -> std::result::Result<
            tonic::Response<super::MsgJoinSwapExternAmountInResponse>,
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
                "/osmosis.gamm.v1beta1.Msg/JoinSwapExternAmountIn",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.gamm.v1beta1.Msg", "JoinSwapExternAmountIn"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn join_swap_share_amount_out(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgJoinSwapShareAmountOut>,
        ) -> std::result::Result<
            tonic::Response<super::MsgJoinSwapShareAmountOutResponse>,
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
                "/osmosis.gamm.v1beta1.Msg/JoinSwapShareAmountOut",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.gamm.v1beta1.Msg", "JoinSwapShareAmountOut"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn exit_swap_extern_amount_out(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExitSwapExternAmountOut>,
        ) -> std::result::Result<
            tonic::Response<super::MsgExitSwapExternAmountOutResponse>,
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
                "/osmosis.gamm.v1beta1.Msg/ExitSwapExternAmountOut",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.gamm.v1beta1.Msg",
                        "ExitSwapExternAmountOut",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn exit_swap_share_amount_in(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExitSwapShareAmountIn>,
        ) -> std::result::Result<
            tonic::Response<super::MsgExitSwapShareAmountInResponse>,
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
                "/osmosis.gamm.v1beta1.Msg/ExitSwapShareAmountIn",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.gamm.v1beta1.Msg", "ExitSwapShareAmountIn"),
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
        async fn join_pool(
            &self,
            request: tonic::Request<super::MsgJoinPool>,
        ) -> std::result::Result<
            tonic::Response<super::MsgJoinPoolResponse>,
            tonic::Status,
        >;
        async fn exit_pool(
            &self,
            request: tonic::Request<super::MsgExitPool>,
        ) -> std::result::Result<
            tonic::Response<super::MsgExitPoolResponse>,
            tonic::Status,
        >;
        async fn swap_exact_amount_in(
            &self,
            request: tonic::Request<super::MsgSwapExactAmountIn>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSwapExactAmountInResponse>,
            tonic::Status,
        >;
        async fn swap_exact_amount_out(
            &self,
            request: tonic::Request<super::MsgSwapExactAmountOut>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSwapExactAmountOutResponse>,
            tonic::Status,
        >;
        async fn join_swap_extern_amount_in(
            &self,
            request: tonic::Request<super::MsgJoinSwapExternAmountIn>,
        ) -> std::result::Result<
            tonic::Response<super::MsgJoinSwapExternAmountInResponse>,
            tonic::Status,
        >;
        async fn join_swap_share_amount_out(
            &self,
            request: tonic::Request<super::MsgJoinSwapShareAmountOut>,
        ) -> std::result::Result<
            tonic::Response<super::MsgJoinSwapShareAmountOutResponse>,
            tonic::Status,
        >;
        async fn exit_swap_extern_amount_out(
            &self,
            request: tonic::Request<super::MsgExitSwapExternAmountOut>,
        ) -> std::result::Result<
            tonic::Response<super::MsgExitSwapExternAmountOutResponse>,
            tonic::Status,
        >;
        async fn exit_swap_share_amount_in(
            &self,
            request: tonic::Request<super::MsgExitSwapShareAmountIn>,
        ) -> std::result::Result<
            tonic::Response<super::MsgExitSwapShareAmountInResponse>,
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
                "/osmosis.gamm.v1beta1.Msg/JoinPool" => {
                    #[allow(non_camel_case_types)]
                    struct JoinPoolSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgJoinPool>
                    for JoinPoolSvc<T> {
                        type Response = super::MsgJoinPoolResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgJoinPool>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).join_pool(request).await };
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
                        let method = JoinPoolSvc(inner);
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
                "/osmosis.gamm.v1beta1.Msg/ExitPool" => {
                    #[allow(non_camel_case_types)]
                    struct ExitPoolSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgExitPool>
                    for ExitPoolSvc<T> {
                        type Response = super::MsgExitPoolResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgExitPool>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).exit_pool(request).await };
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
                        let method = ExitPoolSvc(inner);
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
                "/osmosis.gamm.v1beta1.Msg/SwapExactAmountIn" => {
                    #[allow(non_camel_case_types)]
                    struct SwapExactAmountInSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSwapExactAmountIn>
                    for SwapExactAmountInSvc<T> {
                        type Response = super::MsgSwapExactAmountInResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSwapExactAmountIn>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).swap_exact_amount_in(request).await
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
                        let method = SwapExactAmountInSvc(inner);
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
                "/osmosis.gamm.v1beta1.Msg/SwapExactAmountOut" => {
                    #[allow(non_camel_case_types)]
                    struct SwapExactAmountOutSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgSwapExactAmountOut>
                    for SwapExactAmountOutSvc<T> {
                        type Response = super::MsgSwapExactAmountOutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSwapExactAmountOut>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).swap_exact_amount_out(request).await
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
                        let method = SwapExactAmountOutSvc(inner);
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
                "/osmosis.gamm.v1beta1.Msg/JoinSwapExternAmountIn" => {
                    #[allow(non_camel_case_types)]
                    struct JoinSwapExternAmountInSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgJoinSwapExternAmountIn>
                    for JoinSwapExternAmountInSvc<T> {
                        type Response = super::MsgJoinSwapExternAmountInResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgJoinSwapExternAmountIn>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).join_swap_extern_amount_in(request).await
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
                        let method = JoinSwapExternAmountInSvc(inner);
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
                "/osmosis.gamm.v1beta1.Msg/JoinSwapShareAmountOut" => {
                    #[allow(non_camel_case_types)]
                    struct JoinSwapShareAmountOutSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgJoinSwapShareAmountOut>
                    for JoinSwapShareAmountOutSvc<T> {
                        type Response = super::MsgJoinSwapShareAmountOutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgJoinSwapShareAmountOut>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).join_swap_share_amount_out(request).await
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
                        let method = JoinSwapShareAmountOutSvc(inner);
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
                "/osmosis.gamm.v1beta1.Msg/ExitSwapExternAmountOut" => {
                    #[allow(non_camel_case_types)]
                    struct ExitSwapExternAmountOutSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgExitSwapExternAmountOut>
                    for ExitSwapExternAmountOutSvc<T> {
                        type Response = super::MsgExitSwapExternAmountOutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgExitSwapExternAmountOut>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).exit_swap_extern_amount_out(request).await
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
                        let method = ExitSwapExternAmountOutSvc(inner);
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
                "/osmosis.gamm.v1beta1.Msg/ExitSwapShareAmountIn" => {
                    #[allow(non_camel_case_types)]
                    struct ExitSwapShareAmountInSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgExitSwapShareAmountIn>
                    for ExitSwapShareAmountInSvc<T> {
                        type Response = super::MsgExitSwapShareAmountInResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgExitSwapShareAmountIn>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).exit_swap_share_amount_in(request).await
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
                        let method = ExitSwapShareAmountInSvc(inner);
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
        const NAME: &'static str = "osmosis.gamm.v1beta1.Msg";
    }
}
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
        pub async fn pools(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPoolsResponse>,
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
                "/osmosis.gamm.v1beta1.Query/Pools",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.gamm.v1beta1.Query", "Pools"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn num_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryNumPoolsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryNumPoolsResponse>,
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
                "/osmosis.gamm.v1beta1.Query/NumPools",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.gamm.v1beta1.Query", "NumPools"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn total_liquidity(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTotalLiquidityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTotalLiquidityResponse>,
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
                "/osmosis.gamm.v1beta1.Query/TotalLiquidity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.gamm.v1beta1.Query", "TotalLiquidity"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pools_with_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolsWithFilterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPoolsWithFilterResponse>,
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
                "/osmosis.gamm.v1beta1.Query/PoolsWithFilter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.gamm.v1beta1.Query", "PoolsWithFilter"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn pool(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPoolResponse>,
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
                "/osmosis.gamm.v1beta1.Query/Pool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.gamm.v1beta1.Query", "Pool"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pool_type(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPoolTypeResponse>,
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
                "/osmosis.gamm.v1beta1.Query/PoolType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.gamm.v1beta1.Query", "PoolType"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn calc_join_pool_no_swap_shares(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCalcJoinPoolNoSwapSharesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCalcJoinPoolNoSwapSharesResponse>,
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
                "/osmosis.gamm.v1beta1.Query/CalcJoinPoolNoSwapShares",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.gamm.v1beta1.Query",
                        "CalcJoinPoolNoSwapShares",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn calc_join_pool_shares(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCalcJoinPoolSharesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCalcJoinPoolSharesResponse>,
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
                "/osmosis.gamm.v1beta1.Query/CalcJoinPoolShares",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.gamm.v1beta1.Query", "CalcJoinPoolShares"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn calc_exit_pool_coins_from_shares(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryCalcExitPoolCoinsFromSharesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryCalcExitPoolCoinsFromSharesResponse>,
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
                "/osmosis.gamm.v1beta1.Query/CalcExitPoolCoinsFromShares",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.gamm.v1beta1.Query",
                        "CalcExitPoolCoinsFromShares",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn pool_params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPoolParamsResponse>,
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
                "/osmosis.gamm.v1beta1.Query/PoolParams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.gamm.v1beta1.Query", "PoolParams"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn total_pool_liquidity(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTotalPoolLiquidityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTotalPoolLiquidityResponse>,
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
                "/osmosis.gamm.v1beta1.Query/TotalPoolLiquidity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("osmosis.gamm.v1beta1.Query", "TotalPoolLiquidity"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn total_shares(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTotalSharesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTotalSharesResponse>,
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
                "/osmosis.gamm.v1beta1.Query/TotalShares",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.gamm.v1beta1.Query", "TotalShares"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn spot_price(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySpotPriceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySpotPriceResponse>,
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
                "/osmosis.gamm.v1beta1.Query/SpotPrice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("osmosis.gamm.v1beta1.Query", "SpotPrice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_swap_exact_amount_in(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySwapExactAmountInRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySwapExactAmountInResponse>,
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
                "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountIn",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.gamm.v1beta1.Query",
                        "EstimateSwapExactAmountIn",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_swap_exact_amount_out(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySwapExactAmountOutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySwapExactAmountOutResponse>,
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
                "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountOut",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.gamm.v1beta1.Query",
                        "EstimateSwapExactAmountOut",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn concentrated_pool_id_link_from_cfmm(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryConcentratedPoolIdLinkFromCfmmRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryConcentratedPoolIdLinkFromCfmmResponse>,
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
                "/osmosis.gamm.v1beta1.Query/ConcentratedPoolIdLinkFromCFMM",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.gamm.v1beta1.Query",
                        "ConcentratedPoolIdLinkFromCFMM",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn cfmm_concentrated_pool_links(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryCfmmConcentratedPoolLinksRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QueryCfmmConcentratedPoolLinksResponse>,
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
                "/osmosis.gamm.v1beta1.Query/CFMMConcentratedPoolLinks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "osmosis.gamm.v1beta1.Query",
                        "CFMMConcentratedPoolLinks",
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
        async fn pools(
            &self,
            request: tonic::Request<super::QueryPoolsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPoolsResponse>,
            tonic::Status,
        >;
        async fn num_pools(
            &self,
            request: tonic::Request<super::QueryNumPoolsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryNumPoolsResponse>,
            tonic::Status,
        >;
        async fn total_liquidity(
            &self,
            request: tonic::Request<super::QueryTotalLiquidityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTotalLiquidityResponse>,
            tonic::Status,
        >;
        async fn pools_with_filter(
            &self,
            request: tonic::Request<super::QueryPoolsWithFilterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPoolsWithFilterResponse>,
            tonic::Status,
        >;
        async fn pool(
            &self,
            request: tonic::Request<super::QueryPoolRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPoolResponse>,
            tonic::Status,
        >;
        async fn pool_type(
            &self,
            request: tonic::Request<super::QueryPoolTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPoolTypeResponse>,
            tonic::Status,
        >;
        async fn calc_join_pool_no_swap_shares(
            &self,
            request: tonic::Request<super::QueryCalcJoinPoolNoSwapSharesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCalcJoinPoolNoSwapSharesResponse>,
            tonic::Status,
        >;
        async fn calc_join_pool_shares(
            &self,
            request: tonic::Request<super::QueryCalcJoinPoolSharesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCalcJoinPoolSharesResponse>,
            tonic::Status,
        >;
        async fn calc_exit_pool_coins_from_shares(
            &self,
            request: tonic::Request<super::QueryCalcExitPoolCoinsFromSharesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCalcExitPoolCoinsFromSharesResponse>,
            tonic::Status,
        >;
        async fn pool_params(
            &self,
            request: tonic::Request<super::QueryPoolParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPoolParamsResponse>,
            tonic::Status,
        >;
        async fn total_pool_liquidity(
            &self,
            request: tonic::Request<super::QueryTotalPoolLiquidityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTotalPoolLiquidityResponse>,
            tonic::Status,
        >;
        async fn total_shares(
            &self,
            request: tonic::Request<super::QueryTotalSharesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTotalSharesResponse>,
            tonic::Status,
        >;
        async fn spot_price(
            &self,
            request: tonic::Request<super::QuerySpotPriceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySpotPriceResponse>,
            tonic::Status,
        >;
        async fn estimate_swap_exact_amount_in(
            &self,
            request: tonic::Request<super::QuerySwapExactAmountInRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySwapExactAmountInResponse>,
            tonic::Status,
        >;
        async fn estimate_swap_exact_amount_out(
            &self,
            request: tonic::Request<super::QuerySwapExactAmountOutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuerySwapExactAmountOutResponse>,
            tonic::Status,
        >;
        async fn concentrated_pool_id_link_from_cfmm(
            &self,
            request: tonic::Request<super::QueryConcentratedPoolIdLinkFromCfmmRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryConcentratedPoolIdLinkFromCfmmResponse>,
            tonic::Status,
        >;
        async fn cfmm_concentrated_pool_links(
            &self,
            request: tonic::Request<super::QueryCfmmConcentratedPoolLinksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCfmmConcentratedPoolLinksResponse>,
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
                "/osmosis.gamm.v1beta1.Query/Pools" => {
                    #[allow(non_camel_case_types)]
                    struct PoolsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryPoolsRequest>
                    for PoolsSvc<T> {
                        type Response = super::QueryPoolsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).pools(request).await };
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
                        let method = PoolsSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/NumPools" => {
                    #[allow(non_camel_case_types)]
                    struct NumPoolsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryNumPoolsRequest>
                    for NumPoolsSvc<T> {
                        type Response = super::QueryNumPoolsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryNumPoolsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).num_pools(request).await };
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
                        let method = NumPoolsSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/TotalLiquidity" => {
                    #[allow(non_camel_case_types)]
                    struct TotalLiquiditySvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryTotalLiquidityRequest>
                    for TotalLiquiditySvc<T> {
                        type Response = super::QueryTotalLiquidityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTotalLiquidityRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).total_liquidity(request).await
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
                        let method = TotalLiquiditySvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/PoolsWithFilter" => {
                    #[allow(non_camel_case_types)]
                    struct PoolsWithFilterSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryPoolsWithFilterRequest>
                    for PoolsWithFilterSvc<T> {
                        type Response = super::QueryPoolsWithFilterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolsWithFilterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).pools_with_filter(request).await
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
                        let method = PoolsWithFilterSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/Pool" => {
                    #[allow(non_camel_case_types)]
                    struct PoolSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryPoolRequest>
                    for PoolSvc<T> {
                        type Response = super::QueryPoolResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).pool(request).await };
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
                        let method = PoolSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/PoolType" => {
                    #[allow(non_camel_case_types)]
                    struct PoolTypeSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryPoolTypeRequest>
                    for PoolTypeSvc<T> {
                        type Response = super::QueryPoolTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolTypeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).pool_type(request).await };
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
                        let method = PoolTypeSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/CalcJoinPoolNoSwapShares" => {
                    #[allow(non_camel_case_types)]
                    struct CalcJoinPoolNoSwapSharesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryCalcJoinPoolNoSwapSharesRequest,
                    > for CalcJoinPoolNoSwapSharesSvc<T> {
                        type Response = super::QueryCalcJoinPoolNoSwapSharesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryCalcJoinPoolNoSwapSharesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).calc_join_pool_no_swap_shares(request).await
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
                        let method = CalcJoinPoolNoSwapSharesSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/CalcJoinPoolShares" => {
                    #[allow(non_camel_case_types)]
                    struct CalcJoinPoolSharesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryCalcJoinPoolSharesRequest>
                    for CalcJoinPoolSharesSvc<T> {
                        type Response = super::QueryCalcJoinPoolSharesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryCalcJoinPoolSharesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).calc_join_pool_shares(request).await
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
                        let method = CalcJoinPoolSharesSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/CalcExitPoolCoinsFromShares" => {
                    #[allow(non_camel_case_types)]
                    struct CalcExitPoolCoinsFromSharesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryCalcExitPoolCoinsFromSharesRequest,
                    > for CalcExitPoolCoinsFromSharesSvc<T> {
                        type Response = super::QueryCalcExitPoolCoinsFromSharesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryCalcExitPoolCoinsFromSharesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).calc_exit_pool_coins_from_shares(request).await
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
                        let method = CalcExitPoolCoinsFromSharesSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/PoolParams" => {
                    #[allow(non_camel_case_types)]
                    struct PoolParamsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryPoolParamsRequest>
                    for PoolParamsSvc<T> {
                        type Response = super::QueryPoolParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPoolParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).pool_params(request).await };
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
                        let method = PoolParamsSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/TotalPoolLiquidity" => {
                    #[allow(non_camel_case_types)]
                    struct TotalPoolLiquiditySvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryTotalPoolLiquidityRequest>
                    for TotalPoolLiquiditySvc<T> {
                        type Response = super::QueryTotalPoolLiquidityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryTotalPoolLiquidityRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).total_pool_liquidity(request).await
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
                        let method = TotalPoolLiquiditySvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/TotalShares" => {
                    #[allow(non_camel_case_types)]
                    struct TotalSharesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryTotalSharesRequest>
                    for TotalSharesSvc<T> {
                        type Response = super::QueryTotalSharesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTotalSharesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).total_shares(request).await
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
                        let method = TotalSharesSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/SpotPrice" => {
                    #[allow(non_camel_case_types)]
                    struct SpotPriceSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QuerySpotPriceRequest>
                    for SpotPriceSvc<T> {
                        type Response = super::QuerySpotPriceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySpotPriceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).spot_price(request).await };
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
                        let method = SpotPriceSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountIn" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateSwapExactAmountInSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QuerySwapExactAmountInRequest>
                    for EstimateSwapExactAmountInSvc<T> {
                        type Response = super::QuerySwapExactAmountInResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySwapExactAmountInRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).estimate_swap_exact_amount_in(request).await
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
                        let method = EstimateSwapExactAmountInSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountOut" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateSwapExactAmountOutSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QuerySwapExactAmountOutRequest>
                    for EstimateSwapExactAmountOutSvc<T> {
                        type Response = super::QuerySwapExactAmountOutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QuerySwapExactAmountOutRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).estimate_swap_exact_amount_out(request).await
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
                        let method = EstimateSwapExactAmountOutSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/ConcentratedPoolIdLinkFromCFMM" => {
                    #[allow(non_camel_case_types)]
                    struct ConcentratedPoolIdLinkFromCFMMSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryConcentratedPoolIdLinkFromCfmmRequest,
                    > for ConcentratedPoolIdLinkFromCFMMSvc<T> {
                        type Response = super::QueryConcentratedPoolIdLinkFromCfmmResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryConcentratedPoolIdLinkFromCfmmRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).concentrated_pool_id_link_from_cfmm(request).await
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
                        let method = ConcentratedPoolIdLinkFromCFMMSvc(inner);
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
                "/osmosis.gamm.v1beta1.Query/CFMMConcentratedPoolLinks" => {
                    #[allow(non_camel_case_types)]
                    struct CFMMConcentratedPoolLinksSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryCfmmConcentratedPoolLinksRequest,
                    > for CFMMConcentratedPoolLinksSvc<T> {
                        type Response = super::QueryCfmmConcentratedPoolLinksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryCfmmConcentratedPoolLinksRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).cfmm_concentrated_pool_links(request).await
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
                        let method = CFMMConcentratedPoolLinksSvc(inner);
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
        const NAME: &'static str = "osmosis.gamm.v1beta1.Query";
    }
}
