use corewlan::{
    bssid_did_change_notification, country_code_did_change_notification, error_domain,
    link_did_change_notification, link_quality_did_change_notification,
    link_quality_notification_rssi_key, link_quality_notification_transmit_rate_key,
    merge_networks, mode_did_change_notification, power_did_change_notification,
    scan_cache_did_update_notification, set_wifi_eap_username_and_password, set_wifi_password,
    ssid_did_change_notification, CipherKeyFlags, CoreWlanError, ErrorCode, EventType,
    InterfaceMode, KeychainDomain, PhyMode, Security,
};

#[test]
fn enum_round_trips_preserve_raw_values() {
    assert_eq!(PhyMode::Ieee80211ax.as_raw(), 6);
    assert_eq!(InterfaceMode::Station.as_raw(), 1);
    assert_eq!(Security::Wpa3Personal.as_raw(), 11);
    assert_eq!(EventType::ScanCacheUpdated.as_raw(), 8);
    assert_eq!(ErrorCode::OperationNotPermittedErr.as_raw(), -3930);
}

#[test]
fn constants_are_non_empty() {
    assert_ne!(error_domain(), "");
    assert_ne!(power_did_change_notification(), "");
    assert_ne!(ssid_did_change_notification(), "");
    assert_ne!(bssid_did_change_notification(), "");
    assert_ne!(link_did_change_notification(), "");
    assert_ne!(mode_did_change_notification(), "");
    assert_ne!(country_code_did_change_notification(), "");
    assert_ne!(scan_cache_did_update_notification(), "");
    assert_ne!(link_quality_did_change_notification(), "");
    assert_ne!(link_quality_notification_rssi_key(), "");
    assert_ne!(link_quality_notification_transmit_rate_key(), "");
    assert_eq!(CipherKeyFlags::UNICAST.bits(), 1 << 1);
    assert_eq!(merge_networks(&[]), [] as [corewlan::Network; 0]);
}

#[test]
fn set_wifi_password_rejects_interior_nul_before_touching_the_keychain() {
    assert_eq!(
        set_wifi_password(KeychainDomain::User, b"corewlan-rs-nul-test", "pass\0word"),
        Err(CoreWlanError::InteriorNul("password"))
    );
}

#[test]
fn set_wifi_eap_credentials_reject_interior_nul_before_touching_the_keychain() {
    assert_eq!(
        set_wifi_eap_username_and_password(
            KeychainDomain::User,
            b"corewlan-rs-nul-test",
            Some("user\0name"),
            Some("password"),
        ),
        Err(CoreWlanError::InteriorNul("username"))
    );
    assert_eq!(
        set_wifi_eap_username_and_password(
            KeychainDomain::User,
            b"corewlan-rs-nul-test",
            Some("username"),
            Some("pass\0word"),
        ),
        Err(CoreWlanError::InteriorNul("password"))
    );
}

#[test]
fn interior_nul_error_message_names_the_argument_only() {
    let message = CoreWlanError::InteriorNul("password").to_string();
    assert!(message.contains("password"));
    assert!(message.contains("NUL"));
}
