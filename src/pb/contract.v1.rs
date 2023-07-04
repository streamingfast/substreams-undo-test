// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub approvals: ::prost::alloc::vec::Vec<Approval>,
    #[prost(message, repeated, tag="2")]
    pub approval_for_alls: ::prost::alloc::vec::Vec<ApprovalForAll>,
    #[prost(message, repeated, tag="3")]
    pub ownership_transferreds: ::prost::alloc::vec::Vec<OwnershipTransferred>,
    #[prost(message, repeated, tag="4")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Approval {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub log_index: u32,
    #[prost(bytes="vec", tag="3")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub approved: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApprovalForAll {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub log_index: u32,
    #[prost(bytes="vec", tag="3")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub operator: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="5")]
    pub approved: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnershipTransferred {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub log_index: u32,
    #[prost(bytes="vec", tag="3")]
    pub previous_owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub log_index: u32,
    #[prost(bytes="vec", tag="3")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInfo {
    #[prost(uint64, tag="1")]
    pub block_number: u64,
    #[prost(string, tag="2")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub parent_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInfos {
    #[prost(message, repeated, tag="1")]
    pub block_infos: ::prost::alloc::vec::Vec<BlockInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashedBlockInfo {
    #[prost(uint64, tag="1")]
    pub block_number: u64,
    #[prost(string, tag="2")]
    pub hash: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
