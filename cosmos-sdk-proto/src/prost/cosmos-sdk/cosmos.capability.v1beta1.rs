/// Capability defines an implementation of an object capability. The index
/// provided to a Capability must be globally unique.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Capability {
    #[prost(uint64, tag = "1")]
    pub index: u64,
}
/// Owner defines a single capability owner. An owner is defined by the name of
/// capability and the module name.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Owner {
    #[prost(string, tag = "1")]
    pub module: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// CapabilityOwners defines a set of owners of a single Capability. The set of
/// owners must be unique.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityOwners {
    #[prost(message, repeated, tag = "1")]
    pub owners: ::prost::alloc::vec::Vec<Owner>,
}
/// GenesisOwners defines the capability owners with their corresponding index.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisOwners {
    /// index is the index of the capability owner.
    #[prost(uint64, tag = "1")]
    pub index: u64,
    /// index_owners are the owners at the given index.
    #[prost(message, optional, tag = "2")]
    pub index_owners: ::core::option::Option<CapabilityOwners>,
}
/// GenesisState defines the capability module's genesis state.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// index is the capability global index.
    #[prost(uint64, tag = "1")]
    pub index: u64,
    /// owners represents a map from index to owners of the capability index
    /// index key is string to allow amino marshalling.
    #[prost(message, repeated, tag = "2")]
    pub owners: ::prost::alloc::vec::Vec<GenesisOwners>,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_CAPABILITY: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.capability.v1beta1.Capability")]
    impl ::prost_wkt::MessageSerde for Capability {
        fn package_name(&self) -> &'static str {
            "cosmos.capability.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "Capability"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.capability.v1beta1.Capability"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.capability.v1beta1.Capability" , decoder : | buf : & [u8] | { let msg : Capability = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_OWNER: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.capability.v1beta1.Owner")]
    impl ::prost_wkt::MessageSerde for Owner {
        fn package_name(&self) -> &'static str {
            "cosmos.capability.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "Owner"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.capability.v1beta1.Owner"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.capability.v1beta1.Owner" , decoder : | buf : & [u8] | { let msg : Owner = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_CAPABILITY_OWNERS: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.capability.v1beta1.CapabilityOwners")]
    impl ::prost_wkt::MessageSerde for CapabilityOwners {
        fn package_name(&self) -> &'static str {
            "cosmos.capability.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "CapabilityOwners"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.capability.v1beta1.CapabilityOwners"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.capability.v1beta1.CapabilityOwners" , decoder : | buf : & [u8] | { let msg : CapabilityOwners = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GENESIS_OWNERS: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.capability.v1beta1.GenesisOwners")]
    impl ::prost_wkt::MessageSerde for GenesisOwners {
        fn package_name(&self) -> &'static str {
            "cosmos.capability.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "GenesisOwners"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.capability.v1beta1.GenesisOwners"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.capability.v1beta1.GenesisOwners" , decoder : | buf : & [u8] | { let msg : GenesisOwners = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/cosmos.capability.v1beta1.GenesisState")]
    impl ::prost_wkt::MessageSerde for GenesisState {
        fn package_name(&self) -> &'static str {
            "cosmos.capability.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "GenesisState"
        }
        fn type_url(&self) -> &'static str {
            "type.googleapis.com/cosmos.capability.v1beta1.GenesisState"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/cosmos.capability.v1beta1.GenesisState" , decoder : | buf : & [u8] | { let msg : GenesisState = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
