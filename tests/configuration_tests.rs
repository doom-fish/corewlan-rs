use corewlan::Configuration;

#[test]
fn configuration_new_and_clone_are_equal() {
    let configuration = Configuration::new().expect("configuration init should succeed");
    let cloned = Configuration::from_configuration(&configuration)
        .expect("configuration copy should succeed");
    assert_eq!(configuration, cloned);
}
