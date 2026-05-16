import CoreWLAN
import Foundation
import Security

private func keychainSsid(_ bytes: UnsafePointer<UInt8>?, _ length: Int) -> Data {
    dataFromBytes(bytes, length: length) ?? Data()
}

private func keychainDomain(_ rawValue: Int) -> CWKeychainDomain? {
    CWKeychainDomain(rawValue: rawValue)
}

@_cdecl("cwrs_cw_error_domain")
public func cwrs_cw_error_domain() -> UnsafeMutableRawPointer? {
    retainString(CWErrorDomain)
}

@_cdecl("cwrs_power_did_change_notification")
public func cwrs_power_did_change_notification() -> UnsafeMutableRawPointer? {
    retainString(Notification.Name.CWPowerDidChange.rawValue)
}

@_cdecl("cwrs_ssid_did_change_notification")
public func cwrs_ssid_did_change_notification() -> UnsafeMutableRawPointer? {
    retainString(Notification.Name.CWSSIDDidChange.rawValue)
}

@_cdecl("cwrs_bssid_did_change_notification")
public func cwrs_bssid_did_change_notification() -> UnsafeMutableRawPointer? {
    retainString(Notification.Name.CWBSSIDDidChange.rawValue)
}

@_cdecl("cwrs_link_did_change_notification")
public func cwrs_link_did_change_notification() -> UnsafeMutableRawPointer? {
    retainString(Notification.Name.CWLinkDidChange.rawValue)
}

@_cdecl("cwrs_mode_did_change_notification")
public func cwrs_mode_did_change_notification() -> UnsafeMutableRawPointer? {
    retainString(Notification.Name.CWModeDidChange.rawValue)
}

@_cdecl("cwrs_country_code_did_change_notification")
public func cwrs_country_code_did_change_notification() -> UnsafeMutableRawPointer? {
    retainString(Notification.Name.CWCountryCodeDidChange.rawValue)
}

@_cdecl("cwrs_scan_cache_did_update_notification")
public func cwrs_scan_cache_did_update_notification() -> UnsafeMutableRawPointer? {
    retainString(Notification.Name.CWScanCacheDidUpdate.rawValue)
}

@_cdecl("cwrs_link_quality_did_change_notification")
public func cwrs_link_quality_did_change_notification() -> UnsafeMutableRawPointer? {
    retainString(Notification.Name.CWLinkQualityDidChange.rawValue)
}

@_cdecl("cwrs_link_quality_notification_rssi_key")
public func cwrs_link_quality_notification_rssi_key() -> UnsafeMutableRawPointer? {
    retainString(CWLinkQualityNotificationRSSIKey)
}

@_cdecl("cwrs_link_quality_notification_transmit_rate_key")
public func cwrs_link_quality_notification_transmit_rate_key() -> UnsafeMutableRawPointer? {
    retainString(CWLinkQualityNotificationTransmitRateKey)
}

@_cdecl("cwrs_merge_networks")
public func cwrs_merge_networks(
    _ networks: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) -> UnsafeMutableRawPointer? {
    let values: [CWNetwork] = collectObjects(networks, count: count, as: CWNetwork.self)
    let merged = CWMergeNetworks(Set(values))
    return retainHandle(NSSet(array: Array(merged)))
}

@_cdecl("cwrs_keychain_find_wifi_password")
public func cwrs_keychain_find_wifi_password(
    _ domain: Int,
    _ ssidBytes: UnsafePointer<UInt8>?,
    _ ssidLength: Int,
    _ passwordOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let domain = keychainDomain(domain) else {
        return Int32(errSecParam)
    }

    var password: NSString?
    let status = CWKeychainFindWiFiPassword(domain, keychainSsid(ssidBytes, ssidLength), &password)
    passwordOut?.pointee = retainString(password as String?)
    return status
}

@_cdecl("cwrs_keychain_set_wifi_password")
public func cwrs_keychain_set_wifi_password(
    _ domain: Int,
    _ ssidBytes: UnsafePointer<UInt8>?,
    _ ssidLength: Int,
    _ password: UnsafePointer<CChar>?
) -> Int32 {
    guard
        let domain = keychainDomain(domain),
        let password = stringFromUtf8(password)
    else {
        return Int32(errSecParam)
    }

    return CWKeychainSetWiFiPassword(domain, keychainSsid(ssidBytes, ssidLength), password)
}

@_cdecl("cwrs_keychain_delete_wifi_password")
public func cwrs_keychain_delete_wifi_password(
    _ domain: Int,
    _ ssidBytes: UnsafePointer<UInt8>?,
    _ ssidLength: Int
) -> Int32 {
    guard let domain = keychainDomain(domain) else {
        return Int32(errSecParam)
    }

    return CWKeychainDeleteWiFiPassword(domain, keychainSsid(ssidBytes, ssidLength))
}

@_cdecl("cwrs_keychain_find_wifi_eap_username_and_password")
public func cwrs_keychain_find_wifi_eap_username_and_password(
    _ domain: Int,
    _ ssidBytes: UnsafePointer<UInt8>?,
    _ ssidLength: Int,
    _ usernameOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ passwordOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let domain = keychainDomain(domain) else {
        return Int32(errSecParam)
    }

    var username: NSString?
    var password: NSString?
    let status = CWKeychainFindWiFiEAPUsernameAndPassword(
        domain,
        keychainSsid(ssidBytes, ssidLength),
        &username,
        &password
    )
    usernameOut?.pointee = retainString(username as String?)
    passwordOut?.pointee = retainString(password as String?)
    return status
}

@_cdecl("cwrs_keychain_set_wifi_eap_username_and_password")
public func cwrs_keychain_set_wifi_eap_username_and_password(
    _ domain: Int,
    _ ssidBytes: UnsafePointer<UInt8>?,
    _ ssidLength: Int,
    _ username: UnsafePointer<CChar>?,
    _ password: UnsafePointer<CChar>?
) -> Int32 {
    guard let domain = keychainDomain(domain) else {
        return Int32(errSecParam)
    }

    return CWKeychainSetWiFiEAPUsernameAndPassword(
        domain,
        keychainSsid(ssidBytes, ssidLength),
        stringFromUtf8(username),
        stringFromUtf8(password)
    )
}

@_cdecl("cwrs_keychain_delete_wifi_eap_username_and_password")
public func cwrs_keychain_delete_wifi_eap_username_and_password(
    _ domain: Int,
    _ ssidBytes: UnsafePointer<UInt8>?,
    _ ssidLength: Int
) -> Int32 {
    guard let domain = keychainDomain(domain) else {
        return Int32(errSecParam)
    }

    return CWKeychainDeleteWiFiEAPUsernameAndPassword(domain, keychainSsid(ssidBytes, ssidLength))
}

@_cdecl("cwrs_keychain_copy_wifi_eap_identity")
public func cwrs_keychain_copy_wifi_eap_identity(
    _ domain: Int,
    _ ssidBytes: UnsafePointer<UInt8>?,
    _ ssidLength: Int,
    _ identityOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let domain = keychainDomain(domain) else {
        return Int32(errSecParam)
    }

    var identity: Unmanaged<SecIdentity>?
    let status = CWKeychainCopyWiFiEAPIdentity(domain, keychainSsid(ssidBytes, ssidLength), &identity)
    identityOut?.pointee = retainHandle(identity?.takeRetainedValue())
    return status
}

@_cdecl("cwrs_keychain_set_wifi_eap_identity")
public func cwrs_keychain_set_wifi_eap_identity(
    _ domain: Int,
    _ ssidBytes: UnsafePointer<UInt8>?,
    _ ssidLength: Int,
    _ identityHandle: UnsafeMutableRawPointer?
) -> Int32 {
    guard let domain = keychainDomain(domain) else {
        return Int32(errSecParam)
    }

    let identity: SecIdentity? = borrowHandle(identityHandle, as: SecIdentity.self)
    return CWKeychainSetWiFiEAPIdentity(
        domain,
        keychainSsid(ssidBytes, ssidLength),
        identity
    )
}

@_cdecl("cwrs_keychain_copy_eap_identity_list")
public func cwrs_keychain_copy_eap_identity_list(
    _ listOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    var list: Unmanaged<CFArray>?
    let status = CWKeychainCopyEAPIdentityList(&list)
    listOut?.pointee = retainHandle(list?.takeRetainedValue() as NSArray?)
    return status
}
