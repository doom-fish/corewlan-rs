import CoreWLAN
import Foundation
import Security
import SecurityFoundation

@_cdecl("cwrs_interface_name")
public func cwrs_interface_name(_ interfaceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    return retainString(interface?.interfaceName)
}

@_cdecl("cwrs_interface_power_on")
public func cwrs_interface_power_on(_ interfaceHandle: UnsafeMutableRawPointer?) -> Bool {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    let number = interface?.value(forKey: "powerOn") as? NSNumber
    return number?.boolValue ?? false
}

@_cdecl("cwrs_interface_supported_wlan_channels")
public func cwrs_interface_supported_wlan_channels(_ interfaceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    let channels = interface?.value(forKey: "supportedWLANChannels") as? Set<CWChannel>
    return retainSet(channels)
}

@_cdecl("cwrs_interface_wlan_channel")
public func cwrs_interface_wlan_channel(_ interfaceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    return retainHandle(interface?.value(forKey: "wlanChannel") as? CWChannel)
}

@_cdecl("cwrs_interface_active_phy_mode")
public func cwrs_interface_active_phy_mode(_ interfaceHandle: UnsafeMutableRawPointer?) -> Int {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    let number = interface?.value(forKey: "activePHYMode") as? NSNumber
    return number?.intValue ?? 0
}

@_cdecl("cwrs_interface_ssid")
public func cwrs_interface_ssid(_ interfaceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    return retainString(interface?.value(forKey: "ssid") as? String)
}

@_cdecl("cwrs_interface_ssid_data")
public func cwrs_interface_ssid_data(_ interfaceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    return retainData(interface?.value(forKey: "ssidData") as? Data)
}

@_cdecl("cwrs_interface_bssid")
public func cwrs_interface_bssid(_ interfaceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    return retainString(interface?.value(forKey: "bssid") as? String)
}

@_cdecl("cwrs_interface_rssi_value")
public func cwrs_interface_rssi_value(_ interfaceHandle: UnsafeMutableRawPointer?) -> Int {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    let number = interface?.value(forKey: "rssiValue") as? NSNumber
    return number?.intValue ?? 0
}

@_cdecl("cwrs_interface_noise_measurement")
public func cwrs_interface_noise_measurement(_ interfaceHandle: UnsafeMutableRawPointer?) -> Int {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    let number = interface?.value(forKey: "noiseMeasurement") as? NSNumber
    return number?.intValue ?? 0
}

@_cdecl("cwrs_interface_security")
public func cwrs_interface_security(_ interfaceHandle: UnsafeMutableRawPointer?) -> Int {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    let number = interface?.value(forKey: "security") as? NSNumber
    return number?.intValue ?? Int.max
}

@_cdecl("cwrs_interface_transmit_rate")
public func cwrs_interface_transmit_rate(_ interfaceHandle: UnsafeMutableRawPointer?) -> Double {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    let number = interface?.value(forKey: "transmitRate") as? NSNumber
    return number?.doubleValue ?? 0.0
}

@_cdecl("cwrs_interface_country_code")
public func cwrs_interface_country_code(_ interfaceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    return retainString(interface?.value(forKey: "countryCode") as? String)
}

@_cdecl("cwrs_interface_mode")
public func cwrs_interface_mode(_ interfaceHandle: UnsafeMutableRawPointer?) -> Int {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    let number = interface?.value(forKey: "interfaceMode") as? NSNumber
    return number?.intValue ?? 0
}

@_cdecl("cwrs_interface_transmit_power")
public func cwrs_interface_transmit_power(_ interfaceHandle: UnsafeMutableRawPointer?) -> Int {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    let number = interface?.value(forKey: "transmitPower") as? NSNumber
    return number?.intValue ?? 0
}

@_cdecl("cwrs_interface_hardware_address")
public func cwrs_interface_hardware_address(_ interfaceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    return retainString(interface?.value(forKey: "hardwareAddress") as? String)
}

@_cdecl("cwrs_interface_service_active")
public func cwrs_interface_service_active(_ interfaceHandle: UnsafeMutableRawPointer?) -> Bool {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    let number = interface?.value(forKey: "serviceActive") as? NSNumber
    return number?.boolValue ?? false
}

@_cdecl("cwrs_interface_cached_scan_results")
public func cwrs_interface_cached_scan_results(_ interfaceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    let results = interface?.value(forKey: "cachedScanResults") as? Set<CWNetwork>
    return retainSet(results)
}

@_cdecl("cwrs_interface_configuration")
public func cwrs_interface_configuration(_ interfaceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    return retainHandle(interface?.value(forKey: "configuration") as? CWConfiguration)
}

@_cdecl("cwrs_interface_scan_for_networks_with_name")
public func cwrs_interface_scan_for_networks_with_name(
    _ interfaceHandle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ includeHidden: Bool,
    _ hasIncludeHidden: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> UnsafeMutableRawPointer? {
    guard let interface: CWInterface = borrowHandle(interfaceHandle, as: CWInterface.self) else {
        return nil
    }

    do {
        let results: Set<CWNetwork>
        if hasIncludeHidden {
            results = try interface.scanForNetworks(withName: stringFromUtf8(name), includeHidden: includeHidden)
        } else {
            results = try interface.scanForNetworks(withName: stringFromUtf8(name))
        }
        return retainSet(results)
    } catch {
        setNSError(error, out: errorOut)
        return nil
    }
}

@_cdecl("cwrs_interface_scan_for_networks_with_ssid")
public func cwrs_interface_scan_for_networks_with_ssid(
    _ interfaceHandle: UnsafeMutableRawPointer?,
    _ ssidBytes: UnsafePointer<UInt8>?,
    _ ssidLength: Int,
    _ includeHidden: Bool,
    _ hasIncludeHidden: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> UnsafeMutableRawPointer? {
    guard let interface: CWInterface = borrowHandle(interfaceHandle, as: CWInterface.self) else {
        return nil
    }

    let ssid = dataFromBytes(ssidBytes, length: ssidLength)

    do {
        let results: Set<CWNetwork>
        if hasIncludeHidden {
            results = try interface.scanForNetworks(withSSID: ssid, includeHidden: includeHidden)
        } else {
            results = try interface.scanForNetworks(withSSID: ssid)
        }
        return retainSet(results)
    } catch {
        setNSError(error, out: errorOut)
        return nil
    }
}

@_cdecl("cwrs_interface_set_power")
public func cwrs_interface_set_power(
    _ interfaceHandle: UnsafeMutableRawPointer?,
    _ powerOn: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let interface: CWInterface = borrowHandle(interfaceHandle, as: CWInterface.self) else {
        return false
    }

    do {
        try interface.setPower(powerOn)
        return true
    } catch {
        setNSError(error, out: errorOut)
        return false
    }
}

@_cdecl("cwrs_interface_set_wlan_channel")
public func cwrs_interface_set_wlan_channel(
    _ interfaceHandle: UnsafeMutableRawPointer?,
    _ channelHandle: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard
        let interface: CWInterface = borrowHandle(interfaceHandle, as: CWInterface.self),
        let channel: CWChannel = borrowHandle(channelHandle, as: CWChannel.self)
    else {
        return false
    }

    do {
        try interface.setWLANChannel(channel)
        return true
    } catch {
        setNSError(error, out: errorOut)
        return false
    }
}

@_cdecl("cwrs_interface_set_pairwise_master_key")
public func cwrs_interface_set_pairwise_master_key(
    _ interfaceHandle: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let interface: CWInterface = borrowHandle(interfaceHandle, as: CWInterface.self) else {
        return false
    }

    do {
        try interface.setPairwiseMasterKey(dataFromBytes(bytes, length: length))
        return true
    } catch {
        setNSError(error, out: errorOut)
        return false
    }
}

@_cdecl("cwrs_interface_set_wep_key")
public func cwrs_interface_set_wep_key(
    _ interfaceHandle: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ flags: UInt,
    _ index: Int,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let interface: CWInterface = borrowHandle(interfaceHandle, as: CWInterface.self) else {
        return false
    }

    do {
        try interface.setWEPKey(dataFromBytes(bytes, length: length), flags: CWCipherKeyFlags(rawValue: flags), index: index)
        return true
    } catch {
        setNSError(error, out: errorOut)
        return false
    }
}

@_cdecl("cwrs_interface_associate_to_network")
public func cwrs_interface_associate_to_network(
    _ interfaceHandle: UnsafeMutableRawPointer?,
    _ networkHandle: UnsafeMutableRawPointer?,
    _ password: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard
        let interface: CWInterface = borrowHandle(interfaceHandle, as: CWInterface.self),
        let network: CWNetwork = borrowHandle(networkHandle, as: CWNetwork.self)
    else {
        return false
    }

    do {
        try interface.associate(to: network, password: stringFromUtf8(password))
        return true
    } catch {
        setNSError(error, out: errorOut)
        return false
    }
}

@_cdecl("cwrs_interface_disassociate")
public func cwrs_interface_disassociate(_ interfaceHandle: UnsafeMutableRawPointer?) {
    let interface: CWInterface? = borrowHandle(interfaceHandle, as: CWInterface.self)
    interface?.disassociate()
}

@_cdecl("cwrs_interface_associate_to_enterprise_network")
public func cwrs_interface_associate_to_enterprise_network(
    _ interfaceHandle: UnsafeMutableRawPointer?,
    _ networkHandle: UnsafeMutableRawPointer?,
    _ identityHandle: UnsafeMutableRawPointer?,
    _ username: UnsafePointer<CChar>?,
    _ password: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard
        let interface: CWInterface = borrowHandle(interfaceHandle, as: CWInterface.self),
        let network: CWNetwork = borrowHandle(networkHandle, as: CWNetwork.self)
    else {
        return false
    }

    let identity: SecIdentity? = borrowHandle(identityHandle, as: SecIdentity.self)

    do {
        try interface.associate(toEnterpriseNetwork: network,
                                identity: identity,
                                username: stringFromUtf8(username),
                                password: stringFromUtf8(password))
        return true
    } catch {
        setNSError(error, out: errorOut)
        return false
    }
}

@_cdecl("cwrs_interface_commit_configuration")
public func cwrs_interface_commit_configuration(
    _ interfaceHandle: UnsafeMutableRawPointer?,
    _ configurationHandle: UnsafeMutableRawPointer?,
    _ authorizationHandle: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard
        let interface: CWInterface = borrowHandle(interfaceHandle, as: CWInterface.self),
        let configuration: CWConfiguration = borrowHandle(configurationHandle, as: CWConfiguration.self)
    else {
        return false
    }

    let authorization: SFAuthorization? = borrowHandle(authorizationHandle, as: SFAuthorization.self)

    do {
        try interface.commitConfiguration(configuration, authorization: authorization)
        return true
    } catch {
        setNSError(error, out: errorOut)
        return false
    }
}
