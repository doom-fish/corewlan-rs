use super::Object;
use std::ffi::{c_char, c_void};

extern "C" {
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
    pub fn cwrs_wifi_client_has_delegate(client: Object) -> bool;
    pub fn cwrs_wifi_client_set_delegate(
        client: Object,
        client_connection_interrupted: Option<extern "C" fn(*mut c_void)>,
        client_connection_invalidated: Option<extern "C" fn(*mut c_void)>,
        power_state_did_change: Option<extern "C" fn(*mut c_void, *const c_char)>,
        ssid_did_change: Option<extern "C" fn(*mut c_void, *const c_char)>,
        bssid_did_change: Option<extern "C" fn(*mut c_void, *const c_char)>,
        country_code_did_change: Option<extern "C" fn(*mut c_void, *const c_char)>,
        link_did_change: Option<extern "C" fn(*mut c_void, *const c_char)>,
        link_quality_did_change: Option<extern "C" fn(*mut c_void, *const c_char, isize, f64)>,
        mode_did_change: Option<extern "C" fn(*mut c_void, *const c_char)>,
        scan_cache_updated: Option<extern "C" fn(*mut c_void, *const c_char)>,
        context: *mut c_void,
        release_context: Option<extern "C" fn(*mut c_void)>,
    ) -> bool;
    pub fn cwrs_wifi_client_clear_delegate(client: Object, context: *mut c_void);
}
