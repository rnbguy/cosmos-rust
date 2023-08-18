/// InterchainAccountPacketData is comprised of a raw transaction, type of transaction and optional memo field.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterchainAccountPacketData {
    #[prost(enumeration = "Type", tag = "1")]
    pub r#type: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub memo: ::prost::alloc::string::String,
}
/// CosmosTx contains a list of sdk.Msg's. It should be used when sending transactions to an SDK host chain.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosTx {
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<::prost_wkt_types::Any>,
}
/// Type defines a classification of message issued from a controller chain to its associated interchain accounts
/// host
#[derive(
    ::serde::Serialize,
    ::serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum Type {
    /// Default zero value enumeration
    Unspecified = 0,
    /// Execute a transaction on an interchain accounts host chain
    ExecuteTx = 1,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::Unspecified => "TYPE_UNSPECIFIED",
            Type::ExecuteTx => "TYPE_EXECUTE_TX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TYPE_EXECUTE_TX" => Some(Self::ExecuteTx),
            _ => None,
        }
    }
}
/// An InterchainAccount is defined as a BaseAccount & the address of the account owner on the controller chain
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterchainAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account:
        ::core::option::Option<super::super::super::super::cosmos::auth::v1beta1::BaseAccount>,
    #[prost(string, tag = "2")]
    pub account_owner: ::prost::alloc::string::String,
}
/// Metadata defines a set of protocol specific data encoded into the ICS27 channel version bytestring
/// See ICS004: <https://github.com/cosmos/ibc/tree/master/spec/core/ics-004-channel-and-packet-semantics#Versioning>
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// version defines the ICS27 protocol version
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// controller_connection_id is the connection identifier associated with the controller chain
    #[prost(string, tag = "2")]
    pub controller_connection_id: ::prost::alloc::string::String,
    /// host_connection_id is the connection identifier associated with the host chain
    #[prost(string, tag = "3")]
    pub host_connection_id: ::prost::alloc::string::String,
    /// address defines the interchain account address to be fulfilled upon the OnChanOpenTry handshake step
    /// NOTE: the address field is empty on the OnChanOpenInit handshake step
    #[prost(string, tag = "4")]
    pub address: ::prost::alloc::string::String,
    /// encoding defines the supported codec format
    #[prost(string, tag = "5")]
    pub encoding: ::prost::alloc::string::String,
    /// tx_type defines the type of transactions the interchain account can execute
    #[prost(string, tag = "6")]
    pub tx_type: ::prost::alloc::string::String,
}
/// GenesisState defines the interchain accounts genesis state
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub controller_genesis_state: ::core::option::Option<ControllerGenesisState>,
    #[prost(message, optional, tag = "2")]
    pub host_genesis_state: ::core::option::Option<HostGenesisState>,
}
/// ControllerGenesisState defines the interchain accounts controller genesis state
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControllerGenesisState {
    #[prost(message, repeated, tag = "1")]
    pub active_channels: ::prost::alloc::vec::Vec<ActiveChannel>,
    #[prost(message, repeated, tag = "2")]
    pub interchain_accounts: ::prost::alloc::vec::Vec<RegisteredInterchainAccount>,
    #[prost(string, repeated, tag = "3")]
    pub ports: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub params: ::core::option::Option<super::controller::v1::Params>,
}
/// HostGenesisState defines the interchain accounts host genesis state
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostGenesisState {
    #[prost(message, repeated, tag = "1")]
    pub active_channels: ::prost::alloc::vec::Vec<ActiveChannel>,
    #[prost(message, repeated, tag = "2")]
    pub interchain_accounts: ::prost::alloc::vec::Vec<RegisteredInterchainAccount>,
    #[prost(string, tag = "3")]
    pub port: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub params: ::core::option::Option<super::host::v1::Params>,
}
/// ActiveChannel contains a connection ID, port ID and associated active channel ID
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveChannel {
    #[prost(string, tag = "1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub channel_id: ::prost::alloc::string::String,
}
/// RegisteredInterchainAccount contains a connection ID, port ID and associated interchain account address
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredInterchainAccount {
    #[prost(string, tag = "1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account_address: ::prost::alloc::string::String,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_INTERCHAIN_ACCOUNT_PACKET_DATA: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/ibc.applications.interchain_accounts.v1.InterchainAccountPacketData"
    )]
    impl ::prost_wkt::MessageSerde for InterchainAccountPacketData {
        fn package_name(&self) -> &'static str {
            "ibc.applications.interchain_accounts.v1"
        }
        fn message_name(&self) -> &'static str {
            "InterchainAccountPacketData"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.applications.interchain_accounts.v1.InterchainAccountPacketData"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.applications.interchain_accounts.v1.InterchainAccountPacketData" , decoder : | buf : & [u8] | { let msg : InterchainAccountPacketData = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_COSMOS_TX: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/ibc.applications.interchain_accounts.v1.CosmosTx")]
    impl ::prost_wkt::MessageSerde for CosmosTx {
        fn package_name(&self) -> &'static str {
            "ibc.applications.interchain_accounts.v1"
        }
        fn message_name(&self) -> &'static str {
            "CosmosTx"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.applications.interchain_accounts.v1.CosmosTx"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.applications.interchain_accounts.v1.CosmosTx" , decoder : | buf : & [u8] | { let msg : CosmosTx = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_INTERCHAIN_ACCOUNT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/ibc.applications.interchain_accounts.v1.InterchainAccount"
    )]
    impl ::prost_wkt::MessageSerde for InterchainAccount {
        fn package_name(&self) -> &'static str {
            "ibc.applications.interchain_accounts.v1"
        }
        fn message_name(&self) -> &'static str {
            "InterchainAccount"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.applications.interchain_accounts.v1.InterchainAccount"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.applications.interchain_accounts.v1.InterchainAccount" , decoder : | buf : & [u8] | { let msg : InterchainAccount = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_METADATA: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/ibc.applications.interchain_accounts.v1.Metadata")]
    impl ::prost_wkt::MessageSerde for Metadata {
        fn package_name(&self) -> &'static str {
            "ibc.applications.interchain_accounts.v1"
        }
        fn message_name(&self) -> &'static str {
            "Metadata"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.applications.interchain_accounts.v1.Metadata"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.applications.interchain_accounts.v1.Metadata" , decoder : | buf : & [u8] | { let msg : Metadata = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/ibc.applications.interchain_accounts.v1.GenesisState"
    )]
    impl ::prost_wkt::MessageSerde for GenesisState {
        fn package_name(&self) -> &'static str {
            "ibc.applications.interchain_accounts.v1"
        }
        fn message_name(&self) -> &'static str {
            "GenesisState"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.applications.interchain_accounts.v1.GenesisState"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.applications.interchain_accounts.v1.GenesisState" , decoder : | buf : & [u8] | { let msg : GenesisState = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_CONTROLLER_GENESIS_STATE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/ibc.applications.interchain_accounts.v1.ControllerGenesisState"
    )]
    impl ::prost_wkt::MessageSerde for ControllerGenesisState {
        fn package_name(&self) -> &'static str {
            "ibc.applications.interchain_accounts.v1"
        }
        fn message_name(&self) -> &'static str {
            "ControllerGenesisState"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.applications.interchain_accounts.v1.ControllerGenesisState"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.applications.interchain_accounts.v1.ControllerGenesisState" , decoder : | buf : & [u8] | { let msg : ControllerGenesisState = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_HOST_GENESIS_STATE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/ibc.applications.interchain_accounts.v1.HostGenesisState"
    )]
    impl ::prost_wkt::MessageSerde for HostGenesisState {
        fn package_name(&self) -> &'static str {
            "ibc.applications.interchain_accounts.v1"
        }
        fn message_name(&self) -> &'static str {
            "HostGenesisState"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.applications.interchain_accounts.v1.HostGenesisState"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.applications.interchain_accounts.v1.HostGenesisState" , decoder : | buf : & [u8] | { let msg : HostGenesisState = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_ACTIVE_CHANNEL: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/ibc.applications.interchain_accounts.v1.ActiveChannel"
    )]
    impl ::prost_wkt::MessageSerde for ActiveChannel {
        fn package_name(&self) -> &'static str {
            "ibc.applications.interchain_accounts.v1"
        }
        fn message_name(&self) -> &'static str {
            "ActiveChannel"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.applications.interchain_accounts.v1.ActiveChannel"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.applications.interchain_accounts.v1.ActiveChannel" , decoder : | buf : & [u8] | { let msg : ActiveChannel = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_REGISTERED_INTERCHAIN_ACCOUNT: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/ibc.applications.interchain_accounts.v1.RegisteredInterchainAccount"
    )]
    impl ::prost_wkt::MessageSerde for RegisteredInterchainAccount {
        fn package_name(&self) -> &'static str {
            "ibc.applications.interchain_accounts.v1"
        }
        fn message_name(&self) -> &'static str {
            "RegisteredInterchainAccount"
        }
        fn type_url(&self) -> &'static str {
            "/ibc.applications.interchain_accounts.v1.RegisteredInterchainAccount"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/ibc.applications.interchain_accounts.v1.RegisteredInterchainAccount" , decoder : | buf : & [u8] | { let msg : RegisteredInterchainAccount = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
