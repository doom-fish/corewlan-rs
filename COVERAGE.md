# CoreWLAN coverage audit

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped

## CoreWLAN.h umbrella

- ✅ `CoreWLANTypes.h` re-exported via `security.rs`
- ✅ `CoreWLANConstants.h` re-exported via `security.rs`
- ✅ `CoreWLANUtil.h` non-deprecated C helpers via `security.rs`
- ✅ `CWInterface.h`
- ✅ `CWWiFiClient.h`
- ✅ `CWNetwork.h`
- ✅ `CWConfiguration.h`
- ✅ `CWNetworkProfile.h`
- ✅ `CWChannel.h`

## CWChannel.h

- ✅ `channelNumber` → `Channel::channel_number`
- ✅ `channelWidth` → `Channel::channel_width`
- ✅ `channelBand` → `Channel::channel_band`
- ✅ `isEqualToChannel:` → `PartialEq for Channel`

## CWConfiguration.h

- ✅ `networkProfiles` → `Configuration::network_profiles`
- ✅ `requireAdministratorForAssociation` → `Configuration::require_administrator_for_association`
- ✅ `requireAdministratorForPower` → `Configuration::require_administrator_for_power`
- ✅ `requireAdministratorForIBSSMode` → `Configuration::require_administrator_for_ibss_mode`
- ✅ `rememberJoinedNetworks` → `Configuration::remember_joined_networks`
- ✅ `+configuration` / `-init` → `Configuration::new`
- ✅ `-initWithConfiguration:` / `+configurationWithConfiguration:` → `Configuration::from_configuration`
- ✅ `isEqualToConfiguration:` → `PartialEq for Configuration`
- ✅ `CWMutableConfiguration.networkProfiles` setter → `MutableConfiguration::set_network_profiles`
- ✅ `CWMutableConfiguration.requireAdministratorForAssociation` setter → `MutableConfiguration::set_require_administrator_for_association`
- ✅ `CWMutableConfiguration.requireAdministratorForPower` setter → `MutableConfiguration::set_require_administrator_for_power`
- ✅ `CWMutableConfiguration.requireAdministratorForIBSSMode` setter → `MutableConfiguration::set_require_administrator_for_ibss_mode`
- ✅ `CWMutableConfiguration.rememberJoinedNetworks` setter → `MutableConfiguration::set_remember_joined_networks`

## CWInterface.h

- ✅ `interfaceName` → `Interface::interface_name`
- ✅ `powerOn` → `Interface::power_on`
- ✅ `supportedWLANChannels` → `Interface::supported_wlan_channels`
- ✅ `wlanChannel` → `Interface::wlan_channel`
- ✅ `activePHYMode` → `Interface::active_phy_mode`
- ✅ `ssid` → `Interface::ssid`
- ✅ `ssidData` → `Interface::ssid_data`
- ✅ `bssid` → `Interface::bssid`
- ✅ `rssiValue` → `Interface::rssi_value`
- ✅ `noiseMeasurement` → `Interface::noise_measurement`
- ✅ `security` → `Interface::security`
- ✅ `transmitRate` → `Interface::transmit_rate`
- ✅ `countryCode` → `Interface::country_code`
- ✅ `interfaceMode` → `Interface::interface_mode`
- ✅ `transmitPower` → `Interface::transmit_power`
- ✅ `hardwareAddress` → `Interface::hardware_address`
- ✅ `serviceActive` → `Interface::service_active`
- ✅ `cachedScanResults` → `Interface::cached_scan_results`
- ✅ `configuration` → `Interface::configuration`
- ⏭️ `+interfaceNames` — deprecated in favor of `CWWiFiClient`
- ⏭️ `+interface` — deprecated in favor of `CWWiFiClient`
- ⏭️ `+interfaceWithName:` — deprecated in favor of `CWWiFiClient`
- ⏭️ `-initWithInterfaceName:` — deprecated in favor of `CWWiFiClient`
- ✅ `setPower:error:` → `Interface::set_power`
- ✅ `setWLANChannel:error:` → `Interface::set_wlan_channel`
- ✅ `setPairwiseMasterKey:error:` → `Interface::set_pairwise_master_key`
- ✅ `setWEPKey:flags:index:error:` → `Interface::set_wep_key`
- ✅ `scanForNetworksWithSSID:error:` → `Interface::scan_for_networks_with_ssid`
- ✅ `scanForNetworksWithSSID:includeHidden:error:` → `Interface::scan_for_networks_with_ssid_include_hidden`
- ✅ `scanForNetworksWithName:error:` → `Interface::scan_for_networks_with_name`
- ✅ `scanForNetworksWithName:includeHidden:error:` → `Interface::scan_for_networks_with_name_include_hidden`
- ✅ `associateToNetwork:password:error:` → `Interface::associate_to_network`
- ✅ `disassociate` → `Interface::disassociate`
- ✅ `associateToEnterpriseNetwork:identity:username:password:error:` → `Interface::associate_to_enterprise_network`
- ⏭️ `startIBSSModeWithSSID:security:channel:password:error:` — deprecated on macOS 11+
- ✅ `commitConfiguration:authorization:error:` → `Interface::commit_configuration`

## CWWiFiClient.h

- ✅ `delegate` property → `WiFiClient::set_delegate`, `WiFiClient::has_delegate`, `WiFiClient::clear_delegate`, `DelegateRegistration`
- ✅ `sharedWiFiClient` → `WiFiClient::shared`
- ✅ `init` → `WiFiClient::new`
- ✅ `interface` → `WiFiClient::interface`
- ✅ `interfaceNames` (instance) → `WiFiClient::interface_names`
- ⏭️ `+interfaceNames` — deprecated on macOS 13 in favor of the instance method
- ✅ `interfaceWithName:` → `WiFiClient::interface_with_name`
- ✅ `interfaces` → `WiFiClient::interfaces`
- ✅ `startMonitoringEventWithType:error:` → `WiFiClient::start_monitoring_event`
- ✅ `stopMonitoringEventWithType:error:` → `WiFiClient::stop_monitoring_event`
- ✅ `stopMonitoringAllEventsAndReturnError:` → `WiFiClient::stop_monitoring_all_events`
- ✅ `CWEventDelegate.clientConnectionInterrupted` → `WiFiClientEventDelegate::client_connection_interrupted`
- ✅ `CWEventDelegate.clientConnectionInvalidated` → `WiFiClientEventDelegate::client_connection_invalidated`
- ✅ `CWEventDelegate.powerStateDidChangeForWiFiInterfaceWithName:` → `WiFiClientEventDelegate::power_state_did_change`
- ✅ `CWEventDelegate.ssidDidChangeForWiFiInterfaceWithName:` → `WiFiClientEventDelegate::ssid_did_change`
- ✅ `CWEventDelegate.bssidDidChangeForWiFiInterfaceWithName:` → `WiFiClientEventDelegate::bssid_did_change`
- ✅ `CWEventDelegate.countryCodeDidChangeForWiFiInterfaceWithName:` → `WiFiClientEventDelegate::country_code_did_change`
- ✅ `CWEventDelegate.linkDidChangeForWiFiInterfaceWithName:` → `WiFiClientEventDelegate::link_did_change`
- ✅ `CWEventDelegate.linkQualityDidChangeForWiFiInterfaceWithName:rssi:transmitRate:` → `WiFiClientEventDelegate::link_quality_did_change`
- ✅ `CWEventDelegate.modeDidChangeForWiFiInterfaceWithName:` → `WiFiClientEventDelegate::mode_did_change`
- ✅ `CWEventDelegate.scanCacheUpdatedForWiFiInterfaceWithName:` → `WiFiClientEventDelegate::scan_cache_updated`

## CWNetwork.h

- ✅ `ssid` → `Network::ssid`
- ✅ `ssidData` → `Network::ssid_data`
- ✅ `bssid` → `Network::bssid`
- ✅ `wlanChannel` → `Network::wlan_channel`
- ✅ `rssiValue` → `Network::rssi_value`
- ✅ `noiseMeasurement` → `Network::noise_measurement`
- ✅ `informationElementData` → `Network::information_element_data`
- ✅ `countryCode` → `Network::country_code`
- ✅ `beaconInterval` → `Network::beacon_interval`
- ✅ `ibss` → `Network::ibss`
- ✅ `isEqualToNetwork:` → `PartialEq for Network`
- ✅ `supportsSecurity:` → `Network::supports_security`
- ✅ `supportsPHYMode:` → `Network::supports_phy_mode`

## CWNetworkProfile.h

- ✅ `ssid` → `NetworkProfile::ssid`
- ✅ `ssidData` → `NetworkProfile::ssid_data`
- ✅ `security` → `NetworkProfile::security`
- ✅ `+networkProfile` / `-init` → `NetworkProfile::new`
- ✅ `-initWithNetworkProfile:` / `+networkProfileWithNetworkProfile:` → `NetworkProfile::from_network_profile`
- ✅ `isEqualToNetworkProfile:` → `PartialEq for NetworkProfile`
- ✅ `CWMutableNetworkProfile.ssidData` setter → `MutableNetworkProfile::set_ssid_data`
- ✅ `CWMutableNetworkProfile.security` setter → `MutableNetworkProfile::set_security`

## CoreWLANTypes.h

- ✅ `CWErr` → `security::ErrorCode`
- ✅ `CWPHYMode` → `security::PhyMode`
- ✅ `CWInterfaceMode` → `security::InterfaceMode`
- ✅ `CWSecurity` → `security::Security`
- ✅ `CWIBSSModeSecurity` → `security::IbssModeSecurity`
- ✅ `CWChannelWidth` → `security::ChannelWidth`
- ✅ `CWChannelBand` → `security::ChannelBand`
- ✅ `CWCipherKeyFlags` → `security::CipherKeyFlags`
- ✅ `CWKeychainDomain` → `security::KeychainDomain`
- ✅ `CWEventType` → `security::EventType`

## CoreWLANConstants.h

- ✅ `CWErrorDomain` → `security::error_domain`
- ✅ `CWPowerDidChangeNotification` → `security::power_did_change_notification`
- ✅ `CWSSIDDidChangeNotification` → `security::ssid_did_change_notification`
- ✅ `CWBSSIDDidChangeNotification` → `security::bssid_did_change_notification`
- ✅ `CWLinkDidChangeNotification` → `security::link_did_change_notification`
- ✅ `CWModeDidChangeNotification` → `security::mode_did_change_notification`
- ✅ `CWCountryCodeDidChangeNotification` → `security::country_code_did_change_notification`
- ✅ `CWScanCacheDidUpdateNotification` → `security::scan_cache_did_update_notification`
- ✅ `CWLinkQualityDidChangeNotification` → `security::link_quality_did_change_notification`
- ✅ `CWLinkQualityNotificationRSSIKey` → `security::link_quality_notification_rssi_key`
- ✅ `CWLinkQualityNotificationTransmitRateKey` → `security::link_quality_notification_transmit_rate_key`

## CoreWLANUtil.h

- ✅ `CWKeychainFindWiFiPassword` → `security::find_wifi_password`
- ✅ `CWKeychainSetWiFiPassword` → `security::set_wifi_password`
- ✅ `CWKeychainDeleteWiFiPassword` → `security::delete_wifi_password`
- ✅ `CWKeychainFindWiFiEAPUsernameAndPassword` → `security::find_wifi_eap_username_and_password`
- ✅ `CWKeychainSetWiFiEAPUsernameAndPassword` → `security::set_wifi_eap_username_and_password`
- ✅ `CWKeychainDeleteWiFiEAPUsernameAndPassword` → `security::delete_wifi_eap_username_and_password`
- ✅ `CWKeychainCopyWiFiEAPIdentity` → `security::copy_wifi_eap_identity`
- ✅ `CWKeychainSetWiFiEAPIdentity` → `security::set_wifi_eap_identity`
- ✅ `CWKeychainCopyEAPIdentityList` → `security::copy_eap_identity_list`
- ⏭️ `CWKeychainCopyEAPUsernameAndPassword` — deprecated on macOS 10.9
- ⏭️ `CWKeychainSetEAPUsernameAndPassword` — deprecated on macOS 10.9
- ⏭️ `CWKeychainDeleteEAPUsernameAndPassword` — deprecated on macOS 10.9
- ⏭️ `CWKeychainCopyEAPIdentity` — deprecated on macOS 10.9
- ⏭️ `CWKeychainSetEAPIdentity` — deprecated on macOS 10.9
- ⏭️ `CWKeychainSetPassword` — deprecated on macOS 10.9
- ⏭️ `CWKeychainCopyPassword` — deprecated on macOS 10.9
- ⏭️ `CWKeychainDeletePassword` — deprecated on macOS 10.9
- ✅ `CWMergeNetworks` → `security::merge_networks`
