use corewlan::prelude::*;

fn main() -> corewlan::Result<()> {
    let client = WiFiClient::shared()?;
    let configuration = client
        .interface()
        .and_then(|interface| interface.configuration())
        .unwrap_or_else(Configuration::default);

    println!("preferred networks: {}", configuration.network_profiles().len());
    println!(
        "require admin for association: {}",
        configuration.require_administrator_for_association()
    );
    println!(
        "remember joined networks: {}",
        configuration.remember_joined_networks()
    );
    println!("✅ configuration snapshot OK");
    Ok(())
}
