/// BIP44Params is used as path field in ledger item in Record.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bip44Params {
    /// purpose is a constant set to 44' (or 0x8000002C) following the BIP43 recommendation
    #[prost(uint32, tag = "1")]
    pub purpose: u32,
    /// coin_type is a constant that improves privacy
    #[prost(uint32, tag = "2")]
    pub coin_type: u32,
    /// account splits the key space into independent user identities
    #[prost(uint32, tag = "3")]
    pub account: u32,
    /// change is a constant used for public derivation. Constant 0 is used for external chain and constant 1 for internal
    /// chain.
    #[prost(bool, tag = "4")]
    pub change: bool,
    /// address_index is used as child index in BIP32 derivation
    #[prost(uint32, tag = "5")]
    pub address_index: u32,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_BIP44_PARAMS: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.crypto.hd.v1.BIP44Params")]
    impl ::prost_wkt::MessageSerde for Bip44Params {
        fn package_name(&self) -> &'static str {
            "cosmos.crypto.hd.v1"
        }
        fn message_name(&self) -> &'static str {
            "BIP44Params"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.crypto.hd.v1.BIP44Params"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.crypto.hd.v1.BIP44Params" , decoder : | buf : & [u8] | { let msg : Bip44Params = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
