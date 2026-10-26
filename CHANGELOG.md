# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.2] - 2026-10-26

### Fixed

- Update the test assertions for Clippy 1.99 on hosted runners.

## [0.3.1] - 2026-10-26

### Fixed

- Build the Swift bridge with the default SwiftPM build engine and use its reported product directory.

## [0.3.0] - 2026-09-24

### Security

- Passwords and usernames that contain a NUL byte are rejected with `CoreWlanError::InteriorNul` instead of being silently stripped (`Interface::associate_to_network`, `Interface::associate_to_enterprise_network`, `set_wifi_password`, `set_wifi_eap_username_and_password`).
- Credential C strings are wiped after the call, and the buffer that carries a keychain password back from Swift is wiped before it is freed.

### Changed

- **Breaking:** `Interface::ssid`, `ssid_data`, `bssid`, and `country_code` return `Result<Option<_>>`. They report `CoreWlanError::LocationNotAuthorized` when macOS withholds the value for lack of Location Services authorization; `Ok(None)` now means the interface is not associated (or, for the country code, powered off).
- **Breaking:** `find_wifi_password` returns `Option<SecretString>`, and `find_wifi_eap_username_and_password` returns the password as `SecretString`.
- `Interface::scan_for_networks_with_name*` reject names that contain NUL with `CoreWlanError::InteriorNul`, and `WiFiClient::interface_with_name` returns `None` for them.
- `rust-version` is now 1.82.

### Added

- `location_authorization()` and `LocationAuthorization`.
- `SecretString`, with redacted `Debug` and wipe-on-drop.
- `CoreWlanError::InteriorNul` and `CoreWlanError::LocationNotAuthorized`.

### Fixed

- Keychain lookups release their out-objects when the keychain call fails.
- `build.rs` no longer adds the toolchain's Swift 5.5 back-deployment directory (`usr/lib/swift-5.5/macosx`) to the link search path or the rpath. Its old `libswift_Concurrency.dylib` shadowed the SDK's `libswift_Concurrency.tbd` in every binary that depends on this crate, so linking failed next to a Swift bridge that uses newer concurrency APIs, such as apple-localauthentication's.

### Removed

- The uncompiled Objective-C shim `src/ffi/corewlan_shim.m`, which was still included in the published package.

## [0.2.1] - 2026-06-06

- Guarded the delegate release trampoline against panics unwinding across FFI, and added the coverage audit.

## [0.2.0] - 2026-05-16

- Replaced the Objective-C shim with a SwiftPM-built CoreWLAN bridge.
- Added safe wrappers for `CWMutableConfiguration` and `CWMutableNetworkProfile`.
- Added `CWInterface` mutators for channel, keys, association, enterprise association, and configuration commits.
- Added Rust delegate registration for `CWEventDelegate` notifications.
- Added typed `CWSecurity` helpers for error codes, cipher flags, notification constants, `CWMergeNetworks`, and CoreWLAN keychain utilities.
- Added numbered examples and integration tests covering every logical CoreWLAN area.

## [0.1.0] - 2026-05-16

- Initial release.
- Safe wrappers for `CWWiFiClient`, `CWInterface`, `CWNetwork`, `CWChannel`, `CWConfiguration`, and `CWNetworkProfile`.
- Non-Swift Objective-C shim for object lifetime management, scans, and collection conversion.
- Smoke example that only inspects the current interface state and does not trigger Wi-Fi scans.
