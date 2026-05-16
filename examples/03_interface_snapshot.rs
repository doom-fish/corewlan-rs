use corewlan::prelude::*;

fn main() -> corewlan::Result<()> {
    let client = WiFiClient::shared()?;
    match client.interface() {
        Some(interface) => {
            println!("interface: {:?}", interface.interface_name());
            println!("power: {}", interface.power_on());
            println!("mode: {:?}", interface.interface_mode());
            println!("country: {:?}", interface.country_code());
            println!("hardware: {:?}", interface.hardware_address());
        }
        None => println!("no default Wi-Fi interface"),
    }

    println!("✅ interface snapshot OK");
    Ok(())
}
