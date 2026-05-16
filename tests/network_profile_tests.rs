use corewlan::NetworkProfile;

#[test]
fn network_profile_new_and_clone_are_equal() {
    let profile = NetworkProfile::new().expect("network profile init should succeed");
    let cloned = NetworkProfile::from_network_profile(&profile).expect("network profile copy should succeed");
    assert_eq!(profile, cloned);
}
