use corewlan::prelude::*;

fn main() -> corewlan::Result<()> {
    let client = WiFiClient::shared()?;
    let profile = client
        .interface()
        .and_then(|interface| interface.configuration())
        .and_then(|configuration| configuration.network_profiles().into_iter().next())
        .unwrap_or_else(NetworkProfile::default);

    println!("profile ssid: {:?}", profile.ssid());
    println!("profile security: {:?}", profile.security());
    println!("✅ network profile snapshot OK");
    Ok(())
}
