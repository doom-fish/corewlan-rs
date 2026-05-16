# corewlan

Safe Rust bindings for Apple's [CoreWLAN](https://developer.apple.com/documentation/corewlan) framework on macOS.

> **Status:** v0.2 ships the full class-based CoreWLAN surface across `CWWiFiClient`, `CWInterface`, `CWChannel`, `CWNetwork`, `CWConfiguration`, `CWMutableConfiguration`, `CWNetworkProfile`, `CWMutableNetworkProfile`, and `CWSecurity`, backed by a Swift bridge built with SwiftPM.

## Highlights

- Shared and ephemeral `CWWiFiClient` access.
- Read-only `CWInterface`, `CWNetwork`, `CWChannel`, `CWConfiguration`, and `CWNetworkProfile` snapshots.
- Mutable configuration/profile builders via `CWMutableConfiguration` and `CWMutableNetworkProfile`.
- Interface mutators for power, channel, keys, association, enterprise association, and configuration commits.
- Rust delegate registration for `CWEventDelegate` notifications.
- Typed wrappers for `CoreWLAN` enums, cipher flags, keychain utilities, error codes, and notification constants.

## Quick start

```rust,no_run
use corewlan::prelude::*;

fn main() -> Result<()> {
    let client = WiFiClient::shared()?;

    for name in client.interface_names() {
        println!("wifi interface: {name}");
    }

    if let Some(interface) = client.interface() {
        println!("ssid = {:?}", interface.ssid());
        println!("rssi = {} dBm", interface.rssi_value());
        println!("rate = {:.1} Mbps", interface.transmit_rate());

        if let Some(configuration) = interface.configuration() {
            println!("preferred networks = {}", configuration.network_profiles().len());
        }
    }

    Ok(())
}
```

## Examples

```bash
cargo run --example 01_smoke
cargo run --example 02_wifi_client_events
cargo run --example 07_mutable_configuration
```

The crate ships one numbered example per logical `CoreWLAN` area; see `examples/` for the full list.

## Notes

- `Interface::scan_for_networks_*` wrappers call the blocking `CoreWLAN` scan APIs.
- `Interface::cached_scan_results()` only returns the existing scan cache and does not initiate a scan.
- SSID, BSSID, and country-code values may be unavailable without Location Services authorization.
- Association, power, key, and configuration commit APIs may require administrator privileges or Wi-Fi entitlements; the corresponding tests are marked `#[ignore]`.

## Coverage and verification

- API audit: [`COVERAGE.md`](COVERAGE.md)
- Validation command set:
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test`
  - `for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done`

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
