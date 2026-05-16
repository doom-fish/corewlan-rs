use corewlan::prelude::*;

fn main() -> corewlan::Result<()> {
    let configuration = MutableConfiguration::new()?;
    configuration.set_require_administrator_for_association(true);
    configuration.set_require_administrator_for_power(false);
    configuration.set_remember_joined_networks(true);

    println!(
        "mutable config flags: assoc={}, power={}, remember={}",
        configuration.require_administrator_for_association(),
        configuration.require_administrator_for_power(),
        configuration.remember_joined_networks()
    );
    println!("✅ mutable configuration OK");
    Ok(())
}
