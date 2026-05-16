//! Raw FFI declarations for the bridged `CoreWLAN` surface.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

use std::ffi::c_void;

pub mod channel;
pub mod client;
pub mod configuration;
pub mod core;
pub mod interface;
pub mod mutable_configuration;
pub mod mutable_network_profile;
pub mod network;
pub mod network_profile;
pub mod security;

pub type Object = *mut c_void;

pub use self::{
    channel::*, client::*, configuration::*, core::*, interface::*,
    mutable_configuration::*, mutable_network_profile::*, network::*,
    network_profile::*, security::*,
};
