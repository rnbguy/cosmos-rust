/// MerkleRoot defines a merkle root hash.
/// In the Cosmos SDK, the AppHash of a block header becomes the root.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerkleRoot {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// MerklePrefix is merkle path prefixed to the key.
/// The constructed key from the Path and the key will be append(Path.KeyPath,
/// append(Path.KeyPrefix, key...))
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerklePrefix {
    #[prost(bytes = "vec", tag = "1")]
    pub key_prefix: ::prost::alloc::vec::Vec<u8>,
}
/// MerklePath is the path used to verify commitment proofs, which can be an
/// arbitrary structured object (defined by a commitment type).
/// MerklePath is represented from root-to-leaf
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerklePath {
    #[prost(string, repeated, tag = "1")]
    pub key_path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MerkleProof is a wrapper type over a chain of CommitmentProofs.
/// It demonstrates membership or non-membership for an element or set of
/// elements, verifiable in conjunction with a known commitment root. Proofs
/// should be succinct.
/// MerkleProofs are ordered from leaf-to-root
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerkleProof {
    #[prost(message, repeated, tag = "1")]
    pub proofs: ::prost::alloc::vec::Vec<super::super::super::super::ics23::CommitmentProof>,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MERKLE_ROOT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/ibc.core.commitment.v1.MerkleRoot")]
    impl ::prost_wkt::MessageSerde for MerkleRoot {
        fn package_name(&self) -> &'static str {
            "ibc.core.commitment.v1"
        }
        fn message_name(&self) -> &'static str {
            "MerkleRoot"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.core.commitment.v1.MerkleRoot"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.core.commitment.v1.MerkleRoot" , decoder : | buf : & [u8] | { let msg : MerkleRoot = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MERKLE_PREFIX: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/ibc.core.commitment.v1.MerklePrefix")]
    impl ::prost_wkt::MessageSerde for MerklePrefix {
        fn package_name(&self) -> &'static str {
            "ibc.core.commitment.v1"
        }
        fn message_name(&self) -> &'static str {
            "MerklePrefix"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.core.commitment.v1.MerklePrefix"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.core.commitment.v1.MerklePrefix" , decoder : | buf : & [u8] | { let msg : MerklePrefix = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MERKLE_PATH: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/ibc.core.commitment.v1.MerklePath")]
    impl ::prost_wkt::MessageSerde for MerklePath {
        fn package_name(&self) -> &'static str {
            "ibc.core.commitment.v1"
        }
        fn message_name(&self) -> &'static str {
            "MerklePath"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.core.commitment.v1.MerklePath"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.core.commitment.v1.MerklePath" , decoder : | buf : & [u8] | { let msg : MerklePath = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MERKLE_PROOF: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/ibc.core.commitment.v1.MerkleProof")]
    impl ::prost_wkt::MessageSerde for MerkleProof {
        fn package_name(&self) -> &'static str {
            "ibc.core.commitment.v1"
        }
        fn message_name(&self) -> &'static str {
            "MerkleProof"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.core.commitment.v1.MerkleProof"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.core.commitment.v1.MerkleProof" , decoder : | buf : & [u8] | { let msg : MerkleProof = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
