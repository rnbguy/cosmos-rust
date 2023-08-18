/// GenesisState defines the ibc module's genesis state.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// ICS002 - Clients genesis state
    #[prost(message, optional, tag = "1")]
    pub client_genesis: ::core::option::Option<super::super::client::v1::GenesisState>,
    /// ICS003 - Connections genesis state
    #[prost(message, optional, tag = "2")]
    pub connection_genesis: ::core::option::Option<super::super::connection::v1::GenesisState>,
    /// ICS004 - Channel genesis state
    #[prost(message, optional, tag = "3")]
    pub channel_genesis: ::core::option::Option<super::super::channel::v1::GenesisState>,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/ibc.core.types.v1.GenesisState")]
    impl ::prost_wkt::MessageSerde for GenesisState {
        fn package_name(&self) -> &'static str {
            "ibc.core.types.v1"
        }
        fn message_name(&self) -> &'static str {
            "GenesisState"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.core.types.v1.GenesisState"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.core.types.v1.GenesisState" , decoder : | buf : & [u8] | { let msg : GenesisState = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
