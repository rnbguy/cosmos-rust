/// Snapshot contains Tendermint state sync snapshot info.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(uint32, tag = "2")]
    pub format: u32,
    #[prost(uint32, tag = "3")]
    pub chunks: u32,
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub metadata: ::core::option::Option<Metadata>,
}
/// Metadata contains SDK-specific snapshot metadata.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// SHA-256 chunk hashes
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub chunk_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// SnapshotItem is an item contained in a rootmulti.Store snapshot.
///
/// Since: cosmos-sdk 0.46
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotItem {
    /// item is the specific type of snapshot item.
    #[prost(oneof = "snapshot_item::Item", tags = "1, 2, 3, 4, 5, 6")]
    pub item: ::core::option::Option<snapshot_item::Item>,
}
/// Nested message and enum types in `SnapshotItem`.
pub mod snapshot_item {
    /// item is the specific type of snapshot item.
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        #[prost(message, tag = "1")]
        Store(super::SnapshotStoreItem),
        #[prost(message, tag = "2")]
        Iavl(super::SnapshotIavlItem),
        #[prost(message, tag = "3")]
        Extension(super::SnapshotExtensionMeta),
        #[prost(message, tag = "4")]
        ExtensionPayload(super::SnapshotExtensionPayload),
        #[prost(message, tag = "5")]
        Kv(super::SnapshotKvItem),
        #[prost(message, tag = "6")]
        Schema(super::SnapshotSchema),
    }
}
/// SnapshotStoreItem contains metadata about a snapshotted store.
///
/// Since: cosmos-sdk 0.46
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotStoreItem {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// SnapshotIAVLItem is an exported IAVL node.
///
/// Since: cosmos-sdk 0.46
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotIavlItem {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// version is block height
    #[prost(int64, tag = "3")]
    pub version: i64,
    /// height is depth of the tree.
    #[prost(int32, tag = "4")]
    pub height: i32,
}
/// SnapshotExtensionMeta contains metadata about an external snapshotter.
///
/// Since: cosmos-sdk 0.46
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotExtensionMeta {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub format: u32,
}
/// SnapshotExtensionPayload contains payloads of an external snapshotter.
///
/// Since: cosmos-sdk 0.46
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotExtensionPayload {
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// SnapshotKVItem is an exported Key/Value Pair
///
/// Since: cosmos-sdk 0.46
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotKvItem {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// SnapshotSchema is an exported schema of smt store
///
/// Since: cosmos-sdk 0.46
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotSchema {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_SNAPSHOT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.snapshots.v1beta1.Snapshot")]
    impl ::prost_wkt::MessageSerde for Snapshot {
        fn package_name(&self) -> &'static str {
            "cosmos.base.snapshots.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "Snapshot"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.snapshots.v1beta1.Snapshot"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.snapshots.v1beta1.Snapshot" , decoder : | buf : & [u8] | { let msg : Snapshot = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_METADATA: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.snapshots.v1beta1.Metadata")]
    impl ::prost_wkt::MessageSerde for Metadata {
        fn package_name(&self) -> &'static str {
            "cosmos.base.snapshots.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "Metadata"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.snapshots.v1beta1.Metadata"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.snapshots.v1beta1.Metadata" , decoder : | buf : & [u8] | { let msg : Metadata = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_SNAPSHOT_ITEM: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotItem")]
    impl ::prost_wkt::MessageSerde for SnapshotItem {
        fn package_name(&self) -> &'static str {
            "cosmos.base.snapshots.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "SnapshotItem"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotItem"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotItem" , decoder : | buf : & [u8] | { let msg : SnapshotItem = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_SNAPSHOT_STORE_ITEM: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotStoreItem")]
    impl ::prost_wkt::MessageSerde for SnapshotStoreItem {
        fn package_name(&self) -> &'static str {
            "cosmos.base.snapshots.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "SnapshotStoreItem"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotStoreItem"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotStoreItem" , decoder : | buf : & [u8] | { let msg : SnapshotStoreItem = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_SNAPSHOT_IAVL_ITEM: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotIAVLItem")]
    impl ::prost_wkt::MessageSerde for SnapshotIavlItem {
        fn package_name(&self) -> &'static str {
            "cosmos.base.snapshots.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "SnapshotIAVLItem"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotIAVLItem"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotIAVLItem" , decoder : | buf : & [u8] | { let msg : SnapshotIavlItem = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_SNAPSHOT_EXTENSION_META: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotExtensionMeta"
    )]
    impl ::prost_wkt::MessageSerde for SnapshotExtensionMeta {
        fn package_name(&self) -> &'static str {
            "cosmos.base.snapshots.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "SnapshotExtensionMeta"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotExtensionMeta"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotExtensionMeta" , decoder : | buf : & [u8] | { let msg : SnapshotExtensionMeta = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_SNAPSHOT_EXTENSION_PAYLOAD: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotExtensionPayload"
    )]
    impl ::prost_wkt::MessageSerde for SnapshotExtensionPayload {
        fn package_name(&self) -> &'static str {
            "cosmos.base.snapshots.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "SnapshotExtensionPayload"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotExtensionPayload"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotExtensionPayload" , decoder : | buf : & [u8] | { let msg : SnapshotExtensionPayload = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_SNAPSHOT_KV_ITEM: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotKVItem")]
    impl ::prost_wkt::MessageSerde for SnapshotKvItem {
        fn package_name(&self) -> &'static str {
            "cosmos.base.snapshots.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "SnapshotKVItem"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotKVItem"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotKVItem" , decoder : | buf : & [u8] | { let msg : SnapshotKvItem = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_SNAPSHOT_SCHEMA: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotSchema")]
    impl ::prost_wkt::MessageSerde for SnapshotSchema {
        fn package_name(&self) -> &'static str {
            "cosmos.base.snapshots.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "SnapshotSchema"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotSchema"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.snapshots.v1beta1.SnapshotSchema" , decoder : | buf : & [u8] | { let msg : SnapshotSchema = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
