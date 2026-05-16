use corewlan::prelude::*;

fn main() -> corewlan::Result<()> {
    let client = WiFiClient::shared()?;
    let maybe_network = client
        .interface()
        .and_then(|interface| interface.cached_scan_results().into_iter().next());

    match maybe_network {
        Some(network) => {
            println!("network ssid: {:?}", network.ssid());
            println!("network bssid: {:?}", network.bssid());
            println!("network rssi: {}", network.rssi_value());
            println!("supports WPA2: {}", network.supports_security(Security::Wpa2Personal));
        }
        None => println!("no cached scan results available"),
    }

    println!("✅ network snapshot OK");
    Ok(())
}
