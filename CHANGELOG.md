# Changelog

## 0.2.0

- Replaced the Objective-C shim with a SwiftPM-built CoreWLAN bridge.
- Added safe wrappers for `CWMutableConfiguration` and `CWMutableNetworkProfile`.
- Added `CWInterface` mutators for channel, keys, association, enterprise association, and configuration commits.
- Added Rust delegate registration for `CWEventDelegate` notifications.
- Added typed `CWSecurity` helpers for error codes, cipher flags, notification constants, `CWMergeNetworks`, and CoreWLAN keychain utilities.
- Added numbered examples and integration tests covering every logical CoreWLAN area.

## 0.1.0

- Initial release.
- Safe wrappers for `CWWiFiClient`, `CWInterface`, `CWNetwork`, `CWChannel`, `CWConfiguration`, and `CWNetworkProfile`.
- Non-Swift Objective-C shim for object lifetime management, scans, and collection conversion.
- Smoke example that only inspects the current interface state and does not trigger Wi-Fi scans.
