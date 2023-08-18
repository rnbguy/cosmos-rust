/// PubKey defines a secp256k1 public key
/// Key is the compressed form of the pubkey. The first byte depends is a 0x02 byte
/// if the y-coordinate is the lexicographically largest of the two associated with
/// the x-coordinate. Otherwise the first byte is a 0x03.
/// This prefix is followed with the x-coordinate.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKey {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(with = "serde_with::As::<serde_with::base64::Base64>")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// PrivKey defines a secp256k1 private key.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivKey {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(with = "serde_with::As::<serde_with::base64::Base64>")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_PUB_KEY: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.crypto.secp256k1.PubKey")]
    impl ::prost_wkt::MessageSerde for PubKey {
        fn package_name(&self) -> &'static str {
            "cosmos.crypto.secp256k1"
        }
        fn message_name(&self) -> &'static str {
            "PubKey"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.crypto.secp256k1.PubKey"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.crypto.secp256k1.PubKey" , decoder : | buf : & [u8] | { let msg : PubKey = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_PRIV_KEY: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.crypto.secp256k1.PrivKey")]
    impl ::prost_wkt::MessageSerde for PrivKey {
        fn package_name(&self) -> &'static str {
            "cosmos.crypto.secp256k1"
        }
        fn message_name(&self) -> &'static str {
            "PrivKey"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.crypto.secp256k1.PrivKey"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.crypto.secp256k1.PrivKey" , decoder : | buf : & [u8] | { let msg : PrivKey = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
