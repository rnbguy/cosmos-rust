/// InterfaceDescriptor describes an interface type to be used with
/// accepts_interface and implements_interface and declared by declare_interface.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceDescriptor {
    /// name is the name of the interface. It should be a short-name (without
    /// a period) such that the fully qualified name of the interface will be
    /// package.name, ex. for the package a.b and interface named C, the
    /// fully-qualified name will be a.b.C.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// description is a human-readable description of the interface and its
    /// purpose.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
/// ScalarDescriptor describes an scalar type to be used with
/// the scalar field option and declared by declare_scalar.
/// Scalars extend simple protobuf built-in types with additional
/// syntax and semantics, for instance to represent big integers.
/// Scalars should ideally define an encoding such that there is only one
/// valid syntactical representation for a given semantic meaning,
/// i.e. the encoding should be deterministic.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalarDescriptor {
    /// name is the name of the scalar. It should be a short-name (without
    /// a period) such that the fully qualified name of the scalar will be
    /// package.name, ex. for the package a.b and scalar named C, the
    /// fully-qualified name will be a.b.C.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// description is a human-readable description of the scalar and its
    /// encoding format. For instance a big integer or decimal scalar should
    /// specify precisely the expected encoding format.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// field_type is the type of field with which this scalar can be used.
    /// Scalars can be used with one and only one type of field so that
    /// encoding standards and simple and clear. Currently only string and
    /// bytes fields are supported for scalars.
    #[prost(enumeration = "ScalarType", repeated, tag = "3")]
    pub field_type: ::prost::alloc::vec::Vec<i32>,
}
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
pub enum ScalarType {
    Unspecified = 0,
    String = 1,
    Bytes = 2,
}
impl ScalarType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ScalarType::Unspecified => "SCALAR_TYPE_UNSPECIFIED",
            ScalarType::String => "SCALAR_TYPE_STRING",
            ScalarType::Bytes => "SCALAR_TYPE_BYTES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SCALAR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SCALAR_TYPE_STRING" => Some(Self::String),
            "SCALAR_TYPE_BYTES" => Some(Self::Bytes),
            _ => None,
        }
    }
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_INTERFACE_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos_proto.InterfaceDescriptor")]
    impl ::prost_wkt::MessageSerde for InterfaceDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos_proto"
        }
        fn message_name(&self) -> &'static str {
            "InterfaceDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos_proto.InterfaceDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos_proto.InterfaceDescriptor" , decoder : | buf : & [u8] | { let msg : InterfaceDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_SCALAR_DESCRIPTOR: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "/cosmos_proto.ScalarDescriptor")]
    impl ::prost_wkt::MessageSerde for ScalarDescriptor {
        fn package_name(&self) -> &'static str {
            "cosmos_proto"
        }
        fn message_name(&self) -> &'static str {
            "ScalarDescriptor"
        }
        fn type_url(&self) -> &'static str {
            "/cosmos_proto.ScalarDescriptor"
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
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "/cosmos_proto.ScalarDescriptor" , decoder : | buf : & [u8] | { let msg : ScalarDescriptor = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};