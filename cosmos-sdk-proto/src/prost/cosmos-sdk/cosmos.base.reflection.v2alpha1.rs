/// AppDescriptor describes a cosmos-sdk based application
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppDescriptor {
    /// AuthnDescriptor provides information on how to authenticate transactions on the application
    /// NOTE: experimental and subject to change in future releases.
    #[prost(message, optional, tag = "1")]
    pub authn: ::core::option::Option<AuthnDescriptor>,
    /// chain provides the chain descriptor
    #[prost(message, optional, tag = "2")]
    pub chain: ::core::option::Option<ChainDescriptor>,
    /// codec provides metadata information regarding codec related types
    #[prost(message, optional, tag = "3")]
    pub codec: ::core::option::Option<CodecDescriptor>,
    /// configuration provides metadata information regarding the sdk.Config type
    #[prost(message, optional, tag = "4")]
    pub configuration: ::core::option::Option<ConfigurationDescriptor>,
    /// query_services provides metadata information regarding the available queriable endpoints
    #[prost(message, optional, tag = "5")]
    pub query_services: ::core::option::Option<QueryServicesDescriptor>,
    /// tx provides metadata information regarding how to send transactions to the given application
    #[prost(message, optional, tag = "6")]
    pub tx: ::core::option::Option<TxDescriptor>,
}
/// TxDescriptor describes the accepted transaction type
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxDescriptor {
    /// fullname is the protobuf fullname of the raw transaction type (for instance the tx.Tx type)
    /// it is not meant to support polymorphism of transaction types, it is supposed to be used by
    /// reflection clients to understand if they can handle a specific transaction type in an application.
    #[prost(string, tag = "1")]
    pub fullname: ::prost::alloc::string::String,
    /// msgs lists the accepted application messages (sdk.Msg)
    #[prost(message, repeated, tag = "2")]
    pub msgs: ::prost::alloc::vec::Vec<MsgDescriptor>,
}
/// AuthnDescriptor provides information on how to sign transactions without relying
/// on the online RPCs GetTxMetadata and CombineUnsignedTxAndSignatures
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnDescriptor {
    /// sign_modes defines the supported signature algorithm
    #[prost(message, repeated, tag = "1")]
    pub sign_modes: ::prost::alloc::vec::Vec<SigningModeDescriptor>,
}
/// SigningModeDescriptor provides information on a signing flow of the application
/// NOTE(fdymylja): here we could go as far as providing an entire flow on how
/// to sign a message given a SigningModeDescriptor, but it's better to think about
/// this another time
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigningModeDescriptor {
    /// name defines the unique name of the signing mode
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// number is the unique int32 identifier for the sign_mode enum
    #[prost(int32, tag = "2")]
    pub number: i32,
    /// authn_info_provider_method_fullname defines the fullname of the method to call to get
    /// the metadata required to authenticate using the provided sign_modes
    #[prost(string, tag = "3")]
    pub authn_info_provider_method_fullname: ::prost::alloc::string::String,
}
/// ChainDescriptor describes chain information of the application
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainDescriptor {
    /// id is the chain id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// CodecDescriptor describes the registered interfaces and provides metadata information on the types
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodecDescriptor {
    /// interfaces is a list of the registerted interfaces descriptors
    #[prost(message, repeated, tag = "1")]
    pub interfaces: ::prost::alloc::vec::Vec<InterfaceDescriptor>,
}
/// InterfaceDescriptor describes the implementation of an interface
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceDescriptor {
    /// fullname is the name of the interface
    #[prost(string, tag = "1")]
    pub fullname: ::prost::alloc::string::String,
    /// interface_accepting_messages contains information regarding the proto messages which contain the interface as
    /// google.protobuf.Any field
    #[prost(message, repeated, tag = "2")]
    pub interface_accepting_messages: ::prost::alloc::vec::Vec<InterfaceAcceptingMessageDescriptor>,
    /// interface_implementers is a list of the descriptors of the interface implementers
    #[prost(message, repeated, tag = "3")]
    pub interface_implementers: ::prost::alloc::vec::Vec<InterfaceImplementerDescriptor>,
}
/// InterfaceImplementerDescriptor describes an interface implementer
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceImplementerDescriptor {
    /// fullname is the protobuf queryable name of the interface implementer
    #[prost(string, tag = "1")]
    pub fullname: ::prost::alloc::string::String,
    /// type_url defines the type URL used when marshalling the type as any
    /// this is required so we can provide type safe google.protobuf.Any marshalling and
    /// unmarshalling, making sure that we don't accept just 'any' type
    /// in our interface fields
    #[prost(string, tag = "2")]
    pub type_url: ::prost::alloc::string::String,
}
/// InterfaceAcceptingMessageDescriptor describes a protobuf message which contains
/// an interface represented as a google.protobuf.Any
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceAcceptingMessageDescriptor {
    /// fullname is the protobuf fullname of the type containing the interface
    #[prost(string, tag = "1")]
    pub fullname: ::prost::alloc::string::String,
    /// field_descriptor_names is a list of the protobuf name (not fullname) of the field
    /// which contains the interface as google.protobuf.Any (the interface is the same, but
    /// it can be in multiple fields of the same proto message)
    #[prost(string, repeated, tag = "2")]
    pub field_descriptor_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ConfigurationDescriptor contains metadata information on the sdk.Config
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigurationDescriptor {
    /// bech32_account_address_prefix is the account address prefix
    #[prost(string, tag = "1")]
    pub bech32_account_address_prefix: ::prost::alloc::string::String,
}
/// MsgDescriptor describes a cosmos-sdk message that can be delivered with a transaction
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDescriptor {
    /// msg_type_url contains the TypeURL of a sdk.Msg.
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
}
/// GetAuthnDescriptorRequest is the request used for the GetAuthnDescriptor RPC
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthnDescriptorRequest {}
/// GetAuthnDescriptorResponse is the response returned by the GetAuthnDescriptor RPC
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthnDescriptorResponse {
    /// authn describes how to authenticate to the application when sending transactions
    #[prost(message, optional, tag = "1")]
    pub authn: ::core::option::Option<AuthnDescriptor>,
}
/// GetChainDescriptorRequest is the request used for the GetChainDescriptor RPC
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChainDescriptorRequest {}
/// GetChainDescriptorResponse is the response returned by the GetChainDescriptor RPC
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChainDescriptorResponse {
    /// chain describes application chain information
    #[prost(message, optional, tag = "1")]
    pub chain: ::core::option::Option<ChainDescriptor>,
}
/// GetCodecDescriptorRequest is the request used for the GetCodecDescriptor RPC
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCodecDescriptorRequest {}
/// GetCodecDescriptorResponse is the response returned by the GetCodecDescriptor RPC
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCodecDescriptorResponse {
    /// codec describes the application codec such as registered interfaces and implementations
    #[prost(message, optional, tag = "1")]
    pub codec: ::core::option::Option<CodecDescriptor>,
}
/// GetConfigurationDescriptorRequest is the request used for the GetConfigurationDescriptor RPC
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigurationDescriptorRequest {}
/// GetConfigurationDescriptorResponse is the response returned by the GetConfigurationDescriptor RPC
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigurationDescriptorResponse {
    /// config describes the application's sdk.Config
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<ConfigurationDescriptor>,
}
/// GetQueryServicesDescriptorRequest is the request used for the GetQueryServicesDescriptor RPC
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQueryServicesDescriptorRequest {}
/// GetQueryServicesDescriptorResponse is the response returned by the GetQueryServicesDescriptor RPC
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQueryServicesDescriptorResponse {
    /// queries provides information on the available queryable services
    #[prost(message, optional, tag = "1")]
    pub queries: ::core::option::Option<QueryServicesDescriptor>,
}
/// GetTxDescriptorRequest is the request used for the GetTxDescriptor RPC
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxDescriptorRequest {}
/// GetTxDescriptorResponse is the response returned by the GetTxDescriptor RPC
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxDescriptorResponse {
    /// tx provides information on msgs that can be forwarded to the application
    /// alongside the accepted transaction protobuf type
    #[prost(message, optional, tag = "1")]
    pub tx: ::core::option::Option<TxDescriptor>,
}
/// QueryServicesDescriptor contains the list of cosmos-sdk queriable services
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryServicesDescriptor {
    /// query_services is a list of cosmos-sdk QueryServiceDescriptor
    #[prost(message, repeated, tag = "1")]
    pub query_services: ::prost::alloc::vec::Vec<QueryServiceDescriptor>,
}
/// QueryServiceDescriptor describes a cosmos-sdk queryable service
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryServiceDescriptor {
    /// fullname is the protobuf fullname of the service descriptor
    #[prost(string, tag = "1")]
    pub fullname: ::prost::alloc::string::String,
    /// is_module describes if this service is actually exposed by an application's module
    #[prost(bool, tag = "2")]
    pub is_module: bool,
    /// methods provides a list of query service methods
    #[prost(message, repeated, tag = "3")]
    pub methods: ::prost::alloc::vec::Vec<QueryMethodDescriptor>,
}
/// QueryMethodDescriptor describes a queryable method of a query service
/// no other info is provided beside method name and tendermint queryable path
/// because it would be redundant with the grpc reflection service
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMethodDescriptor {
    /// name is the protobuf name (not fullname) of the method
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// full_query_path is the path that can be used to query
    /// this method via tendermint abci.Query
    #[prost(string, tag = "2")]
    pub full_query_path: ::prost::alloc::string::String,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod reflection_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// ReflectionService defines a service for application reflection.
    #[derive(Debug, Clone)]
    pub struct ReflectionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl ReflectionServiceClient<tonic::transport::Channel> {
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
    impl<T> ReflectionServiceClient<T>
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
        ) -> ReflectionServiceClient<InterceptedService<T, F>>
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
            ReflectionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// GetAuthnDescriptor returns information on how to authenticate transactions in the application
        /// NOTE: this RPC is still experimental and might be subject to breaking changes or removal in
        /// future releases of the cosmos-sdk.
        pub async fn get_authn_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAuthnDescriptorRequest>,
        ) -> std::result::Result<tonic::Response<super::GetAuthnDescriptorResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetAuthnDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.reflection.v2alpha1.ReflectionService",
                "GetAuthnDescriptor",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// GetChainDescriptor returns the description of the chain
        pub async fn get_chain_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChainDescriptorRequest>,
        ) -> std::result::Result<tonic::Response<super::GetChainDescriptorResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetChainDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.reflection.v2alpha1.ReflectionService",
                "GetChainDescriptor",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// GetCodecDescriptor returns the descriptor of the codec of the application
        pub async fn get_codec_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCodecDescriptorRequest>,
        ) -> std::result::Result<tonic::Response<super::GetCodecDescriptorResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetCodecDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.reflection.v2alpha1.ReflectionService",
                "GetCodecDescriptor",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// GetConfigurationDescriptor returns the descriptor for the sdk.Config of the application
        pub async fn get_configuration_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConfigurationDescriptorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetConfigurationDescriptorResponse>,
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetConfigurationDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.reflection.v2alpha1.ReflectionService",
                "GetConfigurationDescriptor",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// GetQueryServicesDescriptor returns the available gRPC queryable services of the application
        pub async fn get_query_services_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQueryServicesDescriptorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetQueryServicesDescriptorResponse>,
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetQueryServicesDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.reflection.v2alpha1.ReflectionService",
                "GetQueryServicesDescriptor",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// GetTxDescriptor returns information on the used transaction object and available msgs that can be used
        pub async fn get_tx_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTxDescriptorRequest>,
        ) -> std::result::Result<tonic::Response<super::GetTxDescriptorResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetTxDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.reflection.v2alpha1.ReflectionService",
                "GetTxDescriptor",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod reflection_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ReflectionServiceServer.
    #[async_trait]
    pub trait ReflectionService: Send + Sync + 'static {
        /// GetAuthnDescriptor returns information on how to authenticate transactions in the application
        /// NOTE: this RPC is still experimental and might be subject to breaking changes or removal in
        /// future releases of the cosmos-sdk.
        async fn get_authn_descriptor(
            &self,
            request: tonic::Request<super::GetAuthnDescriptorRequest>,
        ) -> std::result::Result<tonic::Response<super::GetAuthnDescriptorResponse>, tonic::Status>;
        /// GetChainDescriptor returns the description of the chain
        async fn get_chain_descriptor(
            &self,
            request: tonic::Request<super::GetChainDescriptorRequest>,
        ) -> std::result::Result<tonic::Response<super::GetChainDescriptorResponse>, tonic::Status>;
        /// GetCodecDescriptor returns the descriptor of the codec of the application
        async fn get_codec_descriptor(
            &self,
            request: tonic::Request<super::GetCodecDescriptorRequest>,
        ) -> std::result::Result<tonic::Response<super::GetCodecDescriptorResponse>, tonic::Status>;
        /// GetConfigurationDescriptor returns the descriptor for the sdk.Config of the application
        async fn get_configuration_descriptor(
            &self,
            request: tonic::Request<super::GetConfigurationDescriptorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetConfigurationDescriptorResponse>,
            tonic::Status,
        >;
        /// GetQueryServicesDescriptor returns the available gRPC queryable services of the application
        async fn get_query_services_descriptor(
            &self,
            request: tonic::Request<super::GetQueryServicesDescriptorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetQueryServicesDescriptorResponse>,
            tonic::Status,
        >;
        /// GetTxDescriptor returns information on the used transaction object and available msgs that can be used
        async fn get_tx_descriptor(
            &self,
            request: tonic::Request<super::GetTxDescriptorRequest>,
        ) -> std::result::Result<tonic::Response<super::GetTxDescriptorResponse>, tonic::Status>;
    }
    /// ReflectionService defines a service for application reflection.
    #[derive(Debug)]
    pub struct ReflectionServiceServer<T: ReflectionService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ReflectionService> ReflectionServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ReflectionServiceServer<T>
    where
        T: ReflectionService,
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetAuthnDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetAuthnDescriptorSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::GetAuthnDescriptorRequest>
                        for GetAuthnDescriptorSvc<T>
                    {
                        type Response = super::GetAuthnDescriptorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAuthnDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_authn_descriptor(request).await };
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
                        let method = GetAuthnDescriptorSvc(inner);
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetChainDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetChainDescriptorSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::GetChainDescriptorRequest>
                        for GetChainDescriptorSvc<T>
                    {
                        type Response = super::GetChainDescriptorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetChainDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_chain_descriptor(request).await };
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
                        let method = GetChainDescriptorSvc(inner);
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetCodecDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetCodecDescriptorSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::GetCodecDescriptorRequest>
                        for GetCodecDescriptorSvc<T>
                    {
                        type Response = super::GetCodecDescriptorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCodecDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_codec_descriptor(request).await };
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
                        let method = GetCodecDescriptorSvc(inner);
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetConfigurationDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetConfigurationDescriptorSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::GetConfigurationDescriptorRequest>
                        for GetConfigurationDescriptorSvc<T>
                    {
                        type Response = super::GetConfigurationDescriptorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetConfigurationDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).get_configuration_descriptor(request).await };
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
                        let method = GetConfigurationDescriptorSvc(inner);
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetQueryServicesDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetQueryServicesDescriptorSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::GetQueryServicesDescriptorRequest>
                        for GetQueryServicesDescriptorSvc<T>
                    {
                        type Response = super::GetQueryServicesDescriptorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetQueryServicesDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_query_services_descriptor(request).await
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
                        let method = GetQueryServicesDescriptorSvc(inner);
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
                "/cosmos.base.reflection.v2alpha1.ReflectionService/GetTxDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetTxDescriptorSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::GetTxDescriptorRequest>
                        for GetTxDescriptorSvc<T>
                    {
                        type Response = super::GetTxDescriptorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTxDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_tx_descriptor(request).await };
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
                        let method = GetTxDescriptorSvc(inner);
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
    impl<T: ReflectionService> Clone for ReflectionServiceServer<T> {
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
    impl<T: ReflectionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ReflectionService> tonic::server::NamedService for ReflectionServiceServer<T> {
        const NAME: &'static str = "cosmos.base.reflection.v2alpha1.ReflectionService";
    }
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_APP_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.base.reflection.v2alpha1.AppDescriptor")]
    impl ::prost_wkt::MessageSerde for AppDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "AppDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.AppDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.AppDescriptor" , decoder : | buf : & [u8] | { let msg : AppDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_TX_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.base.reflection.v2alpha1.TxDescriptor")]
    impl ::prost_wkt::MessageSerde for TxDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "TxDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.TxDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.TxDescriptor" , decoder : | buf : & [u8] | { let msg : TxDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_AUTHN_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.base.reflection.v2alpha1.AuthnDescriptor")]
    impl ::prost_wkt::MessageSerde for AuthnDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "AuthnDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.AuthnDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.AuthnDescriptor" , decoder : | buf : & [u8] | { let msg : AuthnDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_SIGNING_MODE_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.SigningModeDescriptor"
    )]
    impl ::prost_wkt::MessageSerde for SigningModeDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "SigningModeDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.SigningModeDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.SigningModeDescriptor" , decoder : | buf : & [u8] | { let msg : SigningModeDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_CHAIN_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.base.reflection.v2alpha1.ChainDescriptor")]
    impl ::prost_wkt::MessageSerde for ChainDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "ChainDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.ChainDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.ChainDescriptor" , decoder : | buf : & [u8] | { let msg : ChainDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_CODEC_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.base.reflection.v2alpha1.CodecDescriptor")]
    impl ::prost_wkt::MessageSerde for CodecDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "CodecDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.CodecDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.CodecDescriptor" , decoder : | buf : & [u8] | { let msg : CodecDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_INTERFACE_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.InterfaceDescriptor"
    )]
    impl ::prost_wkt::MessageSerde for InterfaceDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "InterfaceDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.InterfaceDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.InterfaceDescriptor" , decoder : | buf : & [u8] | { let msg : InterfaceDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_INTERFACE_IMPLEMENTER_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.InterfaceImplementerDescriptor"
    )]
    impl ::prost_wkt::MessageSerde for InterfaceImplementerDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "InterfaceImplementerDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.InterfaceImplementerDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.InterfaceImplementerDescriptor" , decoder : | buf : & [u8] | { let msg : InterfaceImplementerDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_INTERFACE_ACCEPTING_MESSAGE_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.InterfaceAcceptingMessageDescriptor"
    )]
    impl ::prost_wkt::MessageSerde for InterfaceAcceptingMessageDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "InterfaceAcceptingMessageDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.InterfaceAcceptingMessageDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.InterfaceAcceptingMessageDescriptor" , decoder : | buf : & [u8] | { let msg : InterfaceAcceptingMessageDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_CONFIGURATION_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.ConfigurationDescriptor"
    )]
    impl ::prost_wkt::MessageSerde for ConfigurationDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "ConfigurationDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.ConfigurationDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.ConfigurationDescriptor" , decoder : | buf : & [u8] | { let msg : ConfigurationDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.base.reflection.v2alpha1.MsgDescriptor")]
    impl ::prost_wkt::MessageSerde for MsgDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "MsgDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.MsgDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.MsgDescriptor" , decoder : | buf : & [u8] | { let msg : MsgDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_AUTHN_DESCRIPTOR_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.GetAuthnDescriptorRequest"
    )]
    impl ::prost_wkt::MessageSerde for GetAuthnDescriptorRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "GetAuthnDescriptorRequest"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.GetAuthnDescriptorRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.GetAuthnDescriptorRequest" , decoder : | buf : & [u8] | { let msg : GetAuthnDescriptorRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_AUTHN_DESCRIPTOR_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.GetAuthnDescriptorResponse"
    )]
    impl ::prost_wkt::MessageSerde for GetAuthnDescriptorResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "GetAuthnDescriptorResponse"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.GetAuthnDescriptorResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.GetAuthnDescriptorResponse" , decoder : | buf : & [u8] | { let msg : GetAuthnDescriptorResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_CHAIN_DESCRIPTOR_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.GetChainDescriptorRequest"
    )]
    impl ::prost_wkt::MessageSerde for GetChainDescriptorRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "GetChainDescriptorRequest"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.GetChainDescriptorRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.GetChainDescriptorRequest" , decoder : | buf : & [u8] | { let msg : GetChainDescriptorRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_CHAIN_DESCRIPTOR_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.GetChainDescriptorResponse"
    )]
    impl ::prost_wkt::MessageSerde for GetChainDescriptorResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "GetChainDescriptorResponse"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.GetChainDescriptorResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.GetChainDescriptorResponse" , decoder : | buf : & [u8] | { let msg : GetChainDescriptorResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_CODEC_DESCRIPTOR_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.GetCodecDescriptorRequest"
    )]
    impl ::prost_wkt::MessageSerde for GetCodecDescriptorRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "GetCodecDescriptorRequest"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.GetCodecDescriptorRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.GetCodecDescriptorRequest" , decoder : | buf : & [u8] | { let msg : GetCodecDescriptorRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_CODEC_DESCRIPTOR_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.GetCodecDescriptorResponse"
    )]
    impl ::prost_wkt::MessageSerde for GetCodecDescriptorResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "GetCodecDescriptorResponse"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.GetCodecDescriptorResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.GetCodecDescriptorResponse" , decoder : | buf : & [u8] | { let msg : GetCodecDescriptorResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_CONFIGURATION_DESCRIPTOR_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.GetConfigurationDescriptorRequest"
    )]
    impl ::prost_wkt::MessageSerde for GetConfigurationDescriptorRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "GetConfigurationDescriptorRequest"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.GetConfigurationDescriptorRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.GetConfigurationDescriptorRequest" , decoder : | buf : & [u8] | { let msg : GetConfigurationDescriptorRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_CONFIGURATION_DESCRIPTOR_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.GetConfigurationDescriptorResponse"
    )]
    impl ::prost_wkt::MessageSerde for GetConfigurationDescriptorResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "GetConfigurationDescriptorResponse"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.GetConfigurationDescriptorResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.GetConfigurationDescriptorResponse" , decoder : | buf : & [u8] | { let msg : GetConfigurationDescriptorResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_QUERY_SERVICES_DESCRIPTOR_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.GetQueryServicesDescriptorRequest"
    )]
    impl ::prost_wkt::MessageSerde for GetQueryServicesDescriptorRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "GetQueryServicesDescriptorRequest"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.GetQueryServicesDescriptorRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.GetQueryServicesDescriptorRequest" , decoder : | buf : & [u8] | { let msg : GetQueryServicesDescriptorRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_QUERY_SERVICES_DESCRIPTOR_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.GetQueryServicesDescriptorResponse"
    )]
    impl ::prost_wkt::MessageSerde for GetQueryServicesDescriptorResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "GetQueryServicesDescriptorResponse"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.GetQueryServicesDescriptorResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.GetQueryServicesDescriptorResponse" , decoder : | buf : & [u8] | { let msg : GetQueryServicesDescriptorResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_TX_DESCRIPTOR_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.GetTxDescriptorRequest"
    )]
    impl ::prost_wkt::MessageSerde for GetTxDescriptorRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "GetTxDescriptorRequest"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.GetTxDescriptorRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.GetTxDescriptorRequest" , decoder : | buf : & [u8] | { let msg : GetTxDescriptorRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GET_TX_DESCRIPTOR_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.GetTxDescriptorResponse"
    )]
    impl ::prost_wkt::MessageSerde for GetTxDescriptorResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "GetTxDescriptorResponse"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.GetTxDescriptorResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.GetTxDescriptorResponse" , decoder : | buf : & [u8] | { let msg : GetTxDescriptorResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_SERVICES_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.QueryServicesDescriptor"
    )]
    impl ::prost_wkt::MessageSerde for QueryServicesDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "QueryServicesDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.QueryServicesDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.QueryServicesDescriptor" , decoder : | buf : & [u8] | { let msg : QueryServicesDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_SERVICE_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.QueryServiceDescriptor"
    )]
    impl ::prost_wkt::MessageSerde for QueryServiceDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "QueryServiceDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.QueryServiceDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.QueryServiceDescriptor" , decoder : | buf : & [u8] | { let msg : QueryServiceDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_METHOD_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.base.reflection.v2alpha1.QueryMethodDescriptor"
    )]
    impl ::prost_wkt::MessageSerde for QueryMethodDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos.base.reflection.v2alpha1"
        }
        fn message_name(&self) -> &'static str {
            "QueryMethodDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.reflection.v2alpha1.QueryMethodDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.reflection.v2alpha1.QueryMethodDescriptor" , decoder : | buf : & [u8] | { let msg : QueryMethodDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
