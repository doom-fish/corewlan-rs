use super::Object;

extern "C" {
    pub fn cwrs_mutable_configuration_new() -> Object;
    pub fn cwrs_mutable_configuration_with_configuration(configuration: Object) -> Object;
    pub fn cwrs_mutable_configuration_set_network_profiles(
        configuration: Object,
        profiles: *const Object,
        count: usize,
    );
    pub fn cwrs_mutable_configuration_set_require_admin_association(
        configuration: Object,
        value: bool,
    );
    pub fn cwrs_mutable_configuration_set_require_admin_power(configuration: Object, value: bool);
    pub fn cwrs_mutable_configuration_set_require_admin_ibss_mode(
        configuration: Object,
        value: bool,
    );
    pub fn cwrs_mutable_configuration_set_remember_joined_networks(
        configuration: Object,
        value: bool,
    );
}
