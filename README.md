# corewlan

Safe Rust bindings for Apple's [CoreWLAN](https://developer.apple.com/documentation/corewlan) framework on macOS — inspect Wi-Fi interfaces, cached scan state, active link quality, and preferred network configuration.

> **Status:** experimental. v0.1 ships `CWWiFiClient`, `CWInterface`, `CWNetwork`, `CWChannel`, `CWConfiguration`, and `CWNetworkProfile`. The smoke example intentionally avoids active scans so it won't trigger location / Wi-Fi permission prompts.

Zero Swift bridge — this crate uses a small Objective-C shim compiled by `cc` and exposes a safe Rust surface on top.

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
    }

    Ok(())
}
```

## Smoke example

```bash
cargo run --example 01_smoke
```

Expected output (values vary by machine):

```text
interfaces: ["en0"]
default interface: en0
ssid: Some("Your Wi-Fi")
rssi: -58 dBm
transmit rate: 702.0 Mbps
✅ corewlan client + interface OK
```

## Notes

- `Interface::scan_for_networks_*` methods call the blocking `CoreWLAN` scan APIs.
- `Interface::cached_scan_results()` only returns the existing scan cache and does not initiate a scan.
- SSID, BSSID, and country-code values may be `None` if Location Services access is unavailable.
- `WiFiClient::start_monitoring_event` / `stop_monitoring_event` are included for parity with the public headers, but v0.1 does not yet expose Rust delegate callbacks.

## Roadmap

- [x] `WiFiClient::{shared, interface_names, interfaces, interface_with_name}`
- [x] `Interface` state queries + cached scan snapshot accessors
- [x] Blocking scan wrappers (`scan_for_networks_with_name`, `scan_for_networks_with_ssid`)
- [x] `Network`, `Channel`, `Configuration`, and `NetworkProfile` snapshots
- [x] Event registration helpers without delegate callbacks
- [ ] Rust event delegate bridge for `CWEventDelegate`
- [ ] Association / configuration commit helpers

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
