use super::Object;

extern "C" {
    pub fn cwrs_network_profile_new() -> Object;
    pub fn cwrs_network_profile_with_network_profile(profile: Object) -> Object;
    pub fn cwrs_network_profile_ssid(profile: Object) -> Object;
    pub fn cwrs_network_profile_ssid_data(profile: Object) -> Object;
    pub fn cwrs_network_profile_security(profile: Object) -> isize;
    pub fn cwrs_network_profile_equal(lhs: Object, rhs: Object) -> bool;
}
