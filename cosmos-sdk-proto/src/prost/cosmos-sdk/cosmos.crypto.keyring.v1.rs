/// Record is used for representing a key in the keyring.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Record {
    /// name represents a name of Record
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// pub_key represents a public key in any format
    #[prost(message, optional, tag = "2")]
    pub pub_key: ::core::option::Option<::prost_wkt_types::Any>,
    /// Record contains one of the following items
    #[prost(oneof = "record::Item", tags = "3, 4, 5, 6")]
    pub item: ::core::option::Option<record::Item>,
}
/// Nested message and enum types in `Record`.
pub mod record {
    /// Item is a keyring item stored in a keyring backend.
    /// Local item
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Local {
        #[prost(message, optional, tag = "1")]
        pub priv_key: ::core::option::Option<::prost_wkt_types::Any>,
    }
    /// Ledger item
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Ledger {
        #[prost(message, optional, tag = "1")]
        pub path: ::core::option::Option<super::super::super::hd::v1::Bip44Params>,
    }
    /// Multi item
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Multi {}
    /// Offline item
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Offline {}
    /// Record contains one of the following items
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        /// local stores the private key locally.
        #[prost(message, tag = "3")]
        Local(Local),
        /// ledger stores the information about a Ledger key.
        #[prost(message, tag = "4")]
        Ledger(Ledger),
        /// Multi does not store any other information.
        #[prost(message, tag = "5")]
        Multi(Multi),
        /// Offline does not store any other information.
        #[prost(message, tag = "6")]
        Offline(Offline),
    }
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_RECORD: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.crypto.keyring.v1.Record")]
    impl ::prost_wkt::MessageSerde for Record {
        fn package_name(&self) -> &'static str {
            "cosmos.crypto.keyring.v1"
        }
        fn message_name(&self) -> &'static str {
            "Record"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.crypto.keyring.v1.Record"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.crypto.keyring.v1.Record" , decoder : | buf : & [u8] | { let msg : Record = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
