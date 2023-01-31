/// BaseVestingAccount implements the VestingAccount interface. It contains all
/// the necessary fields needed for any vesting account implementation.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account: ::core::option::Option<super::super::auth::v1beta1::BaseAccount>,
    #[prost(message, repeated, tag = "2")]
    pub original_vesting: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "3")]
    pub delegated_free: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "4")]
    pub delegated_vesting: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// Vesting end time, as unix timestamp (in seconds).
    #[prost(int64, tag = "5")]
    pub end_time: i64,
}
/// ContinuousVestingAccount implements the VestingAccount interface. It
/// continuously vests by unlocking coins linearly with respect to time.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContinuousVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
    /// Vesting start time, as unix timestamp (in seconds).
    #[prost(int64, tag = "2")]
    pub start_time: i64,
}
/// DelayedVestingAccount implements the VestingAccount interface. It vests all
/// coins after a specific time, but non prior. In other words, it keeps them
/// locked until a specified time.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelayedVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
}
/// Period defines a length of time and amount of coins that will vest.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Period {
    /// Period duration in seconds.
    #[prost(int64, tag = "1")]
    pub length: i64,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// PeriodicVestingAccount implements the VestingAccount interface. It
/// periodically vests by unlocking coins during each specified period.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeriodicVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
    #[prost(int64, tag = "2")]
    pub start_time: i64,
    #[prost(message, repeated, tag = "3")]
    pub vesting_periods: ::prost::alloc::vec::Vec<Period>,
}
/// PermanentLockedAccount implements the VestingAccount interface. It does
/// not ever release coins, locking them indefinitely. Coins in this account can
/// still be used for delegating and for governance votes even while locked.
///
/// Since: cosmos-sdk 0.43
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermanentLockedAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
}
/// MsgCreateVestingAccount defines a message that enables creating a vesting
/// account.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateVestingAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// end of vesting as unix time (in seconds).
    #[prost(int64, tag = "4")]
    pub end_time: i64,
    #[prost(bool, tag = "5")]
    pub delayed: bool,
}
/// MsgCreateVestingAccountResponse defines the Msg/CreateVestingAccount response type.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateVestingAccountResponse {}
/// MsgCreatePermanentLockedAccount defines a message that enables creating a permanent
/// locked account.
///
/// Since: cosmos-sdk 0.46
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePermanentLockedAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgCreatePermanentLockedAccountResponse defines the Msg/CreatePermanentLockedAccount response type.
///
/// Since: cosmos-sdk 0.46
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePermanentLockedAccountResponse {}
/// MsgCreateVestingAccount defines a message that enables creating a vesting
/// account.
///
/// Since: cosmos-sdk 0.46
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePeriodicVestingAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    /// start of vesting as unix time (in seconds).
    #[prost(int64, tag = "3")]
    pub start_time: i64,
    #[prost(message, repeated, tag = "4")]
    pub vesting_periods: ::prost::alloc::vec::Vec<Period>,
}
/// MsgCreateVestingAccountResponse defines the Msg/CreatePeriodicVestingAccount
/// response type.
///
/// Since: cosmos-sdk 0.46
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePeriodicVestingAccountResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the bank Msg service.
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
        /// CreateVestingAccount defines a method that enables creating a vesting
        /// account.
        pub async fn create_vesting_account(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateVestingAccount>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateVestingAccountResponse>,
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
                "/cosmos.vesting.v1beta1.Msg/CreateVestingAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.vesting.v1beta1.Msg",
                "CreateVestingAccount",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// CreatePermanentLockedAccount defines a method that enables creating a permanent
        /// locked account.
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn create_permanent_locked_account(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreatePermanentLockedAccount>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreatePermanentLockedAccountResponse>,
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
                "/cosmos.vesting.v1beta1.Msg/CreatePermanentLockedAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.vesting.v1beta1.Msg",
                "CreatePermanentLockedAccount",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// CreatePeriodicVestingAccount defines a method that enables creating a
        /// periodic vesting account.
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn create_periodic_vesting_account(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreatePeriodicVestingAccount>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreatePeriodicVestingAccountResponse>,
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
                "/cosmos.vesting.v1beta1.Msg/CreatePeriodicVestingAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.vesting.v1beta1.Msg",
                "CreatePeriodicVestingAccount",
            ));
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
        /// CreateVestingAccount defines a method that enables creating a vesting
        /// account.
        async fn create_vesting_account(
            &self,
            request: tonic::Request<super::MsgCreateVestingAccount>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateVestingAccountResponse>,
            tonic::Status,
        >;
        /// CreatePermanentLockedAccount defines a method that enables creating a permanent
        /// locked account.
        ///
        /// Since: cosmos-sdk 0.46
        async fn create_permanent_locked_account(
            &self,
            request: tonic::Request<super::MsgCreatePermanentLockedAccount>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreatePermanentLockedAccountResponse>,
            tonic::Status,
        >;
        /// CreatePeriodicVestingAccount defines a method that enables creating a
        /// periodic vesting account.
        ///
        /// Since: cosmos-sdk 0.46
        async fn create_periodic_vesting_account(
            &self,
            request: tonic::Request<super::MsgCreatePeriodicVestingAccount>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreatePeriodicVestingAccountResponse>,
            tonic::Status,
        >;
    }
    /// Msg defines the bank Msg service.
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
                "/cosmos.vesting.v1beta1.Msg/CreateVestingAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreateVestingAccountSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateVestingAccount>
                        for CreateVestingAccountSvc<T>
                    {
                        type Response = super::MsgCreateVestingAccountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateVestingAccount>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_vesting_account(request).await };
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
                        let method = CreateVestingAccountSvc(inner);
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
                "/cosmos.vesting.v1beta1.Msg/CreatePermanentLockedAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePermanentLockedAccountSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreatePermanentLockedAccount>
                        for CreatePermanentLockedAccountSvc<T>
                    {
                        type Response = super::MsgCreatePermanentLockedAccountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreatePermanentLockedAccount>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_permanent_locked_account(request).await
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
                        let method = CreatePermanentLockedAccountSvc(inner);
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
                "/cosmos.vesting.v1beta1.Msg/CreatePeriodicVestingAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePeriodicVestingAccountSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreatePeriodicVestingAccount>
                        for CreatePeriodicVestingAccountSvc<T>
                    {
                        type Response = super::MsgCreatePeriodicVestingAccountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreatePeriodicVestingAccount>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_periodic_vesting_account(request).await
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
                        let method = CreatePeriodicVestingAccountSvc(inner);
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
        const NAME: &'static str = "cosmos.vesting.v1beta1.Msg";
    }
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_BASE_VESTING_ACCOUNT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.vesting.v1beta1.BaseVestingAccount")]
    impl ::prost_wkt::MessageSerde for BaseVestingAccount {
        fn package_name(&self) -> &'static str {
            "cosmos.vesting.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "BaseVestingAccount"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.vesting.v1beta1.BaseVestingAccount"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.vesting.v1beta1.BaseVestingAccount" , decoder : | buf : & [u8] | { let msg : BaseVestingAccount = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_CONTINUOUS_VESTING_ACCOUNT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.vesting.v1beta1.ContinuousVestingAccount")]
    impl ::prost_wkt::MessageSerde for ContinuousVestingAccount {
        fn package_name(&self) -> &'static str {
            "cosmos.vesting.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "ContinuousVestingAccount"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.vesting.v1beta1.ContinuousVestingAccount"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.vesting.v1beta1.ContinuousVestingAccount" , decoder : | buf : & [u8] | { let msg : ContinuousVestingAccount = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_DELAYED_VESTING_ACCOUNT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.vesting.v1beta1.DelayedVestingAccount")]
    impl ::prost_wkt::MessageSerde for DelayedVestingAccount {
        fn package_name(&self) -> &'static str {
            "cosmos.vesting.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "DelayedVestingAccount"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.vesting.v1beta1.DelayedVestingAccount"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.vesting.v1beta1.DelayedVestingAccount" , decoder : | buf : & [u8] | { let msg : DelayedVestingAccount = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_PERIOD: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.vesting.v1beta1.Period")]
    impl ::prost_wkt::MessageSerde for Period {
        fn package_name(&self) -> &'static str {
            "cosmos.vesting.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "Period"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.vesting.v1beta1.Period"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.vesting.v1beta1.Period" , decoder : | buf : & [u8] | { let msg : Period = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_PERIODIC_VESTING_ACCOUNT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.vesting.v1beta1.PeriodicVestingAccount")]
    impl ::prost_wkt::MessageSerde for PeriodicVestingAccount {
        fn package_name(&self) -> &'static str {
            "cosmos.vesting.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "PeriodicVestingAccount"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.vesting.v1beta1.PeriodicVestingAccount"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.vesting.v1beta1.PeriodicVestingAccount" , decoder : | buf : & [u8] | { let msg : PeriodicVestingAccount = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_PERMANENT_LOCKED_ACCOUNT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.vesting.v1beta1.PermanentLockedAccount")]
    impl ::prost_wkt::MessageSerde for PermanentLockedAccount {
        fn package_name(&self) -> &'static str {
            "cosmos.vesting.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "PermanentLockedAccount"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.vesting.v1beta1.PermanentLockedAccount"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.vesting.v1beta1.PermanentLockedAccount" , decoder : | buf : & [u8] | { let msg : PermanentLockedAccount = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_VESTING_ACCOUNT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreateVestingAccount")]
    impl ::prost_wkt::MessageSerde for MsgCreateVestingAccount {
        fn package_name(&self) -> &'static str {
            "cosmos.vesting.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgCreateVestingAccount"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreateVestingAccount"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreateVestingAccount" , decoder : | buf : & [u8] | { let msg : MsgCreateVestingAccount = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_VESTING_ACCOUNT_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreateVestingAccountResponse"
    )]
    impl ::prost_wkt::MessageSerde for MsgCreateVestingAccountResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.vesting.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgCreateVestingAccountResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreateVestingAccountResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreateVestingAccountResponse" , decoder : | buf : & [u8] | { let msg : MsgCreateVestingAccountResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_PERMANENT_LOCKED_ACCOUNT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreatePermanentLockedAccount"
    )]
    impl ::prost_wkt::MessageSerde for MsgCreatePermanentLockedAccount {
        fn package_name(&self) -> &'static str {
            "cosmos.vesting.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgCreatePermanentLockedAccount"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreatePermanentLockedAccount"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreatePermanentLockedAccount" , decoder : | buf : & [u8] | { let msg : MsgCreatePermanentLockedAccount = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_PERMANENT_LOCKED_ACCOUNT_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreatePermanentLockedAccountResponse"
    )]
    impl ::prost_wkt::MessageSerde for MsgCreatePermanentLockedAccountResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.vesting.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgCreatePermanentLockedAccountResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreatePermanentLockedAccountResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreatePermanentLockedAccountResponse" , decoder : | buf : & [u8] | { let msg : MsgCreatePermanentLockedAccountResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_PERIODIC_VESTING_ACCOUNT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreatePeriodicVestingAccount"
    )]
    impl ::prost_wkt::MessageSerde for MsgCreatePeriodicVestingAccount {
        fn package_name(&self) -> &'static str {
            "cosmos.vesting.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgCreatePeriodicVestingAccount"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreatePeriodicVestingAccount"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreatePeriodicVestingAccount" , decoder : | buf : & [u8] | { let msg : MsgCreatePeriodicVestingAccount = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_PERIODIC_VESTING_ACCOUNT_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreatePeriodicVestingAccountResponse"
    )]
    impl ::prost_wkt::MessageSerde for MsgCreatePeriodicVestingAccountResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.vesting.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgCreatePeriodicVestingAccountResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreatePeriodicVestingAccountResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.vesting.v1beta1.MsgCreatePeriodicVestingAccountResponse" , decoder : | buf : & [u8] | { let msg : MsgCreatePeriodicVestingAccountResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
