use corewlan::prelude::*;

struct NoopDelegate;

impl WiFiClientEventDelegate for NoopDelegate {}

fn main() -> corewlan::Result<()> {
    let client = WiFiClient::new()?;
    let registration = client.set_delegate(NoopDelegate)?;
    let replacement = client.set_delegate(NoopDelegate)?;
    println!("delegate installed: {}", client.has_delegate());

    client.start_monitoring_event(EventType::PowerDidChange)?;
    client.stop_monitoring_event(EventType::PowerDidChange)?;
    drop(registration);
    println!("delegate still installed after stale drop: {}", client.has_delegate());
    drop(replacement);

    println!("delegate installed after final drop: {}", client.has_delegate());
    println!("✅ wifi client delegate OK");
    Ok(())
}
