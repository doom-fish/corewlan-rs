mod common;

use corewlan::Security;

#[test]
fn network_snapshot_and_equality() {
    if let Some(network) = common::first_network() {
        let clone = network.clone();
        assert_eq!(network, clone);
        let _ = network.ssid();
        let _ = network.bssid();
        let _ = network.wlan_channel();
        let _ = network.supports_security(Security::Wpa2Personal);
    }
}
