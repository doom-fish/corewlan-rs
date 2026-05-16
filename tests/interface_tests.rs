mod common;

#[test]
fn interface_read_only_snapshot_smoke() {
    if let Some(interface) = common::default_interface() {
        let _ = interface.interface_name();
        let _ = interface.power_on();
        let _ = interface.interface_mode();
        let _ = interface.hardware_address();
        let _ = interface.configuration();
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
