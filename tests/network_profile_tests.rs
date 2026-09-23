use corewlan::{MutableNetworkProfile, NetworkProfile, Security};

#[test]
fn network_profile_copy_equals_its_populated_source() {
    let source = MutableNetworkProfile::new().expect("mutable profile init should succeed");
    source.set_ssid_data(Some(b"corewlan-rs"));
    source.set_security(Security::Wpa3Personal);
    let copy =
        NetworkProfile::from_network_profile(&source).expect("network profile copy should succeed");
    assert_eq!(copy.ssid(), Some("corewlan-rs".to_owned()));
    assert_eq!(copy.security(), Security::Wpa3Personal);
    assert_eq!(*source, copy);
}

#[test]
fn network_profile_equals_its_clone() {
    let profile = NetworkProfile::new().expect("network profile init should succeed");
    let clone = profile.clone();
    assert_eq!(profile, clone);
}
