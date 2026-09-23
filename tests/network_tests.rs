mod common;

use corewlan::Security;

#[test]
fn network_snapshot_and_equality() {
    if let Some(network) = common::first_network() {
        let clone = network.clone();
        assert_eq!(network, clone);
        if let Some(ssid) = network.ssid_data() {
            assert!(ssid.len() <= 32);
        }
        if let Some(bssid) = network.bssid() {
            assert_eq!(bssid.split(':').count(), 6);
        }
        if let Some(channel) = network.wlan_channel() {
            assert!(channel.channel_number() > 0);
        }
        assert_eq!(
            clone.supports_security(Security::Wpa2Personal),
            network.supports_security(Security::Wpa2Personal)
        );
    }
}
