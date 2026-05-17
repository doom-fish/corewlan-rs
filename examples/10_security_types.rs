use corewlan::prelude::*;

fn main() {
    println!("error domain: {}", error_domain());
    println!("power notification: {}", power_did_change_notification());
    println!("event type raw: {}", EventType::ScanCacheUpdated.as_raw());
    println!("security raw: {}", Security::Wpa3Personal.as_raw());
    println!(
        "cipher flags: {:?}",
        CipherKeyFlags::UNICAST | CipherKeyFlags::TX
    );
    println!("merged empty scan count: {}", merge_networks(&[]).len());
    println!("✅ security types OK");
}
