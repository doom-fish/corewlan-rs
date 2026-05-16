use super::Object;
use std::ffi::c_char;

extern "C" {
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
    pub fn cwrs_interface_set_wlan_channel(
        interface: Object,
        channel: Object,
        error_out: *mut Object,
    ) -> bool;
    pub fn cwrs_interface_set_pairwise_master_key(
        interface: Object,
        bytes: *const u8,
        length: usize,
        error_out: *mut Object,
    ) -> bool;
    pub fn cwrs_interface_set_wep_key(
        interface: Object,
        bytes: *const u8,
        length: usize,
        flags: usize,
        index: isize,
        error_out: *mut Object,
    ) -> bool;
    pub fn cwrs_interface_associate_to_network(
        interface: Object,
        network: Object,
        password: *const c_char,
        error_out: *mut Object,
    ) -> bool;
    pub fn cwrs_interface_disassociate(interface: Object);
    pub fn cwrs_interface_associate_to_enterprise_network(
        interface: Object,
        network: Object,
        identity: Object,
        username: *const c_char,
        password: *const c_char,
        error_out: *mut Object,
    ) -> bool;
    pub fn cwrs_interface_commit_configuration(
        interface: Object,
        configuration: Object,
        authorization: Object,
        error_out: *mut Object,
    ) -> bool;
}
