use corewlan::prelude::*;

fn main() -> corewlan::Result<()> {
    let profile = MutableNetworkProfile::new()?;
    profile.set_ssid_data(Some(b"corewlan-rs"));
    profile.set_security(Security::Wpa3Personal);

    println!("mutable profile ssid: {:?}", profile.ssid());
    println!("mutable profile security: {:?}", profile.security());
    println!("✅ mutable network profile OK");
    Ok(())
}
