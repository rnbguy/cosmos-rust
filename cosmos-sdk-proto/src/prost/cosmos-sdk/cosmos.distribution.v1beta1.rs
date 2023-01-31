/// Params defines the set of params for the distribution module.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag = "1")]
    pub community_tax: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub base_proposer_reward: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub bonus_proposer_reward: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub withdraw_addr_enabled: bool,
}
/// ValidatorHistoricalRewards represents historical rewards for a validator.
/// Height is implicit within the store key.
/// Cumulative reward ratio is the sum from the zeroeth period
/// until this period of rewards / tokens, per the spec.
/// The reference count indicates the number of objects
/// which might need to reference this historical entry at any point.
/// ReferenceCount =
///     number of outstanding delegations which ended the associated period (and
///     might need to read that record)
///   + number of slashes which ended the associated period (and might need to
///   read that record)
///   + one per validator for the zeroeth period, set on initialization
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorHistoricalRewards {
    #[prost(message, repeated, tag = "1")]
    pub cumulative_reward_ratio: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
    #[prost(uint32, tag = "2")]
    pub reference_count: u32,
}
/// ValidatorCurrentRewards represents current rewards and current
/// period for a validator kept as a running counter and incremented
/// each block as long as the validator's tokens remain constant.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorCurrentRewards {
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
    #[prost(uint64, tag = "2")]
    pub period: u64,
}
/// ValidatorAccumulatedCommission represents accumulated commission
/// for a validator kept as a running counter, can be withdrawn at any time.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorAccumulatedCommission {
    #[prost(message, repeated, tag = "1")]
    pub commission: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// ValidatorOutstandingRewards represents outstanding (un-withdrawn) rewards
/// for a validator inexpensive to track, allows simple sanity checks.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorOutstandingRewards {
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// ValidatorSlashEvent represents a validator slash event.
/// Height is implicit within the store key.
/// This is needed to calculate appropriate amount of staking tokens
/// for delegations which are withdrawn after a slash has occurred.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEvent {
    #[prost(uint64, tag = "1")]
    pub validator_period: u64,
    #[prost(string, tag = "2")]
    pub fraction: ::prost::alloc::string::String,
}
/// ValidatorSlashEvents is a collection of ValidatorSlashEvent messages.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEvents {
    #[prost(message, repeated, tag = "1")]
    pub validator_slash_events: ::prost::alloc::vec::Vec<ValidatorSlashEvent>,
}
/// FeePool is the global fee pool for distribution.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeePool {
    #[prost(message, repeated, tag = "1")]
    pub community_pool: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// CommunityPoolSpendProposal details a proposal for use of community funds,
/// together with how many coins are proposed to be spent, and to which
/// recipient account.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommunityPoolSpendProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// DelegatorStartingInfo represents the starting info for a delegator reward
/// period. It tracks the previous validator period, the delegation's amount of
/// staking token, and the creation height (to check later on if any slashes have
/// occurred). NOTE: Even though validators are slashed to whole staking tokens,
/// the delegators within the validator may be left with less than a full token,
/// thus sdk.Dec is used.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorStartingInfo {
    #[prost(uint64, tag = "1")]
    pub previous_period: u64,
    #[prost(string, tag = "2")]
    pub stake: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub height: u64,
}
/// DelegationDelegatorReward represents the properties
/// of a delegator's delegation reward.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationDelegatorReward {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub reward: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// CommunityPoolSpendProposalWithDeposit defines a CommunityPoolSpendProposal
/// with a deposit
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommunityPoolSpendProposalWithDeposit {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub deposit: ::prost::alloc::string::String,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryValidatorOutstandingRewardsRequest is the request type for the
/// Query/ValidatorOutstandingRewards RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorOutstandingRewardsRequest {
    /// validator_address defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
}
/// QueryValidatorOutstandingRewardsResponse is the response type for the
/// Query/ValidatorOutstandingRewards RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorOutstandingRewardsResponse {
    #[prost(message, optional, tag = "1")]
    pub rewards: ::core::option::Option<ValidatorOutstandingRewards>,
}
/// QueryValidatorCommissionRequest is the request type for the
/// Query/ValidatorCommission RPC method
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorCommissionRequest {
    /// validator_address defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
}
/// QueryValidatorCommissionResponse is the response type for the
/// Query/ValidatorCommission RPC method
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorCommissionResponse {
    /// commission defines the commision the validator received.
    #[prost(message, optional, tag = "1")]
    pub commission: ::core::option::Option<ValidatorAccumulatedCommission>,
}
/// QueryValidatorSlashesRequest is the request type for the
/// Query/ValidatorSlashes RPC method
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSlashesRequest {
    /// validator_address defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// starting_height defines the optional starting height to query the slashes.
    #[prost(uint64, tag = "2")]
    pub starting_height: u64,
    /// starting_height defines the optional ending height to query the slashes.
    #[prost(uint64, tag = "3")]
    pub ending_height: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorSlashesResponse is the response type for the
/// Query/ValidatorSlashes RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSlashesResponse {
    /// slashes defines the slashes the validator received.
    #[prost(message, repeated, tag = "1")]
    pub slashes: ::prost::alloc::vec::Vec<ValidatorSlashEvent>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegationRewardsRequest is the request type for the
/// Query/DelegationRewards RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationRewardsRequest {
    /// delegator_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_address defines the validator address to query for.
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
/// QueryDelegationRewardsResponse is the response type for the
/// Query/DelegationRewards RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationRewardsResponse {
    /// rewards defines the rewards accrued by a delegation.
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// QueryDelegationTotalRewardsRequest is the request type for the
/// Query/DelegationTotalRewards RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationTotalRewardsRequest {
    /// delegator_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
/// QueryDelegationTotalRewardsResponse is the response type for the
/// Query/DelegationTotalRewards RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationTotalRewardsResponse {
    /// rewards defines all the rewards accrued by a delegator.
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::prost::alloc::vec::Vec<DelegationDelegatorReward>,
    /// total defines the sum of all the rewards.
    #[prost(message, repeated, tag = "2")]
    pub total: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// QueryDelegatorValidatorsRequest is the request type for the
/// Query/DelegatorValidators RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorsRequest {
    /// delegator_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
/// QueryDelegatorValidatorsResponse is the response type for the
/// Query/DelegatorValidators RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorsResponse {
    /// validators defines the validators a delegator is delegating for.
    #[prost(string, repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryDelegatorWithdrawAddressRequest is the request type for the
/// Query/DelegatorWithdrawAddress RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorWithdrawAddressRequest {
    /// delegator_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
/// QueryDelegatorWithdrawAddressResponse is the response type for the
/// Query/DelegatorWithdrawAddress RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorWithdrawAddressResponse {
    /// withdraw_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub withdraw_address: ::prost::alloc::string::String,
}
/// QueryCommunityPoolRequest is the request type for the Query/CommunityPool RPC
/// method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCommunityPoolRequest {}
/// QueryCommunityPoolResponse is the response type for the Query/CommunityPool
/// RPC method.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCommunityPoolResponse {
    /// pool defines community pool's coins.
    #[prost(message, repeated, tag = "1")]
    pub pool: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC querier service for distribution module.
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
        /// Params queries params of the distribution module.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.distribution.v1beta1.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "Params",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ValidatorOutstandingRewards queries rewards of a validator address.
        pub async fn validator_outstanding_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorOutstandingRewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidatorOutstandingRewardsResponse>,
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
                "/cosmos.distribution.v1beta1.Query/ValidatorOutstandingRewards",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "ValidatorOutstandingRewards",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ValidatorCommission queries accumulated commission for a validator.
        pub async fn validator_commission(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorCommissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidatorCommissionResponse>,
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
                "/cosmos.distribution.v1beta1.Query/ValidatorCommission",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "ValidatorCommission",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ValidatorSlashes queries slash events of a validator.
        pub async fn validator_slashes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorSlashesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryValidatorSlashesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Query/ValidatorSlashes",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "ValidatorSlashes",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// DelegationRewards queries the total rewards accrued by a delegation.
        pub async fn delegation_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegationRewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDelegationRewardsResponse>,
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
                "/cosmos.distribution.v1beta1.Query/DelegationRewards",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "DelegationRewards",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// DelegationTotalRewards queries the total rewards accrued by a each
        /// validator.
        pub async fn delegation_total_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegationTotalRewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDelegationTotalRewardsResponse>,
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
                "/cosmos.distribution.v1beta1.Query/DelegationTotalRewards",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "DelegationTotalRewards",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// DelegatorValidators queries the validators of a delegator.
        pub async fn delegator_validators(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegatorValidatorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDelegatorValidatorsResponse>,
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
                "/cosmos.distribution.v1beta1.Query/DelegatorValidators",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "DelegatorValidators",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// DelegatorWithdrawAddress queries withdraw address of a delegator.
        pub async fn delegator_withdraw_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegatorWithdrawAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDelegatorWithdrawAddressResponse>,
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
                "/cosmos.distribution.v1beta1.Query/DelegatorWithdrawAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "DelegatorWithdrawAddress",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// CommunityPool queries the community pool coins.
        pub async fn community_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCommunityPoolRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryCommunityPoolResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Query/CommunityPool",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "CommunityPool",
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
        /// Params queries params of the distribution module.
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        /// ValidatorOutstandingRewards queries rewards of a validator address.
        async fn validator_outstanding_rewards(
            &self,
            request: tonic::Request<super::QueryValidatorOutstandingRewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidatorOutstandingRewardsResponse>,
            tonic::Status,
        >;
        /// ValidatorCommission queries accumulated commission for a validator.
        async fn validator_commission(
            &self,
            request: tonic::Request<super::QueryValidatorCommissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidatorCommissionResponse>,
            tonic::Status,
        >;
        /// ValidatorSlashes queries slash events of a validator.
        async fn validator_slashes(
            &self,
            request: tonic::Request<super::QueryValidatorSlashesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryValidatorSlashesResponse>, tonic::Status>;
        /// DelegationRewards queries the total rewards accrued by a delegation.
        async fn delegation_rewards(
            &self,
            request: tonic::Request<super::QueryDelegationRewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDelegationRewardsResponse>,
            tonic::Status,
        >;
        /// DelegationTotalRewards queries the total rewards accrued by a each
        /// validator.
        async fn delegation_total_rewards(
            &self,
            request: tonic::Request<super::QueryDelegationTotalRewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDelegationTotalRewardsResponse>,
            tonic::Status,
        >;
        /// DelegatorValidators queries the validators of a delegator.
        async fn delegator_validators(
            &self,
            request: tonic::Request<super::QueryDelegatorValidatorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDelegatorValidatorsResponse>,
            tonic::Status,
        >;
        /// DelegatorWithdrawAddress queries withdraw address of a delegator.
        async fn delegator_withdraw_address(
            &self,
            request: tonic::Request<super::QueryDelegatorWithdrawAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDelegatorWithdrawAddressResponse>,
            tonic::Status,
        >;
        /// CommunityPool queries the community pool coins.
        async fn community_pool(
            &self,
            request: tonic::Request<super::QueryCommunityPoolRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryCommunityPoolResponse>, tonic::Status>;
    }
    /// Query defines the gRPC querier service for distribution module.
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
                "/cosmos.distribution.v1beta1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest> for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
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
                "/cosmos.distribution.v1beta1.Query/ValidatorOutstandingRewards" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorOutstandingRewardsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryValidatorOutstandingRewardsRequest>
                        for ValidatorOutstandingRewardsSvc<T>
                    {
                        type Response = super::QueryValidatorOutstandingRewardsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryValidatorOutstandingRewardsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).validator_outstanding_rewards(request).await
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
                        let method = ValidatorOutstandingRewardsSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/ValidatorCommission" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorCommissionSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryValidatorCommissionRequest>
                        for ValidatorCommissionSvc<T>
                    {
                        type Response = super::QueryValidatorCommissionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryValidatorCommissionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).validator_commission(request).await };
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
                        let method = ValidatorCommissionSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/ValidatorSlashes" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorSlashesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryValidatorSlashesRequest>
                        for ValidatorSlashesSvc<T>
                    {
                        type Response = super::QueryValidatorSlashesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryValidatorSlashesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).validator_slashes(request).await };
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
                        let method = ValidatorSlashesSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/DelegationRewards" => {
                    #[allow(non_camel_case_types)]
                    struct DelegationRewardsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDelegationRewardsRequest>
                        for DelegationRewardsSvc<T>
                    {
                        type Response = super::QueryDelegationRewardsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDelegationRewardsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delegation_rewards(request).await };
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
                        let method = DelegationRewardsSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/DelegationTotalRewards" => {
                    #[allow(non_camel_case_types)]
                    struct DelegationTotalRewardsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryDelegationTotalRewardsRequest>
                        for DelegationTotalRewardsSvc<T>
                    {
                        type Response = super::QueryDelegationTotalRewardsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDelegationTotalRewardsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).delegation_total_rewards(request).await };
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
                        let method = DelegationTotalRewardsSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/DelegatorValidators" => {
                    #[allow(non_camel_case_types)]
                    struct DelegatorValidatorsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryDelegatorValidatorsRequest>
                        for DelegatorValidatorsSvc<T>
                    {
                        type Response = super::QueryDelegatorValidatorsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDelegatorValidatorsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delegator_validators(request).await };
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
                        let method = DelegatorValidatorsSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/DelegatorWithdrawAddress" => {
                    #[allow(non_camel_case_types)]
                    struct DelegatorWithdrawAddressSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryDelegatorWithdrawAddressRequest>
                        for DelegatorWithdrawAddressSvc<T>
                    {
                        type Response = super::QueryDelegatorWithdrawAddressResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDelegatorWithdrawAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).delegator_withdraw_address(request).await };
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
                        let method = DelegatorWithdrawAddressSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/CommunityPool" => {
                    #[allow(non_camel_case_types)]
                    struct CommunityPoolSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryCommunityPoolRequest>
                        for CommunityPoolSvc<T>
                    {
                        type Response = super::QueryCommunityPoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryCommunityPoolRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).community_pool(request).await };
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
                        let method = CommunityPoolSvc(inner);
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
        const NAME: &'static str = "cosmos.distribution.v1beta1.Query";
    }
}
/// MsgSetWithdrawAddress sets the withdraw address for
/// a delegator (or validator self-delegation).
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetWithdrawAddress {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub withdraw_address: ::prost::alloc::string::String,
}
/// MsgSetWithdrawAddressResponse defines the Msg/SetWithdrawAddress response type.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetWithdrawAddressResponse {}
/// MsgWithdrawDelegatorReward represents delegation withdrawal to a delegator
/// from a single validator.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawDelegatorReward {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
/// MsgWithdrawDelegatorRewardResponse defines the Msg/WithdrawDelegatorReward response type.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawDelegatorRewardResponse {
    /// Since: cosmos-sdk 0.46
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgWithdrawValidatorCommission withdraws the full commission to the validator
/// address.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawValidatorCommission {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
}
/// MsgWithdrawValidatorCommissionResponse defines the Msg/WithdrawValidatorCommission response type.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawValidatorCommissionResponse {
    /// Since: cosmos-sdk 0.46
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgFundCommunityPool allows an account to directly
/// fund the community pool.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundCommunityPool {
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(string, tag = "2")]
    pub depositor: ::prost::alloc::string::String,
}
/// MsgFundCommunityPoolResponse defines the Msg/FundCommunityPool response type.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundCommunityPoolResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the distribution Msg service.
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
        /// SetWithdrawAddress defines a method to change the withdraw address
        /// for a delegator (or validator self-delegation).
        pub async fn set_withdraw_address(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetWithdrawAddress>,
        ) -> std::result::Result<tonic::Response<super::MsgSetWithdrawAddressResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Msg/SetWithdrawAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Msg",
                "SetWithdrawAddress",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// WithdrawDelegatorReward defines a method to withdraw rewards of delegator
        /// from a single validator.
        pub async fn withdraw_delegator_reward(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawDelegatorReward>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWithdrawDelegatorRewardResponse>,
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
                "/cosmos.distribution.v1beta1.Msg/WithdrawDelegatorReward",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Msg",
                "WithdrawDelegatorReward",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// WithdrawValidatorCommission defines a method to withdraw the
        /// full commission to the validator address.
        pub async fn withdraw_validator_commission(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawValidatorCommission>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWithdrawValidatorCommissionResponse>,
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
                "/cosmos.distribution.v1beta1.Msg/WithdrawValidatorCommission",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Msg",
                "WithdrawValidatorCommission",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// FundCommunityPool defines a method to allow an account to directly
        /// fund the community pool.
        pub async fn fund_community_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgFundCommunityPool>,
        ) -> std::result::Result<tonic::Response<super::MsgFundCommunityPoolResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Msg/FundCommunityPool",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Msg",
                "FundCommunityPool",
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
        /// SetWithdrawAddress defines a method to change the withdraw address
        /// for a delegator (or validator self-delegation).
        async fn set_withdraw_address(
            &self,
            request: tonic::Request<super::MsgSetWithdrawAddress>,
        ) -> std::result::Result<tonic::Response<super::MsgSetWithdrawAddressResponse>, tonic::Status>;
        /// WithdrawDelegatorReward defines a method to withdraw rewards of delegator
        /// from a single validator.
        async fn withdraw_delegator_reward(
            &self,
            request: tonic::Request<super::MsgWithdrawDelegatorReward>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWithdrawDelegatorRewardResponse>,
            tonic::Status,
        >;
        /// WithdrawValidatorCommission defines a method to withdraw the
        /// full commission to the validator address.
        async fn withdraw_validator_commission(
            &self,
            request: tonic::Request<super::MsgWithdrawValidatorCommission>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWithdrawValidatorCommissionResponse>,
            tonic::Status,
        >;
        /// FundCommunityPool defines a method to allow an account to directly
        /// fund the community pool.
        async fn fund_community_pool(
            &self,
            request: tonic::Request<super::MsgFundCommunityPool>,
        ) -> std::result::Result<tonic::Response<super::MsgFundCommunityPoolResponse>, tonic::Status>;
    }
    /// Msg defines the distribution Msg service.
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
                "/cosmos.distribution.v1beta1.Msg/SetWithdrawAddress" => {
                    #[allow(non_camel_case_types)]
                    struct SetWithdrawAddressSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSetWithdrawAddress>
                        for SetWithdrawAddressSvc<T>
                    {
                        type Response = super::MsgSetWithdrawAddressResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetWithdrawAddress>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).set_withdraw_address(request).await };
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
                        let method = SetWithdrawAddressSvc(inner);
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
                "/cosmos.distribution.v1beta1.Msg/WithdrawDelegatorReward" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawDelegatorRewardSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdrawDelegatorReward>
                        for WithdrawDelegatorRewardSvc<T>
                    {
                        type Response = super::MsgWithdrawDelegatorRewardResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdrawDelegatorReward>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).withdraw_delegator_reward(request).await };
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
                        let method = WithdrawDelegatorRewardSvc(inner);
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
                "/cosmos.distribution.v1beta1.Msg/WithdrawValidatorCommission" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawValidatorCommissionSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdrawValidatorCommission>
                        for WithdrawValidatorCommissionSvc<T>
                    {
                        type Response = super::MsgWithdrawValidatorCommissionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdrawValidatorCommission>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).withdraw_validator_commission(request).await
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
                        let method = WithdrawValidatorCommissionSvc(inner);
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
                "/cosmos.distribution.v1beta1.Msg/FundCommunityPool" => {
                    #[allow(non_camel_case_types)]
                    struct FundCommunityPoolSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgFundCommunityPool> for FundCommunityPoolSvc<T> {
                        type Response = super::MsgFundCommunityPoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgFundCommunityPool>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).fund_community_pool(request).await };
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
                        let method = FundCommunityPoolSvc(inner);
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
        const NAME: &'static str = "cosmos.distribution.v1beta1.Msg";
    }
}
/// DelegatorWithdrawInfo is the address for where distributions rewards are
/// withdrawn to by default this struct is only used at genesis to feed in
/// default withdraw addresses.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorWithdrawInfo {
    /// delegator_address is the address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// withdraw_address is the address to withdraw the delegation rewards to.
    #[prost(string, tag = "2")]
    pub withdraw_address: ::prost::alloc::string::String,
}
/// ValidatorOutstandingRewardsRecord is used for import/export via genesis json.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorOutstandingRewardsRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// outstanding_rewards represents the oustanding rewards of a validator.
    #[prost(message, repeated, tag = "2")]
    pub outstanding_rewards: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// ValidatorAccumulatedCommissionRecord is used for import / export via genesis
/// json.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorAccumulatedCommissionRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// accumulated is the accumulated commission of a validator.
    #[prost(message, optional, tag = "2")]
    pub accumulated: ::core::option::Option<ValidatorAccumulatedCommission>,
}
/// ValidatorHistoricalRewardsRecord is used for import / export via genesis
/// json.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorHistoricalRewardsRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// period defines the period the historical rewards apply to.
    #[prost(uint64, tag = "2")]
    pub period: u64,
    /// rewards defines the historical rewards of a validator.
    #[prost(message, optional, tag = "3")]
    pub rewards: ::core::option::Option<ValidatorHistoricalRewards>,
}
/// ValidatorCurrentRewardsRecord is used for import / export via genesis json.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorCurrentRewardsRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// rewards defines the current rewards of a validator.
    #[prost(message, optional, tag = "2")]
    pub rewards: ::core::option::Option<ValidatorCurrentRewards>,
}
/// DelegatorStartingInfoRecord used for import / export via genesis json.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorStartingInfoRecord {
    /// delegator_address is the address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_address is the address of the validator.
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// starting_info defines the starting info of a delegator.
    #[prost(message, optional, tag = "3")]
    pub starting_info: ::core::option::Option<DelegatorStartingInfo>,
}
/// ValidatorSlashEventRecord is used for import / export via genesis json.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEventRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// height defines the block height at which the slash event occured.
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// period is the period of the slash event.
    #[prost(uint64, tag = "3")]
    pub period: u64,
    /// validator_slash_event describes the slash event.
    #[prost(message, optional, tag = "4")]
    pub validator_slash_event: ::core::option::Option<ValidatorSlashEvent>,
}
/// GenesisState defines the distribution module's genesis state.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// fee_pool defines the fee pool at genesis.
    #[prost(message, optional, tag = "2")]
    pub fee_pool: ::core::option::Option<FeePool>,
    /// fee_pool defines the delegator withdraw infos at genesis.
    #[prost(message, repeated, tag = "3")]
    pub delegator_withdraw_infos: ::prost::alloc::vec::Vec<DelegatorWithdrawInfo>,
    /// fee_pool defines the previous proposer at genesis.
    #[prost(string, tag = "4")]
    pub previous_proposer: ::prost::alloc::string::String,
    /// fee_pool defines the outstanding rewards of all validators at genesis.
    #[prost(message, repeated, tag = "5")]
    pub outstanding_rewards: ::prost::alloc::vec::Vec<ValidatorOutstandingRewardsRecord>,
    /// fee_pool defines the accumulated commisions of all validators at genesis.
    #[prost(message, repeated, tag = "6")]
    pub validator_accumulated_commissions:
        ::prost::alloc::vec::Vec<ValidatorAccumulatedCommissionRecord>,
    /// fee_pool defines the historical rewards of all validators at genesis.
    #[prost(message, repeated, tag = "7")]
    pub validator_historical_rewards: ::prost::alloc::vec::Vec<ValidatorHistoricalRewardsRecord>,
    /// fee_pool defines the current rewards of all validators at genesis.
    #[prost(message, repeated, tag = "8")]
    pub validator_current_rewards: ::prost::alloc::vec::Vec<ValidatorCurrentRewardsRecord>,
    /// fee_pool defines the delegator starting infos at genesis.
    #[prost(message, repeated, tag = "9")]
    pub delegator_starting_infos: ::prost::alloc::vec::Vec<DelegatorStartingInfoRecord>,
    /// fee_pool defines the validator slash events at genesis.
    #[prost(message, repeated, tag = "10")]
    pub validator_slash_events: ::prost::alloc::vec::Vec<ValidatorSlashEventRecord>,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_PARAMS: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.distribution.v1beta1.Params")]
    impl ::prost_wkt::MessageSerde for Params {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "Params"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.Params"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.Params" , decoder : | buf : & [u8] | { let msg : Params = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_VALIDATOR_HISTORICAL_REWARDS: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorHistoricalRewards"
    )]
    impl ::prost_wkt::MessageSerde for ValidatorHistoricalRewards {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "ValidatorHistoricalRewards"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorHistoricalRewards"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorHistoricalRewards" , decoder : | buf : & [u8] | { let msg : ValidatorHistoricalRewards = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_VALIDATOR_CURRENT_REWARDS: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorCurrentRewards"
    )]
    impl ::prost_wkt::MessageSerde for ValidatorCurrentRewards {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "ValidatorCurrentRewards"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorCurrentRewards"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorCurrentRewards" , decoder : | buf : & [u8] | { let msg : ValidatorCurrentRewards = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_VALIDATOR_ACCUMULATED_COMMISSION: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorAccumulatedCommission"
    )]
    impl ::prost_wkt::MessageSerde for ValidatorAccumulatedCommission {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "ValidatorAccumulatedCommission"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorAccumulatedCommission"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorAccumulatedCommission" , decoder : | buf : & [u8] | { let msg : ValidatorAccumulatedCommission = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_VALIDATOR_OUTSTANDING_REWARDS: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorOutstandingRewards"
    )]
    impl ::prost_wkt::MessageSerde for ValidatorOutstandingRewards {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "ValidatorOutstandingRewards"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorOutstandingRewards"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorOutstandingRewards" , decoder : | buf : & [u8] | { let msg : ValidatorOutstandingRewards = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_VALIDATOR_SLASH_EVENT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorSlashEvent")]
    impl ::prost_wkt::MessageSerde for ValidatorSlashEvent {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "ValidatorSlashEvent"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorSlashEvent"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorSlashEvent" , decoder : | buf : & [u8] | { let msg : ValidatorSlashEvent = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_VALIDATOR_SLASH_EVENTS: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorSlashEvents")]
    impl ::prost_wkt::MessageSerde for ValidatorSlashEvents {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "ValidatorSlashEvents"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorSlashEvents"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorSlashEvents" , decoder : | buf : & [u8] | { let msg : ValidatorSlashEvents = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_FEE_POOL: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.distribution.v1beta1.FeePool")]
    impl ::prost_wkt::MessageSerde for FeePool {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "FeePool"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.FeePool"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.FeePool" , decoder : | buf : & [u8] | { let msg : FeePool = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_COMMUNITY_POOL_SPEND_PROPOSAL: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.CommunityPoolSpendProposal"
    )]
    impl ::prost_wkt::MessageSerde for CommunityPoolSpendProposal {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "CommunityPoolSpendProposal"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.CommunityPoolSpendProposal"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.CommunityPoolSpendProposal" , decoder : | buf : & [u8] | { let msg : CommunityPoolSpendProposal = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_DELEGATOR_STARTING_INFO: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.DelegatorStartingInfo"
    )]
    impl ::prost_wkt::MessageSerde for DelegatorStartingInfo {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "DelegatorStartingInfo"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.DelegatorStartingInfo"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.DelegatorStartingInfo" , decoder : | buf : & [u8] | { let msg : DelegatorStartingInfo = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_DELEGATION_DELEGATOR_REWARD: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.DelegationDelegatorReward"
    )]
    impl ::prost_wkt::MessageSerde for DelegationDelegatorReward {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "DelegationDelegatorReward"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.DelegationDelegatorReward"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.DelegationDelegatorReward" , decoder : | buf : & [u8] | { let msg : DelegationDelegatorReward = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_COMMUNITY_POOL_SPEND_PROPOSAL_WITH_DEPOSIT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.CommunityPoolSpendProposalWithDeposit"
    )]
    impl ::prost_wkt::MessageSerde for CommunityPoolSpendProposalWithDeposit {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "CommunityPoolSpendProposalWithDeposit"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.CommunityPoolSpendProposalWithDeposit"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.CommunityPoolSpendProposalWithDeposit" , decoder : | buf : & [u8] | { let msg : CommunityPoolSpendProposalWithDeposit = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryParamsRequest")]
    impl ::prost_wkt::MessageSerde for QueryParamsRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryParamsRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryParamsRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryParamsRequest" , decoder : | buf : & [u8] | { let msg : QueryParamsRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryParamsResponse")]
    impl ::prost_wkt::MessageSerde for QueryParamsResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryParamsResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryParamsResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryParamsResponse" , decoder : | buf : & [u8] | { let msg : QueryParamsResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_VALIDATOR_OUTSTANDING_REWARDS_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorOutstandingRewardsRequest"
    )]
    impl ::prost_wkt::MessageSerde for QueryValidatorOutstandingRewardsRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryValidatorOutstandingRewardsRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorOutstandingRewardsRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorOutstandingRewardsRequest" , decoder : | buf : & [u8] | { let msg : QueryValidatorOutstandingRewardsRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_VALIDATOR_OUTSTANDING_REWARDS_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorOutstandingRewardsResponse"
    )]
    impl ::prost_wkt::MessageSerde for QueryValidatorOutstandingRewardsResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryValidatorOutstandingRewardsResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorOutstandingRewardsResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorOutstandingRewardsResponse" , decoder : | buf : & [u8] | { let msg : QueryValidatorOutstandingRewardsResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_VALIDATOR_COMMISSION_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorCommissionRequest"
    )]
    impl ::prost_wkt::MessageSerde for QueryValidatorCommissionRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryValidatorCommissionRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorCommissionRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorCommissionRequest" , decoder : | buf : & [u8] | { let msg : QueryValidatorCommissionRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_VALIDATOR_COMMISSION_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorCommissionResponse"
    )]
    impl ::prost_wkt::MessageSerde for QueryValidatorCommissionResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryValidatorCommissionResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorCommissionResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorCommissionResponse" , decoder : | buf : & [u8] | { let msg : QueryValidatorCommissionResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_VALIDATOR_SLASHES_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorSlashesRequest"
    )]
    impl ::prost_wkt::MessageSerde for QueryValidatorSlashesRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryValidatorSlashesRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorSlashesRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorSlashesRequest" , decoder : | buf : & [u8] | { let msg : QueryValidatorSlashesRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_VALIDATOR_SLASHES_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorSlashesResponse"
    )]
    impl ::prost_wkt::MessageSerde for QueryValidatorSlashesResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryValidatorSlashesResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorSlashesResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryValidatorSlashesResponse" , decoder : | buf : & [u8] | { let msg : QueryValidatorSlashesResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_DELEGATION_REWARDS_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegationRewardsRequest"
    )]
    impl ::prost_wkt::MessageSerde for QueryDelegationRewardsRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryDelegationRewardsRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegationRewardsRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegationRewardsRequest" , decoder : | buf : & [u8] | { let msg : QueryDelegationRewardsRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_DELEGATION_REWARDS_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegationRewardsResponse"
    )]
    impl ::prost_wkt::MessageSerde for QueryDelegationRewardsResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryDelegationRewardsResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegationRewardsResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegationRewardsResponse" , decoder : | buf : & [u8] | { let msg : QueryDelegationRewardsResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_DELEGATION_TOTAL_REWARDS_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegationTotalRewardsRequest"
    )]
    impl ::prost_wkt::MessageSerde for QueryDelegationTotalRewardsRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryDelegationTotalRewardsRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegationTotalRewardsRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegationTotalRewardsRequest" , decoder : | buf : & [u8] | { let msg : QueryDelegationTotalRewardsRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_DELEGATION_TOTAL_REWARDS_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegationTotalRewardsResponse"
    )]
    impl ::prost_wkt::MessageSerde for QueryDelegationTotalRewardsResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryDelegationTotalRewardsResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegationTotalRewardsResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegationTotalRewardsResponse" , decoder : | buf : & [u8] | { let msg : QueryDelegationTotalRewardsResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_DELEGATOR_VALIDATORS_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegatorValidatorsRequest"
    )]
    impl ::prost_wkt::MessageSerde for QueryDelegatorValidatorsRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryDelegatorValidatorsRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegatorValidatorsRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegatorValidatorsRequest" , decoder : | buf : & [u8] | { let msg : QueryDelegatorValidatorsRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_DELEGATOR_VALIDATORS_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegatorValidatorsResponse"
    )]
    impl ::prost_wkt::MessageSerde for QueryDelegatorValidatorsResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryDelegatorValidatorsResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegatorValidatorsResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegatorValidatorsResponse" , decoder : | buf : & [u8] | { let msg : QueryDelegatorValidatorsResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_DELEGATOR_WITHDRAW_ADDRESS_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegatorWithdrawAddressRequest"
    )]
    impl ::prost_wkt::MessageSerde for QueryDelegatorWithdrawAddressRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryDelegatorWithdrawAddressRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegatorWithdrawAddressRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegatorWithdrawAddressRequest" , decoder : | buf : & [u8] | { let msg : QueryDelegatorWithdrawAddressRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_DELEGATOR_WITHDRAW_ADDRESS_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegatorWithdrawAddressResponse"
    )]
    impl ::prost_wkt::MessageSerde for QueryDelegatorWithdrawAddressResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryDelegatorWithdrawAddressResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegatorWithdrawAddressResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryDelegatorWithdrawAddressResponse" , decoder : | buf : & [u8] | { let msg : QueryDelegatorWithdrawAddressResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_COMMUNITY_POOL_REQUEST: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryCommunityPoolRequest"
    )]
    impl ::prost_wkt::MessageSerde for QueryCommunityPoolRequest {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryCommunityPoolRequest"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryCommunityPoolRequest"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryCommunityPoolRequest" , decoder : | buf : & [u8] | { let msg : QueryCommunityPoolRequest = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_QUERY_COMMUNITY_POOL_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.QueryCommunityPoolResponse"
    )]
    impl ::prost_wkt::MessageSerde for QueryCommunityPoolResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "QueryCommunityPoolResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.QueryCommunityPoolResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.QueryCommunityPoolResponse" , decoder : | buf : & [u8] | { let msg : QueryCommunityPoolResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_SET_WITHDRAW_ADDRESS: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.MsgSetWithdrawAddress"
    )]
    impl ::prost_wkt::MessageSerde for MsgSetWithdrawAddress {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgSetWithdrawAddress"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.MsgSetWithdrawAddress"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.MsgSetWithdrawAddress" , decoder : | buf : & [u8] | { let msg : MsgSetWithdrawAddress = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_SET_WITHDRAW_ADDRESS_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.MsgSetWithdrawAddressResponse"
    )]
    impl ::prost_wkt::MessageSerde for MsgSetWithdrawAddressResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgSetWithdrawAddressResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.MsgSetWithdrawAddressResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.MsgSetWithdrawAddressResponse" , decoder : | buf : & [u8] | { let msg : MsgSetWithdrawAddressResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_WITHDRAW_DELEGATOR_REWARD: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward"
    )]
    impl ::prost_wkt::MessageSerde for MsgWithdrawDelegatorReward {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgWithdrawDelegatorReward"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward" , decoder : | buf : & [u8] | { let msg : MsgWithdrawDelegatorReward = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_WITHDRAW_DELEGATOR_REWARD_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.MsgWithdrawDelegatorRewardResponse"
    )]
    impl ::prost_wkt::MessageSerde for MsgWithdrawDelegatorRewardResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgWithdrawDelegatorRewardResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.MsgWithdrawDelegatorRewardResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.MsgWithdrawDelegatorRewardResponse" , decoder : | buf : & [u8] | { let msg : MsgWithdrawDelegatorRewardResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_WITHDRAW_VALIDATOR_COMMISSION: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission"
    )]
    impl ::prost_wkt::MessageSerde for MsgWithdrawValidatorCommission {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgWithdrawValidatorCommission"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission" , decoder : | buf : & [u8] | { let msg : MsgWithdrawValidatorCommission = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_WITHDRAW_VALIDATOR_COMMISSION_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommissionResponse"
    )]
    impl ::prost_wkt::MessageSerde for MsgWithdrawValidatorCommissionResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgWithdrawValidatorCommissionResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommissionResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommissionResponse" , decoder : | buf : & [u8] | { let msg : MsgWithdrawValidatorCommissionResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_FUND_COMMUNITY_POOL: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.distribution.v1beta1.MsgFundCommunityPool")]
    impl ::prost_wkt::MessageSerde for MsgFundCommunityPool {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgFundCommunityPool"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.MsgFundCommunityPool"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.MsgFundCommunityPool" , decoder : | buf : & [u8] | { let msg : MsgFundCommunityPool = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MSG_FUND_COMMUNITY_POOL_RESPONSE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.MsgFundCommunityPoolResponse"
    )]
    impl ::prost_wkt::MessageSerde for MsgFundCommunityPoolResponse {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MsgFundCommunityPoolResponse"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.MsgFundCommunityPoolResponse"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.MsgFundCommunityPoolResponse" , decoder : | buf : & [u8] | { let msg : MsgFundCommunityPoolResponse = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_DELEGATOR_WITHDRAW_INFO: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.DelegatorWithdrawInfo"
    )]
    impl ::prost_wkt::MessageSerde for DelegatorWithdrawInfo {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "DelegatorWithdrawInfo"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.DelegatorWithdrawInfo"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.DelegatorWithdrawInfo" , decoder : | buf : & [u8] | { let msg : DelegatorWithdrawInfo = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_VALIDATOR_OUTSTANDING_REWARDS_RECORD: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorOutstandingRewardsRecord"
    )]
    impl ::prost_wkt::MessageSerde for ValidatorOutstandingRewardsRecord {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "ValidatorOutstandingRewardsRecord"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorOutstandingRewardsRecord"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorOutstandingRewardsRecord" , decoder : | buf : & [u8] | { let msg : ValidatorOutstandingRewardsRecord = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_VALIDATOR_ACCUMULATED_COMMISSION_RECORD: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorAccumulatedCommissionRecord"
    )]
    impl ::prost_wkt::MessageSerde for ValidatorAccumulatedCommissionRecord {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "ValidatorAccumulatedCommissionRecord"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorAccumulatedCommissionRecord"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorAccumulatedCommissionRecord" , decoder : | buf : & [u8] | { let msg : ValidatorAccumulatedCommissionRecord = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_VALIDATOR_HISTORICAL_REWARDS_RECORD: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorHistoricalRewardsRecord"
    )]
    impl ::prost_wkt::MessageSerde for ValidatorHistoricalRewardsRecord {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "ValidatorHistoricalRewardsRecord"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorHistoricalRewardsRecord"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorHistoricalRewardsRecord" , decoder : | buf : & [u8] | { let msg : ValidatorHistoricalRewardsRecord = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_VALIDATOR_CURRENT_REWARDS_RECORD: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorCurrentRewardsRecord"
    )]
    impl ::prost_wkt::MessageSerde for ValidatorCurrentRewardsRecord {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "ValidatorCurrentRewardsRecord"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorCurrentRewardsRecord"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorCurrentRewardsRecord" , decoder : | buf : & [u8] | { let msg : ValidatorCurrentRewardsRecord = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_DELEGATOR_STARTING_INFO_RECORD: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.DelegatorStartingInfoRecord"
    )]
    impl ::prost_wkt::MessageSerde for DelegatorStartingInfoRecord {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "DelegatorStartingInfoRecord"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.DelegatorStartingInfoRecord"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.DelegatorStartingInfoRecord" , decoder : | buf : & [u8] | { let msg : DelegatorStartingInfoRecord = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_VALIDATOR_SLASH_EVENT_RECORD: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorSlashEventRecord"
    )]
    impl ::prost_wkt::MessageSerde for ValidatorSlashEventRecord {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "ValidatorSlashEventRecord"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorSlashEventRecord"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.ValidatorSlashEventRecord" , decoder : | buf : & [u8] | { let msg : ValidatorSlashEventRecord = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.distribution.v1beta1.GenesisState")]
    impl ::prost_wkt::MessageSerde for GenesisState {
        fn package_name(&self) -> &'static str {
            "cosmos.distribution.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "GenesisState"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.distribution.v1beta1.GenesisState"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.distribution.v1beta1.GenesisState" , decoder : | buf : & [u8] | { let msg : GenesisState = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
