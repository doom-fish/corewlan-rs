# corewlan

Safe Rust bindings for Apple's [CoreWLAN](https://developer.apple.com/documentation/corewlan) framework on macOS.

> **Status:** v0.3 ships the full class-based CoreWLAN surface across `CWWiFiClient`, `CWInterface`, `CWChannel`, `CWNetwork`, `CWConfiguration`, `CWMutableConfiguration`, `CWNetworkProfile`, `CWMutableNetworkProfile`, and `CWSecurity`, backed by a Swift bridge built with SwiftPM.

## Installation

```toml
[dependencies]
corewlan = "0.3"
```

## Requirements

- macOS 10.13 or later (the Swift bridge's deployment target).
- Xcode or the Command Line Tools, so `build.rs` can run `swift build`.

## Highlights

- Shared and ephemeral `CWWiFiClient` access.
- Read-only `CWInterface`, `CWNetwork`, `CWChannel`, `CWConfiguration`, and `CWNetworkProfile` snapshots.
- Mutable configuration/profile builders via `CWMutableConfiguration` and `CWMutableNetworkProfile`.
- Interface mutators for power, channel, keys, association, enterprise association, and configuration commits.
- Rust delegate registration for `CWEventDelegate` notifications.
- Typed wrappers for `CoreWLAN` enums, cipher flags, keychain utilities, error codes, and notification constants.
- `location_authorization()` reports the Location Services status that gates SSID, BSSID, and country-code reads.
- Credentials are checked for NUL bytes, passed through buffers that are wiped after the call, and returned as `SecretString`.

## Quick start

```rust,no_run
use corewlan::prelude::*;

fn main() -> Result<()> {
    let client = WiFiClient::shared()?;

    for name in client.interface_names() {
        println!("wifi interface: {name}");
    }

    if let Some(interface) = client.interface() {
        match interface.ssid() {
            Ok(ssid) => println!("ssid = {ssid:?}"),
            Err(CoreWlanError::LocationNotAuthorized(status)) => {
                println!("ssid needs Location Services authorization ({status:?})");
            }
            Err(error) => return Err(error),
        }
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

## Location Services

macOS withholds the SSID, BSSID, and country code unless Location Services is enabled and the calling app is authorized. `Interface::ssid`, `ssid_data`, `bssid`, and `country_code` therefore return `Result<Option<_>>`:

- `Ok(Some(value))`: the value is available.
- `Ok(None)`: the interface is not participating in a network (for `country_code`: the interface is off), or `CoreWLAN` has no value even though access is authorized.
- `Err(CoreWlanError::LocationNotAuthorized(status))`: the interface is associated (for `country_code`: powered on) but the process has no Location Services authorization.

`location_authorization()` returns the current `CoreLocation` status and never prompts. This crate does not request authorization: an app that needs these values must request it through `CoreLocation` and declare `NSLocationWhenInUseUsageDescription` (`NSLocationUsageDescription` on older macOS) in its `Info.plist`. Scan results are affected too: without authorization `Network::bssid()` is `None`.

## Credentials

- Passwords and usernames that contain a NUL byte are rejected with `CoreWlanError::InteriorNul` before `CoreWLAN` is called; they are never truncated or stripped.
- Credential strings are copied into buffers that are wiped once the call returns.
- `find_wifi_password` and `find_wifi_eap_username_and_password` return passwords as `SecretString`: `Debug` prints `<redacted>`, the buffer is wiped on drop, and `expose()` borrows the value.
- Copies that Foundation and `CoreWLAN` make internally are outside this crate's control.

## Notes

- `Interface::scan_for_networks_*` wrappers call the blocking `CoreWLAN` scan APIs.
- `Interface::cached_scan_results()` only returns the existing scan cache and does not initiate a scan.
- Association, power, key, and configuration commit APIs may require administrator privileges or Wi-Fi entitlements; the corresponding tests are marked `#[ignore]`.

## Coverage and verification

- API audit: [`COVERAGE.md`](COVERAGE.md)
- Validation command set:
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test`
  - `for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done`

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
