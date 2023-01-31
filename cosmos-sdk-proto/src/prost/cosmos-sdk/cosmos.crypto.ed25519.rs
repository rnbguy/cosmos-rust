/// PubKey is an ed25519 public key for handling Tendermint keys in SDK.
/// It's needed for Any serialization and SDK compatibility.
/// It must not be used in a non Tendermint key context because it doesn't implement
/// ADR-28. Nevertheless, you will like to use ed25519 in app user level
/// then you must create a new proto message and follow ADR-28 for Address construction.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKey {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(with = "serde_with::As::<serde_with::base64::Base64>")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// Deprecated: PrivKey defines a ed25519 private key.
/// NOTE: ed25519 keys must not be used in SDK apps except in a tendermint validator context.
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
    #[typetag::serde(name = "type.googleapis.com/cosmos.crypto.ed25519.PubKey")]
    impl ::prost_wkt::MessageSerde for PubKey {
        fn package_name(&self) -> &'static str {
            "cosmos.crypto.ed25519"
        }
        fn message_name(&self) -> &'static str {
            "PubKey"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.crypto.ed25519.PubKey"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.crypto.ed25519.PubKey" , decoder : | buf : & [u8] | { let msg : PubKey = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_PRIV_KEY: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.crypto.ed25519.PrivKey")]
    impl ::prost_wkt::MessageSerde for PrivKey {
        fn package_name(&self) -> &'static str {
            "cosmos.crypto.ed25519"
        }
        fn message_name(&self) -> &'static str {
            "PrivKey"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.crypto.ed25519.PrivKey"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.crypto.ed25519.PrivKey" , decoder : | buf : & [u8] | { let msg : PrivKey = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
