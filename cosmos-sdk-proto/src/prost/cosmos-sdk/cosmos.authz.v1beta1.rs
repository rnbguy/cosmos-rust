/// GenericAuthorization gives the grantee unrestricted permissions to execute
/// the provided method on behalf of the granter's account.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericAuthorization {
    /// Msg, identified by it's type URL, to grant unrestricted permissions to execute
    #[prost(string, tag = "1")]
    pub msg: ::prost::alloc::string::String,
}
/// Grant gives permissions to execute
/// the provide method with expiration time.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Grant {
    #[prost(message, optional, tag = "1")]
    pub authorization: ::core::option::Option<::prost_wkt_types::Any>,
    /// time when the grant will expire and will be pruned. If null, then the grant
    /// doesn't have a time expiration (other conditions  in `authorization`
    /// may apply to invalidate the grant)
    #[prost(message, optional, tag = "2")]
    pub expiration: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
/// GrantAuthorization extends a grant with both the addresses of the grantee and granter.
/// It is used in genesis.proto and query.proto
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantAuthorization {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub authorization: ::core::option::Option<::prost_wkt_types::Any>,
    #[prost(message, optional, tag = "4")]
    pub expiration: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
/// GrantQueueItem contains the list of TypeURL of a sdk.Msg.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantQueueItem {
    /// msg_type_urls contains the list of TypeURL of a sdk.Msg.
    #[prost(string, repeated, tag = "1")]
    pub msg_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventGrant is emitted on Msg/Grant
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventGrant {
    /// Msg type URL for which an autorization is granted
    #[prost(string, tag = "2")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// Granter account address
    #[prost(string, tag = "3")]
    pub granter: ::prost::alloc::string::String,
    /// Grantee account address
    #[prost(string, tag = "4")]
    pub grantee: ::prost::alloc::string::String,
}
/// EventRevoke is emitted on Msg/Revoke
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRevoke {
    /// Msg type URL for which an autorization is revoked
    #[prost(string, tag = "2")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// Granter account address
    #[prost(string, tag = "3")]
    pub granter: ::prost::alloc::string::String,
    /// Grantee account address
    #[prost(string, tag = "4")]
    pub grantee: ::prost::alloc::string::String,
}
/// QueryGrantsRequest is the request type for the Query/Grants RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantsRequest {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// Optional, msg_type_url, when set, will query only grants matching given msg type.
    #[prost(string, tag = "3")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGrantsResponse is the response type for the Query/Authorizations RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantsResponse {
    /// authorizations is a list of grants granted for grantee by granter.
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryGranterGrantsRequest is the request type for the Query/GranterGrants RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGranterGrantsRequest {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGranterGrantsResponse is the response type for the Query/GranterGrants RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGranterGrantsResponse {
    /// grants is a list of grants granted by the granter.
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<GrantAuthorization>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryGranteeGrantsRequest is the request type for the Query/IssuedGrants RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGranteeGrantsRequest {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGranteeGrantsResponse is the response type for the Query/GranteeGrants RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGranteeGrantsResponse {
    /// grants is a list of grants granted to the grantee.
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<GrantAuthorization>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC querier service.
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
        /// Returns list of `Authorization`, granted to the grantee by the granter.
        pub async fn grants(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGrantsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGrantsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.authz.v1beta1.Query/Grants");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.authz.v1beta1.Query", "Grants"));
            self.inner.unary(req, path, codec).await
        }
        /// GranterGrants returns list of `GrantAuthorization`, granted by granter.
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn granter_grants(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGranterGrantsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGranterGrantsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.authz.v1beta1.Query/GranterGrants");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.authz.v1beta1.Query",
                "GranterGrants",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// GranteeGrants returns a list of `GrantAuthorization` by grantee.
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn grantee_grants(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGranteeGrantsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGranteeGrantsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.authz.v1beta1.Query/GranteeGrants");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.authz.v1beta1.Query",
                "GranteeGrants",
            ));
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
        /// Returns list of `Authorization`, granted to the grantee by the granter.
        async fn grants(
            &self,
            request: tonic::Request<super::QueryGrantsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGrantsResponse>, tonic::Status>;
        /// GranterGrants returns list of `GrantAuthorization`, granted by granter.
        ///
        /// Since: cosmos-sdk 0.46
        async fn granter_grants(
            &self,
            request: tonic::Request<super::QueryGranterGrantsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGranterGrantsResponse>, tonic::Status>;
        /// GranteeGrants returns a list of `GrantAuthorization` by grantee.
        ///
        /// Since: cosmos-sdk 0.46
        async fn grantee_grants(
            &self,
            request: tonic::Request<super::QueryGranteeGrantsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGranteeGrantsResponse>, tonic::Status>;
    }
    /// Query defines the gRPC querier service.
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
                "/cosmos.authz.v1beta1.Query/Grants" => {
                    #[allow(non_camel_case_types)]
                    struct GrantsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGrantsRequest> for GrantsSvc<T> {
                        type Response = super::QueryGrantsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGrantsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).grants(request).await };
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
                        let method = GrantsSvc(inner);
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
                "/cosmos.authz.v1beta1.Query/GranterGrants" => {
                    #[allow(non_camel_case_types)]
                    struct GranterGrantsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGranterGrantsRequest>
                        for GranterGrantsSvc<T>
                    {
                        type Response = super::QueryGranterGrantsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGranterGrantsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).granter_grants(request).await };
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
                        let method = GranterGrantsSvc(inner);
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
                "/cosmos.authz.v1beta1.Query/GranteeGrants" => {
                    #[allow(non_camel_case_types)]
                    struct GranteeGrantsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGranteeGrantsRequest>
                        for GranteeGrantsSvc<T>
                    {
                        type Response = super::QueryGranteeGrantsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGranteeGrantsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).grantee_grants(request).await };
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
                        let method = GranteeGrantsSvc(inner);
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
        const NAME: &'static str = "cosmos.authz.v1beta1.Query";
    }
}
/// MsgGrant is a request type for Grant method. It declares authorization to the grantee
/// on behalf of the granter with the provided expiration time.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrant {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub grant: ::core::option::Option<Grant>,
}
/// MsgExecResponse defines the Msg/MsgExecResponse response type.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// MsgExec attempts to execute the provided messages using
/// authorizations granted to the grantee. Each message should have only
/// one signer corresponding to the granter of the authorization.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExec {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    /// Authorization Msg requests to execute. Each msg must implement Authorization interface
    /// The x/authz will try to find a grant matching (msg.signers\[0\], grantee, MsgTypeURL(msg))
    /// triple and validate it.
    #[prost(message, repeated, tag = "2")]
    pub msgs: ::prost::alloc::vec::Vec<::prost_wkt_types::Any>,
}
/// MsgGrantResponse defines the Msg/MsgGrant response type.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrantResponse {}
/// MsgRevoke revokes any authorization with the provided sdk.Msg type on the
/// granter's account with that has been granted to the grantee.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevoke {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub msg_type_url: ::prost::alloc::string::String,
}
/// MsgRevokeResponse defines the Msg/MsgRevokeResponse response type.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the authz Msg service.
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
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
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
        /// Grant grants the provided authorization to the grantee on the granter's
        /// account with the provided expiration time. If there is already a grant
        /// for the given (granter, grantee, Authorization) triple, then the grant
        /// will be overwritten.
        pub async fn grant(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGrant>,
        ) -> std::result::Result<tonic::Response<super::MsgGrantResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.authz.v1beta1.Msg/Grant");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.authz.v1beta1.Msg", "Grant"));
            self.inner.unary(req, path, codec).await
        }
        /// Exec attempts to execute the provided messages using
        /// authorizations granted to the grantee. Each message should have only
        /// one signer corresponding to the granter of the authorization.
        pub async fn exec(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExec>,
        ) -> std::result::Result<tonic::Response<super::MsgExecResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.authz.v1beta1.Msg/Exec");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.authz.v1beta1.Msg", "Exec"));
            self.inner.unary(req, path, codec).await
        }
        /// Revoke revokes any authorization corresponding to the provided method name on the
        /// granter's account that has been granted to the grantee.
        pub async fn revoke(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRevoke>,
        ) -> std::result::Result<tonic::Response<super::MsgRevokeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.authz.v1beta1.Msg/Revoke");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.authz.v1beta1.Msg", "Revoke"));
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
        /// Grant grants the provided authorization to the grantee on the granter's
        /// account with the provided expiration time. If there is already a grant
        /// for the given (granter, grantee, Authorization) triple, then the grant
        /// will be overwritten.
        async fn grant(
            &self,
            request: tonic::Request<super::MsgGrant>,
        ) -> std::result::Result<tonic::Response<super::MsgGrantResponse>, tonic::Status>;
        /// Exec attempts to execute the provided messages using
        /// authorizations granted to the grantee. Each message should have only
        /// one signer corresponding to the granter of the authorization.
        async fn exec(
            &self,
            request: tonic::Request<super::MsgExec>,
        ) -> std::result::Result<tonic::Response<super::MsgExecResponse>, tonic::Status>;
        /// Revoke revokes any authorization corresponding to the provided method name on the
        /// granter's account that has been granted to the grantee.
        async fn revoke(
            &self,
            request: tonic::Request<super::MsgRevoke>,
        ) -> std::result::Result<tonic::Response<super::MsgRevokeResponse>, tonic::Status>;
    }
    /// Msg defines the authz Msg service.
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
                "/cosmos.authz.v1beta1.Msg/Grant" => {
                    #[allow(non_camel_case_types)]
                    struct GrantSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGrant> for GrantSvc<T> {
                        type Response = super::MsgGrantResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGrant>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).grant(request).await };
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
                        let method = GrantSvc(inner);
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
                "/cosmos.authz.v1beta1.Msg/Exec" => {
                    #[allow(non_camel_case_types)]
                    struct ExecSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgExec> for ExecSvc<T> {
                        type Response = super::MsgExecResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgExec>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).exec(request).await };
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
                        let method = ExecSvc(inner);
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
                "/cosmos.authz.v1beta1.Msg/Revoke" => {
                    #[allow(non_camel_case_types)]
                    struct RevokeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgRevoke> for RevokeSvc<T> {
                        type Response = super::MsgRevokeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRevoke>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).revoke(request).await };
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
                        let method = RevokeSvc(inner);
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
        const NAME: &'static str = "cosmos.authz.v1beta1.Msg";
    }
}
/// GenesisState defines the authz module's genesis state.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub authorization: ::prost::alloc::vec::Vec<GrantAuthorization>,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GENERIC_AUTHORIZATION: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.GenericAuthorization")]
    impl ::prost_wkt::MessageSerde for GenericAuthorization {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "GenericAuthorization"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.GenericAuthorization"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.GenericAuthorization" , decoder : | buf : & [u8] | { let msg : GenericAuthorization = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GRANT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.Grant")]
    impl ::prost_wkt::MessageSerde for Grant {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "Grant"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.Grant"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.Grant" , decoder : | buf : & [u8] | { let msg : Grant = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GRANT_AUTHORIZATION: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.GrantAuthorization")]
    impl ::prost_wkt::MessageSerde for GrantAuthorization {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "GrantAuthorization"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.GrantAuthorization"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.GrantAuthorization" , decoder : | buf : & [u8] | { let msg : GrantAuthorization = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GRANT_QUEUE_ITEM: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.GrantQueueItem")]
    impl ::prost_wkt::MessageSerde for GrantQueueItem {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "GrantQueueItem"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.GrantQueueItem"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.GrantQueueItem" , decoder : | buf : & [u8] | { let msg : GrantQueueItem = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_EVENT_GRANT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.EventGrant")]
    impl ::prost_wkt::MessageSerde for EventGrant {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "EventGrant"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.EventGrant"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.EventGrant" , decoder : | buf : & [u8] | { let msg : EventGrant = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_EVENT_REVOKE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.EventRevoke")]
    impl ::prost_wkt::MessageSerde for EventRevoke {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "EventRevoke"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.EventRevoke"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.EventRevoke" , decoder : | buf : & [u8] | { let msg : EventRevoke = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_GRANTS_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.QueryGrantsRequest")]
    impl ::prost_wkt::MessageSerde for QueryGrantsRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryGrantsRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.QueryGrantsRequest"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.QueryGrantsRequest" , decoder : | buf : & [u8] | { let msg : QueryGrantsRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_GRANTS_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.QueryGrantsResponse")]
    impl ::prost_wkt::MessageSerde for QueryGrantsResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryGrantsResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.QueryGrantsResponse"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.QueryGrantsResponse" , decoder : | buf : & [u8] | { let msg : QueryGrantsResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_GRANTER_GRANTS_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.QueryGranterGrantsRequest")]
    impl ::prost_wkt::MessageSerde for QueryGranterGrantsRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryGranterGrantsRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.QueryGranterGrantsRequest"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.QueryGranterGrantsRequest" , decoder : | buf : & [u8] | { let msg : QueryGranterGrantsRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_GRANTER_GRANTS_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.QueryGranterGrantsResponse")]
    impl ::prost_wkt::MessageSerde for QueryGranterGrantsResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryGranterGrantsResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.QueryGranterGrantsResponse"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.QueryGranterGrantsResponse" , decoder : | buf : & [u8] | { let msg : QueryGranterGrantsResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_GRANTEE_GRANTS_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.QueryGranteeGrantsRequest")]
    impl ::prost_wkt::MessageSerde for QueryGranteeGrantsRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryGranteeGrantsRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.QueryGranteeGrantsRequest"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.QueryGranteeGrantsRequest" , decoder : | buf : & [u8] | { let msg : QueryGranteeGrantsRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_GRANTEE_GRANTS_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.QueryGranteeGrantsResponse")]
    impl ::prost_wkt::MessageSerde for QueryGranteeGrantsResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryGranteeGrantsResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.QueryGranteeGrantsResponse"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.QueryGranteeGrantsResponse" , decoder : | buf : & [u8] | { let msg : QueryGranteeGrantsResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_GRANT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.MsgGrant")]
    impl ::prost_wkt::MessageSerde for MsgGrant {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgGrant"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.MsgGrant"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.MsgGrant" , decoder : | buf : & [u8] | { let msg : MsgGrant = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_EXEC_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.MsgExecResponse")]
    impl ::prost_wkt::MessageSerde for MsgExecResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgExecResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.MsgExecResponse"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.MsgExecResponse" , decoder : | buf : & [u8] | { let msg : MsgExecResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_EXEC: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.MsgExec")]
    impl ::prost_wkt::MessageSerde for MsgExec {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgExec"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.MsgExec"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.MsgExec" , decoder : | buf : & [u8] | { let msg : MsgExec = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_GRANT_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.MsgGrantResponse")]
    impl ::prost_wkt::MessageSerde for MsgGrantResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgGrantResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.MsgGrantResponse"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.MsgGrantResponse" , decoder : | buf : & [u8] | { let msg : MsgGrantResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_REVOKE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.MsgRevoke")]
    impl ::prost_wkt::MessageSerde for MsgRevoke {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgRevoke"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.MsgRevoke"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.MsgRevoke" , decoder : | buf : & [u8] | { let msg : MsgRevoke = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_REVOKE_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.MsgRevokeResponse")]
    impl ::prost_wkt::MessageSerde for MsgRevokeResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgRevokeResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.MsgRevokeResponse"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.MsgRevokeResponse" , decoder : | buf : & [u8] | { let msg : MsgRevokeResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.authz.v1beta1.GenesisState")]
    impl ::prost_wkt::MessageSerde for GenesisState {
        fn package_name(&self) -> &'static str {
            "cosmos.authz.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "GenesisState"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.authz.v1beta1.GenesisState"
        }
        fn new_instance(
            &self,
            data: Vec<u8>,
        ) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError>
        {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> =
                ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.authz.v1beta1.GenesisState" , decoder : | buf : & [u8] | { let msg : GenesisState = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
