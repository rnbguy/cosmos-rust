/// Plan specifies information about a planned upgrade and when it should occur.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plan {
    /// Sets the name for the upgrade. This name will be used by the upgraded
    /// version of the software to apply any special "on-upgrade" commands during
    /// the first BeginBlock method after the upgrade is applied. It is also used
    /// to detect whether a software version can handle a given upgrade. If no
    /// upgrade handler with this name has been set in the software, it will be
    /// assumed that the software is out-of-date when the upgrade Time or Height is
    /// reached and the software will exit.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The time after which the upgrade must be performed.
    /// Leave set to its zero value to use a pre-defined Height instead.
    #[prost(message, optional, tag = "2")]
    pub time: ::core::option::Option<::prost_wkt_types::Timestamp>,
    /// The height at which the upgrade must be performed.
    /// Only used if Time is not set.
    #[prost(int64, tag = "3")]
    pub height: i64,
    /// Any application specific upgrade info to be included on-chain
    /// such as a git commit that validators could automatically upgrade to
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
}
/// SoftwareUpgradeProposal is a gov Content type for initiating a software
/// upgrade.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SoftwareUpgradeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub plan: ::core::option::Option<Plan>,
}
/// CancelSoftwareUpgradeProposal is a gov Content type for cancelling a software
/// upgrade.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelSoftwareUpgradeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_PLAN: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.upgrade.v1beta1.Plan")]
    impl ::prost_wkt::MessageSerde for Plan {
        fn package_name(&self) -> &'static str {
            "cosmos.upgrade.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "Plan"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.upgrade.v1beta1.Plan"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.upgrade.v1beta1.Plan" , decoder : | buf : & [u8] | { let msg : Plan = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_SOFTWARE_UPGRADE_PROPOSAL: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal")]
    impl ::prost_wkt::MessageSerde for SoftwareUpgradeProposal {
        fn package_name(&self) -> &'static str {
            "cosmos.upgrade.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "SoftwareUpgradeProposal"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal" , decoder : | buf : & [u8] | { let msg : SoftwareUpgradeProposal = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_CANCEL_SOFTWARE_UPGRADE_PROPOSAL: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(
        name = "/cosmos.upgrade.v1beta1.CancelSoftwareUpgradeProposal"
    )]
    impl ::prost_wkt::MessageSerde for CancelSoftwareUpgradeProposal {
        fn package_name(&self) -> &'static str {
            "cosmos.upgrade.v1beta1"
        }
        fn message_name(&self) -> &'static str {
            "CancelSoftwareUpgradeProposal"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos.upgrade.v1beta1.CancelSoftwareUpgradeProposal"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos.upgrade.v1beta1.CancelSoftwareUpgradeProposal" , decoder : | buf : & [u8] | { let msg : CancelSoftwareUpgradeProposal = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};
