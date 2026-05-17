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
pub mod mutable_configuration;
pub mod mutable_network_profile;
pub mod network;
pub mod network_profile;
pub mod profile;
pub mod security;
pub mod types;

pub use channel::Channel;
pub use client::{DelegateRegistration, WiFiClient, WiFiClientEventDelegate};
pub use configuration::Configuration;
pub use error::{CoreWlanError, Result};
pub use interface::Interface;
pub use mutable_configuration::MutableConfiguration;
pub use mutable_network_profile::MutableNetworkProfile;
pub use network::Network;
pub use network_profile::NetworkProfile;
pub use security::{
    bssid_did_change_notification, copy_eap_identity_list, copy_wifi_eap_identity,
    country_code_did_change_notification, delete_wifi_eap_username_and_password,
    delete_wifi_password, error_domain, find_wifi_eap_username_and_password, find_wifi_password,
    link_did_change_notification, link_quality_did_change_notification,
    link_quality_notification_rssi_key, link_quality_notification_transmit_rate_key,
    merge_networks, mode_did_change_notification, power_did_change_notification,
    scan_cache_did_update_notification, set_wifi_eap_identity, set_wifi_eap_username_and_password,
    set_wifi_password, ssid_did_change_notification, Authorization, ChannelBand, ChannelWidth,
    CipherKeyFlags, ErrorCode, EventType, IbssModeSecurity, Identity, InterfaceMode,
    KeychainDomain, PhyMode, Security,
};

/// Common imports.
pub mod prelude {
    pub use crate::channel::Channel;
    pub use crate::client::{DelegateRegistration, WiFiClient, WiFiClientEventDelegate};
    pub use crate::configuration::Configuration;
    pub use crate::error::{CoreWlanError, Result};
    pub use crate::interface::Interface;
    pub use crate::mutable_configuration::MutableConfiguration;
    pub use crate::mutable_network_profile::MutableNetworkProfile;
    pub use crate::network::Network;
    pub use crate::network_profile::NetworkProfile;
    pub use crate::security::{
        bssid_did_change_notification, copy_eap_identity_list, copy_wifi_eap_identity,
        country_code_did_change_notification, delete_wifi_eap_username_and_password,
        delete_wifi_password, error_domain, find_wifi_eap_username_and_password,
        find_wifi_password, link_did_change_notification, link_quality_did_change_notification,
        link_quality_notification_rssi_key, link_quality_notification_transmit_rate_key,
        merge_networks, mode_did_change_notification, power_did_change_notification,
        scan_cache_did_update_notification, set_wifi_eap_identity,
        set_wifi_eap_username_and_password, set_wifi_password, ssid_did_change_notification,
        Authorization, ChannelBand, ChannelWidth, CipherKeyFlags, ErrorCode, EventType,
        IbssModeSecurity, Identity, InterfaceMode, KeychainDomain, PhyMode, Security,
    };
}
