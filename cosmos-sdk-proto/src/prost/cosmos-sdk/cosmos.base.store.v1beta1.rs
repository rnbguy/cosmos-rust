/// CommitInfo defines commit information used by the multi-store when committing
/// a version/height.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitInfo {
    #[prost(int64, tag = "1")]
    pub version: i64,
    #[prost(message, repeated, tag = "2")]
    pub store_infos: ::prost::alloc::vec::Vec<StoreInfo>,
}
/// StoreInfo defines store-specific commit information. It contains a reference
/// between a store name and the commit ID.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub commit_id: ::core::option::Option<CommitId>,
}
/// CommitID defines the committment information when a specific store is
/// committed.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitId {
    #[prost(int64, tag = "1")]
    pub version: i64,
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// StoreKVPair is a KVStore KVPair used for listening to state changes (Sets and Deletes)
/// It optionally includes the StoreKey for the originating KVStore and a Boolean flag to distinguish between Sets and
/// Deletes
///
/// Since: cosmos-sdk 0.43
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreKvPair {
    /// the store key for the KVStore this pair originates from
    #[prost(string, tag = "1")]
    pub store_key: ::prost::alloc::string::String,
    /// true indicates a delete operation, false indicates a set operation
    #[prost(bool, tag = "2")]
    pub delete: bool,
    #[prost(bytes = "vec", tag = "3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// BlockMetadata contains all the abci event data of a block
/// the file streamer dump them into files together with the state changes.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMetadata {
    #[prost(message, optional, tag = "1")]
    #[serde(skip)]
    pub request_begin_block:
        ::core::option::Option<::tendermint_proto::v0_34::abci::RequestBeginBlock>,
    #[prost(message, optional, tag = "2")]
    #[serde(skip)]
    pub response_begin_block:
        ::core::option::Option<::tendermint_proto::v0_34::abci::ResponseBeginBlock>,
    #[prost(message, repeated, tag = "3")]
    pub deliver_txs: ::prost::alloc::vec::Vec<block_metadata::DeliverTx>,
    #[prost(message, optional, tag = "4")]
    #[serde(skip)]
    pub request_end_block: ::core::option::Option<::tendermint_proto::v0_34::abci::RequestEndBlock>,
    #[prost(message, optional, tag = "5")]
    #[serde(skip)]
    pub response_end_block:
        ::core::option::Option<::tendermint_proto::v0_34::abci::ResponseEndBlock>,
    #[prost(message, optional, tag = "6")]
    #[serde(skip)]
    pub response_commit: ::core::option::Option<::tendermint_proto::v0_34::abci::ResponseCommit>,
}
/// Nested message and enum types in `BlockMetadata`.
pub mod block_metadata {
    /// DeliverTx encapulate deliver tx request and response.
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeliverTx {
        #[prost(message, optional, tag = "1")]
        #[serde(skip)]
        pub request: ::core::option::Option<::tendermint_proto::v0_34::abci::RequestDeliverTx>,
        #[prost(message, optional, tag = "2")]
        #[serde(skip)]
        pub response: ::core::option::Option<::tendermint_proto::v0_34::abci::ResponseDeliverTx>,
    }
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_COMMIT_INFO: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.store.v1beta1.CommitInfo")]
    impl ::prost_wkt::MessageSerde for CommitInfo {
        fn package_name(&self) -> &'static str {
            "cosmos.base.store.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "CommitInfo"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.store.v1beta1.CommitInfo"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.store.v1beta1.CommitInfo" , decoder : | buf : & [u8] | { let msg : CommitInfo = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_STORE_INFO: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.store.v1beta1.StoreInfo")]
    impl ::prost_wkt::MessageSerde for StoreInfo {
        fn package_name(&self) -> &'static str {
            "cosmos.base.store.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "StoreInfo"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.store.v1beta1.StoreInfo"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.store.v1beta1.StoreInfo" , decoder : | buf : & [u8] | { let msg : StoreInfo = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_COMMIT_ID: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.store.v1beta1.CommitID")]
    impl ::prost_wkt::MessageSerde for CommitId {
        fn package_name(&self) -> &'static str {
            "cosmos.base.store.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "CommitID"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.store.v1beta1.CommitID"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.store.v1beta1.CommitID" , decoder : | buf : & [u8] | { let msg : CommitId = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_STORE_KV_PAIR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.store.v1beta1.StoreKVPair")]
    impl ::prost_wkt::MessageSerde for StoreKvPair {
        fn package_name(&self) -> &'static str {
            "cosmos.base.store.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "StoreKVPair"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.store.v1beta1.StoreKVPair"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.store.v1beta1.StoreKVPair" , decoder : | buf : & [u8] | { let msg : StoreKvPair = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_BLOCK_METADATA: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.store.v1beta1.BlockMetadata")]
    impl ::prost_wkt::MessageSerde for BlockMetadata {
        fn package_name(&self) -> &'static str {
            "cosmos.base.store.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "BlockMetadata"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.store.v1beta1.BlockMetadata"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.store.v1beta1.BlockMetadata" , decoder : | buf : & [u8] | { let msg : BlockMetadata = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
