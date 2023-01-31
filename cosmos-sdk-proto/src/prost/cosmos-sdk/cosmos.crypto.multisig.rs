/// LegacyAminoPubKey specifies a public key type
/// which nests multiple public keys and a threshold,
/// it uses legacy amino address rules.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyAminoPubKey {
    #[prost(uint32, tag = "1")]
    pub threshold: u32,
    #[prost(message, repeated, tag = "2")]
    pub public_keys: ::prost::alloc::vec::Vec<::prost_wkt_types::Any>,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_LEGACY_AMINO_PUB_KEY: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.crypto.multisig.LegacyAminoPubKey")]
    impl ::prost_wkt::MessageSerde for LegacyAminoPubKey {
        fn package_name(&self) -> &'static str {
            "cosmos.crypto.multisig"
        }
        fn message_name(&self) -> &'static str {
            "LegacyAminoPubKey"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.crypto.multisig.LegacyAminoPubKey"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.crypto.multisig.LegacyAminoPubKey" , decoder : | buf : & [u8] | { let msg : LegacyAminoPubKey = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
