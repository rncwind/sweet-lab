#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageA {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub value: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageB {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(float, tag = "2")]
    pub value: f32,
}
