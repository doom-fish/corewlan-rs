//! Raw FFI declarations for the current public `CoreWLAN` headers.
//!
//! Non-Swift Objective-C shim compiled from `corewlan_shim.m`.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

use core::ffi::{c_char, c_void};

pub type Object = *mut c_void;

extern "C" {
    pub fn cwrs_retain(obj: Object) -> Object;
    pub fn cwrs_release(obj: Object);
    pub fn cwrs_free_buffer(buffer: *mut c_void);

    pub fn cwrs_string_copy_utf8(string: Object) -> *mut c_char;
    pub fn cwrs_data_copy_bytes(data: Object, len_out: *mut usize) -> *mut u8;

    pub fn cwrs_array_count(array: Object) -> usize;
    pub fn cwrs_array_object_at_index(array: Object, index: usize) -> Object;
    pub fn cwrs_ordered_set_count(set: Object) -> usize;
    pub fn cwrs_ordered_set_object_at_index(set: Object, index: usize) -> Object;
    pub fn cwrs_set_copy_all_objects(set: Object) -> Object;

    pub fn cwrs_error_code(error: Object) -> isize;
    pub fn cwrs_error_domain(error: Object) -> *mut c_char;
    pub fn cwrs_error_description(error: Object) -> *mut c_char;

    pub fn cwrs_wifi_client_shared() -> Object;
    pub fn cwrs_wifi_client_new() -> Object;
    pub fn cwrs_wifi_client_interface(client: Object) -> Object;
    pub fn cwrs_wifi_client_interface_with_name(client: Object, name: *const c_char) -> Object;
    pub fn cwrs_wifi_client_interfaces(client: Object) -> Object;
    pub fn cwrs_wifi_client_interface_names(client: Object) -> Object;
    pub fn cwrs_wifi_client_start_monitoring_event(
        client: Object,
        event_type: isize,
        error_out: *mut Object,
    ) -> bool;
    pub fn cwrs_wifi_client_stop_monitoring_event(
        client: Object,
        event_type: isize,
        error_out: *mut Object,
    ) -> bool;
    pub fn cwrs_wifi_client_stop_monitoring_all_events(
        client: Object,
        error_out: *mut Object,
    ) -> bool;

    pub fn cwrs_interface_name(interface: Object) -> Object;
    pub fn cwrs_interface_power_on(interface: Object) -> bool;
    pub fn cwrs_interface_supported_wlan_channels(interface: Object) -> Object;
    pub fn cwrs_interface_wlan_channel(interface: Object) -> Object;
    pub fn cwrs_interface_active_phy_mode(interface: Object) -> isize;
    pub fn cwrs_interface_ssid(interface: Object) -> Object;
    pub fn cwrs_interface_ssid_data(interface: Object) -> Object;
    pub fn cwrs_interface_bssid(interface: Object) -> Object;
    pub fn cwrs_interface_rssi_value(interface: Object) -> isize;
    pub fn cwrs_interface_noise_measurement(interface: Object) -> isize;
    pub fn cwrs_interface_security(interface: Object) -> isize;
    pub fn cwrs_interface_transmit_rate(interface: Object) -> f64;
    pub fn cwrs_interface_country_code(interface: Object) -> Object;
    pub fn cwrs_interface_mode(interface: Object) -> isize;
    pub fn cwrs_interface_transmit_power(interface: Object) -> isize;
    pub fn cwrs_interface_hardware_address(interface: Object) -> Object;
    pub fn cwrs_interface_service_active(interface: Object) -> bool;
    pub fn cwrs_interface_cached_scan_results(interface: Object) -> Object;
    pub fn cwrs_interface_configuration(interface: Object) -> Object;
    pub fn cwrs_interface_scan_for_networks_with_name(
        interface: Object,
        name: *const c_char,
        include_hidden: bool,
        has_include_hidden: bool,
        error_out: *mut Object,
    ) -> Object;
    pub fn cwrs_interface_scan_for_networks_with_ssid(
        interface: Object,
        ssid_bytes: *const u8,
        ssid_len: usize,
        include_hidden: bool,
        has_include_hidden: bool,
        error_out: *mut Object,
    ) -> Object;
    pub fn cwrs_interface_set_power(
        interface: Object,
        power_on: bool,
        error_out: *mut Object,
    ) -> bool;

    pub fn cwrs_network_ssid(network: Object) -> Object;
    pub fn cwrs_network_ssid_data(network: Object) -> Object;
    pub fn cwrs_network_bssid(network: Object) -> Object;
    pub fn cwrs_network_wlan_channel(network: Object) -> Object;
    pub fn cwrs_network_rssi_value(network: Object) -> isize;
    pub fn cwrs_network_noise_measurement(network: Object) -> isize;
    pub fn cwrs_network_information_element_data(network: Object) -> Object;
    pub fn cwrs_network_country_code(network: Object) -> Object;
    pub fn cwrs_network_beacon_interval(network: Object) -> isize;
    pub fn cwrs_network_ibss(network: Object) -> bool;
    pub fn cwrs_network_supports_security(network: Object, security: isize) -> bool;
    pub fn cwrs_network_supports_phy_mode(network: Object, phy_mode: isize) -> bool;

    pub fn cwrs_channel_number(channel: Object) -> isize;
    pub fn cwrs_channel_width(channel: Object) -> isize;
    pub fn cwrs_channel_band(channel: Object) -> isize;

    pub fn cwrs_configuration_network_profiles(configuration: Object) -> Object;
    pub fn cwrs_configuration_require_admin_association(configuration: Object) -> bool;
    pub fn cwrs_configuration_require_admin_power(configuration: Object) -> bool;
    pub fn cwrs_configuration_require_admin_ibss_mode(configuration: Object) -> bool;
    pub fn cwrs_configuration_remember_joined_networks(configuration: Object) -> bool;

    pub fn cwrs_network_profile_ssid(profile: Object) -> Object;
    pub fn cwrs_network_profile_ssid_data(profile: Object) -> Object;
    pub fn cwrs_network_profile_security(profile: Object) -> isize;
}
