#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's `CoreWLAN` framework on macOS.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod object;

pub mod channel;
pub mod client;
pub mod configuration;
pub mod error;
pub mod ffi;
pub mod interface;
pub mod network;
pub mod profile;
pub mod types;

pub use channel::Channel;
pub use client::WiFiClient;
pub use configuration::Configuration;
pub use error::{CoreWlanError, Result};
pub use interface::Interface;
pub use network::Network;
pub use profile::NetworkProfile;
pub use types::{ChannelBand, ChannelWidth, EventType, InterfaceMode, PhyMode, Security};

/// Common imports.
pub mod prelude {
    pub use crate::channel::Channel;
    pub use crate::client::WiFiClient;
    pub use crate::configuration::Configuration;
    pub use crate::error::{CoreWlanError, Result};
    pub use crate::interface::Interface;
    pub use crate::network::Network;
    pub use crate::profile::NetworkProfile;
    pub use crate::types::{
        ChannelBand, ChannelWidth, EventType, InterfaceMode, PhyMode, Security,
    };
}
