use super::Object;
use std::ffi::c_char;

extern "C" {
    pub fn cwrs_cw_error_domain() -> Object;
    pub fn cwrs_power_did_change_notification() -> Object;
    pub fn cwrs_ssid_did_change_notification() -> Object;
    pub fn cwrs_bssid_did_change_notification() -> Object;
    pub fn cwrs_link_did_change_notification() -> Object;
    pub fn cwrs_mode_did_change_notification() -> Object;
    pub fn cwrs_country_code_did_change_notification() -> Object;
    pub fn cwrs_scan_cache_did_update_notification() -> Object;
    pub fn cwrs_link_quality_did_change_notification() -> Object;
    pub fn cwrs_link_quality_notification_rssi_key() -> Object;
    pub fn cwrs_link_quality_notification_transmit_rate_key() -> Object;
    pub fn cwrs_merge_networks(networks: *const Object, count: usize) -> Object;
    pub fn cwrs_keychain_find_wifi_password(
        domain: isize,
        ssid_bytes: *const u8,
        ssid_len: usize,
        password_out: *mut Object,
    ) -> i32;
    pub fn cwrs_keychain_set_wifi_password(
        domain: isize,
        ssid_bytes: *const u8,
        ssid_len: usize,
        password: *const c_char,
    ) -> i32;
    pub fn cwrs_keychain_delete_wifi_password(
        domain: isize,
        ssid_bytes: *const u8,
        ssid_len: usize,
    ) -> i32;
    pub fn cwrs_keychain_find_wifi_eap_username_and_password(
        domain: isize,
        ssid_bytes: *const u8,
        ssid_len: usize,
        username_out: *mut Object,
        password_out: *mut Object,
    ) -> i32;
    pub fn cwrs_keychain_set_wifi_eap_username_and_password(
        domain: isize,
        ssid_bytes: *const u8,
        ssid_len: usize,
        username: *const c_char,
        password: *const c_char,
    ) -> i32;
    pub fn cwrs_keychain_delete_wifi_eap_username_and_password(
        domain: isize,
        ssid_bytes: *const u8,
        ssid_len: usize,
    ) -> i32;
    pub fn cwrs_keychain_copy_wifi_eap_identity(
        domain: isize,
        ssid_bytes: *const u8,
        ssid_len: usize,
        identity_out: *mut Object,
    ) -> i32;
    pub fn cwrs_keychain_set_wifi_eap_identity(
        domain: isize,
        ssid_bytes: *const u8,
        ssid_len: usize,
        identity: Object,
    ) -> i32;
    pub fn cwrs_keychain_copy_eap_identity_list(list_out: *mut Object) -> i32;
}
