/// Pairs defines a repeated slice of Pair objects.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pairs {
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::prost::alloc::vec::Vec<Pair>,
}
/// Pair defines a key/value bytes tuple.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_PAIRS: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.base.kv.v1beta1.Pairs")]
    impl ::prost_wkt::MessageSerde for Pairs {
        fn package_name(&self) -> &'static str {
            "cosmos.base.kv.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "Pairs"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.kv.v1beta1.Pairs"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.kv.v1beta1.Pairs" , decoder : | buf : & [u8] | { let msg : Pairs = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_PAIR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.base.kv.v1beta1.Pair")]
    impl ::prost_wkt::MessageSerde for Pair {
        fn package_name(&self) -> &'static str {
            "cosmos.base.kv.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "Pair"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.base.kv.v1beta1.Pair"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.base.kv.v1beta1.Pair" , decoder : | buf : & [u8] | { let msg : Pair = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
