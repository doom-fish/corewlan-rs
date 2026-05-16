use corewlan::prelude::*;

fn main() -> corewlan::Result<()> {
    let client = WiFiClient::shared()?;
    let maybe_channel = client.interface().and_then(|interface| {
        interface
            .wlan_channel()
            .or_else(|| interface.supported_wlan_channels().into_iter().next())
    });

    match maybe_channel {
        Some(channel) => {
            println!("channel number: {}", channel.channel_number());
            println!("channel width: {:?}", channel.channel_width());
            println!("channel band: {:?}", channel.channel_band());
        }
        None => println!("no current channel available"),
    }

    println!("✅ channel snapshot OK");
    Ok(())
}
