/// Coin defines a token with a denomination and an amount.
///
/// NOTE: The amount field is an Int which implements the custom method
/// signatures required by gogoproto.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// DecCoin defines a token with a denomination and a decimal amount.
///
/// NOTE: The amount field is an Dec which implements the custom method
/// signatures required by gogoproto.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecCoin {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// IntProto defines a Protobuf wrapper around an Int object.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntProto {
    #[prost(string, tag = "1")]
    pub int: ::prost::alloc::string::String,
}
/// DecProto defines a Protobuf wrapper around a Dec object.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecProto {
    #[prost(string, tag = "1")]
    pub dec: ::prost::alloc::string::String,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_COIN: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.v1beta1.Coin")]
    impl ::prost_wkt::MessageSerde for Coin {
        fn package_name(&self) -> &'static str {
            "cosmos.base.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "Coin"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.v1beta1.Coin"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.v1beta1.Coin" , decoder : | buf : & [u8] | { let msg : Coin = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_DEC_COIN: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.v1beta1.DecCoin")]
    impl ::prost_wkt::MessageSerde for DecCoin {
        fn package_name(&self) -> &'static str {
            "cosmos.base.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "DecCoin"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.v1beta1.DecCoin"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.v1beta1.DecCoin" , decoder : | buf : & [u8] | { let msg : DecCoin = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_INT_PROTO: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.v1beta1.IntProto")]
    impl ::prost_wkt::MessageSerde for IntProto {
        fn package_name(&self) -> &'static str {
            "cosmos.base.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "IntProto"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.v1beta1.IntProto"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.v1beta1.IntProto" , decoder : | buf : & [u8] | { let msg : IntProto = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_DEC_PROTO: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.base.v1beta1.DecProto")]
    impl ::prost_wkt::MessageSerde for DecProto {
        fn package_name(&self) -> &'static str {
            "cosmos.base.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "DecProto"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.base.v1beta1.DecProto"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.base.v1beta1.DecProto" , decoder : | buf : & [u8] | { let msg : DecProto = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
