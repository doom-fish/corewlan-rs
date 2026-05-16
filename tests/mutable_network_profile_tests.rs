use corewlan::{MutableNetworkProfile, Security};

#[test]
fn mutable_network_profile_setters_update_local_state() {
    let profile = MutableNetworkProfile::new().expect("mutable profile init should succeed");
    profile.set_ssid_data(Some(b"corewlan-rs"));
    profile.set_security(Security::Wpa3Personal);

    assert_eq!(profile.ssid(), Some("corewlan-rs".to_owned()));
    assert_eq!(profile.security(), Security::Wpa3Personal);
}
