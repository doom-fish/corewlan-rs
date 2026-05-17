#![allow(dead_code)]

use corewlan::{Channel, Interface, Network, WiFiClient};
use std::sync::{Mutex, MutexGuard, OnceLock};

pub fn default_interface() -> Option<Interface> {
    WiFiClient::shared()
        .ok()
        .and_then(|client| client.interface())
}

pub fn first_channel() -> Option<Channel> {
    default_interface().and_then(|interface| {
        interface
            .wlan_channel()
            .or_else(|| interface.supported_wlan_channels().into_iter().next())
    })
}

pub fn first_network() -> Option<Network> {
    default_interface().and_then(|interface| interface.cached_scan_results().into_iter().next())
}

pub fn corewlan_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .expect("test lock poisoned")
}
