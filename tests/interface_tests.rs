mod common;

use corewlan::{location_authorization, CoreWlanError, InterfaceMode, LocationAuthorization};

fn is_mac_address(value: &str) -> bool {
    let octets = value.split(':').collect::<Vec<_>>();
    octets.len() == 6
        && octets
            .iter()
            .all(|octet| octet.len() == 2 && octet.chars().all(|c| c.is_ascii_hexdigit()))
}

#[test]
fn interface_read_only_snapshot_smoke() {
    if let Some(interface) = common::default_interface() {
        let name = interface
            .interface_name()
            .expect("the default interface has a BSD name");
        assert_ne!(name, "");
        if let Some(address) = interface.hardware_address() {
            assert!(
                is_mac_address(&address),
                "unexpected hardware address format"
            );
        }
        if !interface.power_on() {
            assert_eq!(interface.interface_mode(), InterfaceMode::None);
        }
        if interface.interface_mode() == InterfaceMode::None {
            assert_eq!(interface.rssi_value(), 0);
        }
    }
}

#[test]
fn location_authorization_reports_a_core_location_status() {
    assert!(!matches!(
        location_authorization(),
        LocationAuthorization::Unknown(_)
    ));
}

#[test]
fn location_gated_reads_distinguish_missing_authorization() {
    if let Some(interface) = common::default_interface() {
        let associated = interface.interface_mode() != InterfaceMode::None;
        let authorized = location_authorization().is_authorized();

        for result in [
            interface.ssid().map(|ssid| ssid.map(String::into_bytes)),
            interface.ssid_data(),
            interface.bssid().map(|bssid| bssid.map(String::into_bytes)),
        ] {
            match result {
                Ok(Some(value)) => assert!(associated && !value.is_empty()),
                Ok(None) => assert!(!associated || authorized),
                Err(CoreWlanError::LocationNotAuthorized(status)) => {
                    assert!(associated);
                    assert!(!status.is_authorized());
                }
                Err(other) => panic!("unexpected error: {other}"),
            }
        }

        if let Ok(Some(bssid)) = interface.bssid() {
            assert!(is_mac_address(&bssid));
        }

        match interface.country_code() {
            Ok(Some(code)) => assert!(interface.power_on() && !code.is_empty()),
            Ok(None) => assert!(!interface.power_on() || authorized),
            Err(CoreWlanError::LocationNotAuthorized(status)) => {
                assert!(interface.power_on());
                assert!(!status.is_authorized());
            }
            Err(other) => panic!("unexpected error: {other}"),
        }
    }
}

#[test]
#[ignore = "mutates Wi-Fi state or requires administrator privileges"]
fn interface_mutating_apis_compile_and_dispatch() {
    if let Some(interface) = common::default_interface() {
        let _ = interface.set_power(interface.power_on());
        if let Some(channel) = interface.wlan_channel() {
            let _ = interface.set_wlan_channel(&channel);
        }
        let _ = interface.set_pairwise_master_key(None);
        let _ = interface.set_wep_key(None, corewlan::CipherKeyFlags::NONE, 1);
        if let Some(configuration) = interface.configuration() {
            let _ = interface.commit_configuration(&configuration, None);
        }
    }
}

#[test]
fn scan_by_name_rejects_interior_nul_without_scanning() {
    if let Some(interface) = common::default_interface() {
        assert_eq!(
            interface
                .scan_for_networks_with_name(Some("corewlan\0rs"))
                .map(|networks| networks.len()),
            Err(CoreWlanError::InteriorNul("name"))
        );
        assert_eq!(
            interface
                .scan_for_networks_with_name_include_hidden(Some("corewlan\0rs"), true)
                .map(|networks| networks.len()),
            Err(CoreWlanError::InteriorNul("name"))
        );
    }
}
