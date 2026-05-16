use corewlan::prelude::*;

fn main() -> corewlan::Result<()> {
    let client = WiFiClient::shared()?;
    let names = client.interface_names();
    println!("interfaces: {names:?}");

    if let Some(interface) = client.interface() {
        println!(
            "default interface: {}",
            interface
                .interface_name()
                .unwrap_or_else(|| "<unknown>".to_owned())
        );
        println!("ssid: {:?}", interface.ssid());
        println!("rssi: {} dBm", interface.rssi_value());
        println!("transmit rate: {:.1} Mbps", interface.transmit_rate());
    } else {
        println!("default interface: <none>");
    }

    println!("✅ corewlan smoke OK");
    Ok(())
}
