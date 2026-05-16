use super::Object;

extern "C" {
    pub fn cwrs_configuration_new() -> Object;
    pub fn cwrs_configuration_with_configuration(configuration: Object) -> Object;
    pub fn cwrs_configuration_network_profiles(configuration: Object) -> Object;
    pub fn cwrs_configuration_require_admin_association(configuration: Object) -> bool;
    pub fn cwrs_configuration_require_admin_power(configuration: Object) -> bool;
    pub fn cwrs_configuration_require_admin_ibss_mode(configuration: Object) -> bool;
    pub fn cwrs_configuration_remember_joined_networks(configuration: Object) -> bool;
    pub fn cwrs_configuration_equal(lhs: Object, rhs: Object) -> bool;
}
