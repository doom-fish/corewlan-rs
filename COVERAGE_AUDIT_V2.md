# corewlan-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 147
VERIFIED: 121
GAPS: 0
EXEMPT: 26
COVERAGE_PCT: 100.00

Enumeration derived by parsing CoreWLAN framework headers in MacOSX26.2.sdk: CoreWLANConstants.h (11 FOUNDATION_EXTERN constants), CoreWLANTypes.h (10 typedef enums/options), CoreWLANUtil.h (18 C functions), plus Objective-C interfaces and their properties/methods from CWChannel.h, CWConfiguration.h, CWInterface.h, CWNetwork.h, CWNetworkProfile.h, and CWWiFiClient.h. All 26 exempts were re-verified against SDK 26.2 headers for deprecation attributes. Zero gaps confirmed — all non-deprecated public symbols have safe-wrapper equivalents in the crate.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `CWErr` | typedef | `CoreWLANTypes.h` | `security::ErrorCode` |
| `CWPHYMode` | typedef | `CoreWLANTypes.h` | `security::PhyMode` |
| `CWInterfaceMode` | typedef | `CoreWLANTypes.h` | `security::InterfaceMode` |
| `CWSecurity` | typedef | `CoreWLANTypes.h` | `security::Security` |
| `CWChannelWidth` | typedef | `CoreWLANTypes.h` | `security::ChannelWidth` |
| `CWChannelBand` | typedef | `CoreWLANTypes.h` | `security::ChannelBand` |
| `CWCipherKeyFlags` | typedef | `CoreWLANTypes.h` | `security::CipherKeyFlags` |
| `CWKeychainDomain` | typedef | `CoreWLANTypes.h` | `security::KeychainDomain` |
| `CWEventType` | typedef | `CoreWLANTypes.h` | `security::EventType` |
| `CWErrorDomain` | constant | `CoreWLANConstants.h` | `security::error_domain` |
| `CWKeychainFindWiFiPassword` | function | `CoreWLANUtil.h` | `security::find_wifi_password` |
| `CWKeychainSetWiFiPassword` | function | `CoreWLANUtil.h` | `security::set_wifi_password` |
| `CWKeychainDeleteWiFiPassword` | function | `CoreWLANUtil.h` | `security::delete_wifi_password` |
| `CWKeychainFindWiFiEAPUsernameAndPassword` | function | `CoreWLANUtil.h` | `security::find_wifi_eap_username_and_password` |
| `CWKeychainSetWiFiEAPUsernameAndPassword` | function | `CoreWLANUtil.h` | `security::set_wifi_eap_username_and_password` |
| `CWKeychainDeleteWiFiEAPUsernameAndPassword` | function | `CoreWLANUtil.h` | `security::delete_wifi_eap_username_and_password` |
| `CWKeychainCopyWiFiEAPIdentity` | function | `CoreWLANUtil.h` | `security::copy_wifi_eap_identity` |
| `CWKeychainSetWiFiEAPIdentity` | function | `CoreWLANUtil.h` | `security::set_wifi_eap_identity` |
| `CWKeychainCopyEAPIdentityList` | function | `CoreWLANUtil.h` | `security::copy_eap_identity_list` |
| `CWMergeNetworks` | function | `CoreWLANUtil.h` | `security::merge_networks` |
| `CWChannel` | interface | `CWChannel.h` | `Channel` |
| `CWChannel.channelNumber` | property | `CWChannel.h` | `Channel::channel_number` |
| `CWChannel.channelWidth` | property | `CWChannel.h` | `Channel::channel_width` |
| `CWChannel.channelBand` | property | `CWChannel.h` | `Channel::channel_band` |
| `CWChannel.isEqualToChannel:` | method | `CWChannel.h` | `PartialEq for Channel` |
| `CWConfiguration` | interface | `CWConfiguration.h` | `Configuration` |
| `CWConfiguration.networkProfiles` | property | `CWConfiguration.h` | `Configuration::network_profiles` |
| `CWConfiguration.requireAdministratorForAssociation` | property | `CWConfiguration.h` | `Configuration::require_administrator_for_association` |
| `CWConfiguration.requireAdministratorForPower` | property | `CWConfiguration.h` | `Configuration::require_administrator_for_power` |
| `CWConfiguration.requireAdministratorForIBSSMode` | property | `CWConfiguration.h` | `Configuration::require_administrator_for_ibss_mode` |
| `CWConfiguration.rememberJoinedNetworks` | property | `CWConfiguration.h` | `Configuration::remember_joined_networks` |
| `CWConfiguration.configuration` | class method | `CWConfiguration.h` | `Configuration::new` |
| `CWConfiguration.init` | method | `CWConfiguration.h` | `Configuration::new` |
| `CWConfiguration.initWithConfiguration:` | method | `CWConfiguration.h` | `Configuration::from_configuration` |
| `CWConfiguration.configurationWithConfiguration:` | class method | `CWConfiguration.h` | `Configuration::from_configuration` |
| `CWConfiguration.isEqualToConfiguration:` | method | `CWConfiguration.h` | `PartialEq for Configuration` |
| `CWMutableConfiguration` | interface | `CWConfiguration.h` | `MutableConfiguration` |
| `CWMutableConfiguration.networkProfiles` | property | `CWConfiguration.h` | `MutableConfiguration::set_network_profiles (getter via Deref<Target = Configuration>)` |
| `CWMutableConfiguration.requireAdministratorForAssociation` | property | `CWConfiguration.h` | `MutableConfiguration::set_require_administrator_for_association (getter via Deref<Target = Configuration>)` |
| `CWMutableConfiguration.requireAdministratorForPower` | property | `CWConfiguration.h` | `MutableConfiguration::set_require_administrator_for_power (getter via Deref<Target = Configuration>)` |
| `CWMutableConfiguration.rememberJoinedNetworks` | property | `CWConfiguration.h` | `MutableConfiguration::set_remember_joined_networks (getter via Deref<Target = Configuration>)` |
| `CWNetworkProfile` | interface | `CWNetworkProfile.h` | `NetworkProfile` |
| `CWNetworkProfile.ssid` | property | `CWNetworkProfile.h` | `NetworkProfile::ssid` |
| `CWNetworkProfile.ssidData` | property | `CWNetworkProfile.h` | `NetworkProfile::ssid_data` |
| `CWNetworkProfile.security` | property | `CWNetworkProfile.h` | `NetworkProfile::security` |
| `CWNetworkProfile.networkProfile` | class method | `CWNetworkProfile.h` | `NetworkProfile::new` |
| `CWNetworkProfile.init` | method | `CWNetworkProfile.h` | `NetworkProfile::new` |
| `CWNetworkProfile.initWithNetworkProfile:` | method | `CWNetworkProfile.h` | `NetworkProfile::from_network_profile` |
| `CWNetworkProfile.networkProfileWithNetworkProfile:` | class method | `CWNetworkProfile.h` | `NetworkProfile::from_network_profile` |
| `CWNetworkProfile.isEqualToNetworkProfile:` | method | `CWNetworkProfile.h` | `PartialEq for NetworkProfile` |
| `CWMutableNetworkProfile` | interface | `CWNetworkProfile.h` | `MutableNetworkProfile` |
| `CWMutableNetworkProfile.ssidData` | property | `CWNetworkProfile.h` | `MutableNetworkProfile::set_ssid_data (getter via Deref<Target = NetworkProfile>)` |
| `CWMutableNetworkProfile.security` | property | `CWNetworkProfile.h` | `MutableNetworkProfile::set_security (getter via Deref<Target = NetworkProfile>)` |
| `CWNetwork` | interface | `CWNetwork.h` | `Network` |
| `CWNetwork.ssid` | property | `CWNetwork.h` | `Network::ssid` |
| `CWNetwork.ssidData` | property | `CWNetwork.h` | `Network::ssid_data` |
| `CWNetwork.bssid` | property | `CWNetwork.h` | `Network::bssid` |
| `CWNetwork.wlanChannel` | property | `CWNetwork.h` | `Network::wlan_channel` |
| `CWNetwork.rssiValue` | property | `CWNetwork.h` | `Network::rssi_value` |
| `CWNetwork.noiseMeasurement` | property | `CWNetwork.h` | `Network::noise_measurement` |
| `CWNetwork.informationElementData` | property | `CWNetwork.h` | `Network::information_element_data` |
| `CWNetwork.countryCode` | property | `CWNetwork.h` | `Network::country_code` |
| `CWNetwork.beaconInterval` | property | `CWNetwork.h` | `Network::beacon_interval` |
| `CWNetwork.ibss` | property | `CWNetwork.h` | `Network::ibss` |
| `CWNetwork.isEqualToNetwork:` | method | `CWNetwork.h` | `PartialEq for Network` |
| `CWNetwork.supportsSecurity:` | method | `CWNetwork.h` | `Network::supports_security` |
| `CWNetwork.supportsPHYMode:` | method | `CWNetwork.h` | `Network::supports_phy_mode` |
| `CWInterface` | interface | `CWInterface.h` | `Interface` |
| `CWInterface.interfaceName` | property | `CWInterface.h` | `Interface::interface_name` |
| `CWInterface.powerOn` | method | `CWInterface.h` | `Interface::power_on` |
| `CWInterface.supportedWLANChannels` | method | `CWInterface.h` | `Interface::supported_wlan_channels` |
| `CWInterface.wlanChannel` | method | `CWInterface.h` | `Interface::wlan_channel` |
| `CWInterface.activePHYMode` | method | `CWInterface.h` | `Interface::active_phy_mode` |
| `CWInterface.ssid` | method | `CWInterface.h` | `Interface::ssid` |
| `CWInterface.ssidData` | method | `CWInterface.h` | `Interface::ssid_data` |
| `CWInterface.bssid` | method | `CWInterface.h` | `Interface::bssid` |
| `CWInterface.rssiValue` | method | `CWInterface.h` | `Interface::rssi_value` |
| `CWInterface.noiseMeasurement` | method | `CWInterface.h` | `Interface::noise_measurement` |
| `CWInterface.security` | method | `CWInterface.h` | `Interface::security` |
| `CWInterface.transmitRate` | method | `CWInterface.h` | `Interface::transmit_rate` |
| `CWInterface.countryCode` | method | `CWInterface.h` | `Interface::country_code` |
| `CWInterface.interfaceMode` | method | `CWInterface.h` | `Interface::interface_mode` |
| `CWInterface.transmitPower` | method | `CWInterface.h` | `Interface::transmit_power` |
| `CWInterface.hardwareAddress` | method | `CWInterface.h` | `Interface::hardware_address` |
| `CWInterface.serviceActive` | method | `CWInterface.h` | `Interface::service_active` |
| `CWInterface.cachedScanResults` | method | `CWInterface.h` | `Interface::cached_scan_results` |
| `CWInterface.configuration` | method | `CWInterface.h` | `Interface::configuration` |
| `CWInterface.setPower:error:` | method | `CWInterface.h` | `Interface::set_power` |
| `CWInterface.setWLANChannel:error:` | method | `CWInterface.h` | `Interface::set_wlan_channel` |
| `CWInterface.setPairwiseMasterKey:error:` | method | `CWInterface.h` | `Interface::set_pairwise_master_key` |
| `CWInterface.setWEPKey:flags:index:error:` | method | `CWInterface.h` | `Interface::set_wep_key` |
| `CWInterface.scanForNetworksWithSSID:error:` | method | `CWInterface.h` | `Interface::scan_for_networks_with_ssid` |
| `CWInterface.scanForNetworksWithSSID:includeHidden:error:` | method | `CWInterface.h` | `Interface::scan_for_networks_with_ssid_include_hidden` |
| `CWInterface.scanForNetworksWithName:error:` | method | `CWInterface.h` | `Interface::scan_for_networks_with_name` |
| `CWInterface.scanForNetworksWithName:includeHidden:error:` | method | `CWInterface.h` | `Interface::scan_for_networks_with_name_include_hidden` |
| `CWInterface.associateToNetwork:password:error:` | method | `CWInterface.h` | `Interface::associate_to_network` |
| `CWInterface.disassociate` | method | `CWInterface.h` | `Interface::disassociate` |
| `CWInterface.associateToEnterpriseNetwork:identity:username:password:error:` | method | `CWInterface.h` | `Interface::associate_to_enterprise_network` |
| `CWInterface.commitConfiguration:authorization:error:` | method | `CWInterface.h` | `Interface::commit_configuration` |
| `CWEventDelegate` | protocol | `CWWiFiClient.h` | `WiFiClientEventDelegate` |
| `CWEventDelegate.clientConnectionInterrupted` | protocol method | `CWWiFiClient.h` | `WiFiClientEventDelegate::client_connection_interrupted` |
| `CWEventDelegate.clientConnectionInvalidated` | protocol method | `CWWiFiClient.h` | `WiFiClientEventDelegate::client_connection_invalidated` |
| `CWEventDelegate.powerStateDidChangeForWiFiInterfaceWithName:` | protocol method | `CWWiFiClient.h` | `WiFiClientEventDelegate::power_state_did_change` |
| `CWEventDelegate.ssidDidChangeForWiFiInterfaceWithName:` | protocol method | `CWWiFiClient.h` | `WiFiClientEventDelegate::ssid_did_change` |
| `CWEventDelegate.bssidDidChangeForWiFiInterfaceWithName:` | protocol method | `CWWiFiClient.h` | `WiFiClientEventDelegate::bssid_did_change` |
| `CWEventDelegate.countryCodeDidChangeForWiFiInterfaceWithName:` | protocol method | `CWWiFiClient.h` | `WiFiClientEventDelegate::country_code_did_change` |
| `CWEventDelegate.linkDidChangeForWiFiInterfaceWithName:` | protocol method | `CWWiFiClient.h` | `WiFiClientEventDelegate::link_did_change` |
| `CWEventDelegate.linkQualityDidChangeForWiFiInterfaceWithName:rssi:transmitRate:` | protocol method | `CWWiFiClient.h` | `WiFiClientEventDelegate::link_quality_did_change` |
| `CWEventDelegate.modeDidChangeForWiFiInterfaceWithName:` | protocol method | `CWWiFiClient.h` | `WiFiClientEventDelegate::mode_did_change` |
| `CWEventDelegate.scanCacheUpdatedForWiFiInterfaceWithName:` | protocol method | `CWWiFiClient.h` | `WiFiClientEventDelegate::scan_cache_updated` |
| `CWWiFiClient` | interface | `CWWiFiClient.h` | `WiFiClient` |
| `CWWiFiClient.delegate` | property | `CWWiFiClient.h` | `WiFiClient::set_delegate / WiFiClient::clear_delegate / WiFiClient::has_delegate / DelegateRegistration` |
| `CWWiFiClient.sharedWiFiClient` | class method | `CWWiFiClient.h` | `WiFiClient::shared` |
| `CWWiFiClient.init` | method | `CWWiFiClient.h` | `WiFiClient::new` |
| `CWWiFiClient.interface` | method | `CWWiFiClient.h` | `WiFiClient::interface` |
| `CWWiFiClient.interfaceNames` | method | `CWWiFiClient.h` | `WiFiClient::interface_names` |
| `CWWiFiClient.interfaceWithName:` | method | `CWWiFiClient.h` | `WiFiClient::interface_with_name` |
| `CWWiFiClient.interfaces` | method | `CWWiFiClient.h` | `WiFiClient::interfaces` |
| `CWWiFiClient.startMonitoringEventWithType:error:` | method | `CWWiFiClient.h` | `WiFiClient::start_monitoring_event` |
| `CWWiFiClient.stopMonitoringEventWithType:error:` | method | `CWWiFiClient.h` | `WiFiClient::stop_monitoring_event` |
| `CWWiFiClient.stopMonitoringAllEventsAndReturnError:` | method | `CWWiFiClient.h` | `WiFiClient::stop_monitoring_all_events` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |

No non-deprecated gaps identified.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `CWIBSSModeSecurity` | typedef | `CoreWLANTypes.h` | Deprecated IBSS security enum; ad-hoc APIs are intentionally excluded from the coverage denominator. | `NS_ENUM_DEPRECATED_MAC(10_7, 11_0)` |
| `CWPowerDidChangeNotification` | constant | `CoreWLANConstants.h` | Deprecated notification constant; modern event monitoring uses CWWiFiClient/CWEventType, so audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_6, 10_10)` |
| `CWSSIDDidChangeNotification` | constant | `CoreWLANConstants.h` | Deprecated notification constant; modern event monitoring uses CWWiFiClient/CWEventType, so audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_6, 10_10)` |
| `CWBSSIDDidChangeNotification` | constant | `CoreWLANConstants.h` | Deprecated notification constant; modern event monitoring uses CWWiFiClient/CWEventType, so audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_6, 10_10)` |
| `CWLinkDidChangeNotification` | constant | `CoreWLANConstants.h` | Deprecated notification constant; modern event monitoring uses CWWiFiClient/CWEventType, so audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_6, 10_10)` |
| `CWModeDidChangeNotification` | constant | `CoreWLANConstants.h` | Deprecated notification constant; modern event monitoring uses CWWiFiClient/CWEventType, so audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_6, 10_10)` |
| `CWCountryCodeDidChangeNotification` | constant | `CoreWLANConstants.h` | Deprecated notification constant; modern event monitoring uses CWWiFiClient/CWEventType, so audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_6, 10_10)` |
| `CWScanCacheDidUpdateNotification` | constant | `CoreWLANConstants.h` | Deprecated notification constant; modern event monitoring uses CWWiFiClient/CWEventType, so audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_7, 10_10)` |
| `CWLinkQualityDidChangeNotification` | constant | `CoreWLANConstants.h` | Deprecated notification constant; modern event monitoring uses CWWiFiClient/CWEventType, so audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_7, 10_10)` |
| `CWLinkQualityNotificationRSSIKey` | constant | `CoreWLANConstants.h` | Deprecated notification constant; modern event monitoring uses CWWiFiClient/CWEventType, so audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_6, 10_10)` |
| `CWLinkQualityNotificationTransmitRateKey` | constant | `CoreWLANConstants.h` | Deprecated notification constant; modern event monitoring uses CWWiFiClient/CWEventType, so audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_6, 10_10)` |
| `CWKeychainCopyEAPUsernameAndPassword` | function | `CoreWLANUtil.h` | Deprecated pre-10.9 keychain helper; the Wi-Fi-scoped replacement exists and audit policy excludes the legacy symbol. | `NS_DEPRECATED_MAC(10_7, 10_9)` |
| `CWKeychainSetEAPUsernameAndPassword` | function | `CoreWLANUtil.h` | Deprecated pre-10.9 keychain helper; the Wi-Fi-scoped replacement exists and audit policy excludes the legacy symbol. | `NS_DEPRECATED_MAC(10_7, 10_9)` |
| `CWKeychainDeleteEAPUsernameAndPassword` | function | `CoreWLANUtil.h` | Deprecated pre-10.9 keychain helper; the Wi-Fi-scoped replacement exists and audit policy excludes the legacy symbol. | `NS_DEPRECATED_MAC(10_7, 10_9)` |
| `CWKeychainCopyEAPIdentity` | function | `CoreWLANUtil.h` | Deprecated pre-10.9 keychain helper; the Wi-Fi-scoped replacement exists and audit policy excludes the legacy symbol. | `NS_DEPRECATED_MAC(10_7, 10_9)` |
| `CWKeychainSetEAPIdentity` | function | `CoreWLANUtil.h` | Deprecated pre-10.9 keychain helper; the Wi-Fi-scoped replacement exists and audit policy excludes the legacy symbol. | `NS_DEPRECATED_MAC(10_7, 10_9)` |
| `CWKeychainSetPassword` | function | `CoreWLANUtil.h` | Deprecated pre-10.9 keychain helper; the Wi-Fi-scoped replacement exists and audit policy excludes the legacy symbol. | `NS_DEPRECATED_MAC(10_7, 10_9)` |
| `CWKeychainCopyPassword` | function | `CoreWLANUtil.h` | Deprecated pre-10.9 keychain helper; the Wi-Fi-scoped replacement exists and audit policy excludes the legacy symbol. | `NS_DEPRECATED_MAC(10_7, 10_9)` |
| `CWKeychainDeletePassword` | function | `CoreWLANUtil.h` | Deprecated pre-10.9 keychain helper; the Wi-Fi-scoped replacement exists and audit policy excludes the legacy symbol. | `NS_DEPRECATED_MAC(10_7, 10_9)` |
| `CWMutableConfiguration.requireAdministratorForIBSSMode` | property | `CWConfiguration.h` | Deprecated IBSS-mode setter; ad-hoc configuration APIs are intentionally excluded from the denominator. | `NS_DEPRECATED_MAC(10_7, 11_0)` |
| `CWInterface.interfaceNames` | class method | `CWInterface.h` | Deprecated constructor/helper superseded by CWWiFiClient entry points; audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_6, 10_10)` |
| `CWInterface.interface` | class method | `CWInterface.h` | Deprecated constructor/helper superseded by CWWiFiClient entry points; audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_6, 10_10)` |
| `CWInterface.interfaceWithName:` | class method | `CWInterface.h` | Deprecated constructor/helper superseded by CWWiFiClient entry points; audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_6, 10_10)` |
| `CWInterface.initWithInterfaceName:` | method | `CWInterface.h` | Deprecated constructor/helper superseded by CWWiFiClient entry points; audit policy marks it exempt. | `NS_DEPRECATED_MAC(10_6, 10_10)` |
| `CWInterface.startIBSSModeWithSSID:security:channel:password:error:` | method | `CWInterface.h` | Deprecated IBSS network creation API intentionally excluded from the coverage denominator. | `NS_DEPRECATED_MAC(10_7, 11_0)` |
| `CWWiFiClient.interfaceNames` | class method | `CWWiFiClient.h` | Deprecated class helper superseded by the instance method -[CWWiFiClient interfaceNames]. | `NS_DEPRECATED_MAC(10_10, 13_0)` |
