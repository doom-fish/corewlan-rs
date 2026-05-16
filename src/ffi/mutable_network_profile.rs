use super::Object;

extern "C" {
    pub fn cwrs_mutable_network_profile_new() -> Object;
    pub fn cwrs_mutable_network_profile_with_network_profile(profile: Object) -> Object;
    pub fn cwrs_mutable_network_profile_set_ssid_data(
        profile: Object,
        bytes: *const u8,
        length: usize,
    );
    pub fn cwrs_mutable_network_profile_set_security(profile: Object, security: isize);
}
