use corewlan::MutableConfiguration;

#[test]
fn mutable_configuration_setters_update_local_state() {
    let configuration =
        MutableConfiguration::new().expect("mutable configuration init should succeed");
    configuration.set_require_administrator_for_association(true);
    configuration.set_require_administrator_for_power(true);
    configuration.set_remember_joined_networks(false);

    assert!(configuration.require_administrator_for_association());
    assert!(configuration.require_administrator_for_power());
    assert!(!configuration.remember_joined_networks());
}
