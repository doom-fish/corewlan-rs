use corewlan::{
    bssid_did_change_notification, country_code_did_change_notification, error_domain,
    link_did_change_notification, link_quality_did_change_notification,
    link_quality_notification_rssi_key, link_quality_notification_transmit_rate_key,
    merge_networks, mode_did_change_notification, power_did_change_notification,
    scan_cache_did_update_notification, ssid_did_change_notification, CipherKeyFlags, ErrorCode,
    EventType, InterfaceMode, PhyMode, Security,
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
    assert!(!error_domain().is_empty());
    assert!(!power_did_change_notification().is_empty());
    assert!(!ssid_did_change_notification().is_empty());
    assert!(!bssid_did_change_notification().is_empty());
    assert!(!link_did_change_notification().is_empty());
    assert!(!mode_did_change_notification().is_empty());
    assert!(!country_code_did_change_notification().is_empty());
    assert!(!scan_cache_did_update_notification().is_empty());
    assert!(!link_quality_did_change_notification().is_empty());
    assert!(!link_quality_notification_rssi_key().is_empty());
    assert!(!link_quality_notification_transmit_rate_key().is_empty());
    assert_eq!(CipherKeyFlags::UNICAST.bits(), 1 << 1);
    assert!(merge_networks(&[]).is_empty());
}
