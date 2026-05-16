use super::Object;

extern "C" {
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
    pub fn cwrs_network_equal(lhs: Object, rhs: Object) -> bool;
}
