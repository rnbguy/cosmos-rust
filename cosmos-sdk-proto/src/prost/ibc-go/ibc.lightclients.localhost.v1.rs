/// ClientState defines a loopback (localhost) client. It requires (read-only)
/// access to keys outside the client prefix.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    /// self chain ID
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// self latest block height
    #[prost(message, optional, tag = "2")]
    pub height: ::core::option::Option<super::super::super::core::client::v1::Height>,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_CLIENT_STATE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/ibc.lightclients.localhost.v1.ClientState")]
    impl ::prost_wkt::MessageSerde for ClientState {
        fn package_name(&self) -> &'static str {
            "ibc.lightclients.localhost.v1"
        }
        fn message_name(&self) -> &'static str {
            "ClientState"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.lightclients.localhost.v1.ClientState"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.lightclients.localhost.v1.ClientState" , decoder : | buf : & [u8] | { let msg : ClientState = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
