/// MultiSignature wraps the signatures from a multisig.LegacyAminoPubKey.
/// See cosmos.tx.v1betata1.ModeInfo.Multi for how to specify which signers
/// signed and with which modes.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiSignature {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// CompactBitArray is an implementation of a space efficient bit array.
/// This is used to ensure that the encoded data takes up a minimal amount of
/// space after proto encoding.
/// This is not thread safe, and is not intended for concurrent usage.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactBitArray {
    #[prost(uint32, tag = "1")]
    pub extra_bits_stored: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub elems: ::prost::alloc::vec::Vec<u8>,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_MULTI_SIGNATURE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.crypto.multisig.v1beta1.MultiSignature")]
    impl ::prost_wkt::MessageSerde for MultiSignature {
        fn package_name(&self) -> &'static str {
            "cosmos.crypto.multisig.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "MultiSignature"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.crypto.multisig.v1beta1.MultiSignature"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.crypto.multisig.v1beta1.MultiSignature" , decoder : | buf : & [u8] | { let msg : MultiSignature = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_COMPACT_BIT_ARRAY: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.crypto.multisig.v1beta1.CompactBitArray")]
    impl ::prost_wkt::MessageSerde for CompactBitArray {
        fn package_name(&self) -> &'static str {
            "cosmos.crypto.multisig.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "CompactBitArray"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.crypto.multisig.v1beta1.CompactBitArray"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.crypto.multisig.v1beta1.CompactBitArray" , decoder : | buf : & [u8] | { let msg : CompactBitArray = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
