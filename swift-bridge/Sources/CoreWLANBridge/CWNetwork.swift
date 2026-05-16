import CoreWLAN
import Foundation

@_cdecl("cwrs_network_ssid")
public func cwrs_network_ssid(_ networkHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let network: CWNetwork? = borrowHandle(networkHandle, as: CWNetwork.self)
    return retainString(network?.value(forKey: "ssid") as? String)
}

@_cdecl("cwrs_network_ssid_data")
public func cwrs_network_ssid_data(_ networkHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let network: CWNetwork? = borrowHandle(networkHandle, as: CWNetwork.self)
    return retainData(network?.value(forKey: "ssidData") as? Data)
}

@_cdecl("cwrs_network_bssid")
public func cwrs_network_bssid(_ networkHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let network: CWNetwork? = borrowHandle(networkHandle, as: CWNetwork.self)
    return retainString(network?.value(forKey: "bssid") as? String)
}

@_cdecl("cwrs_network_wlan_channel")
public func cwrs_network_wlan_channel(_ networkHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let network: CWNetwork? = borrowHandle(networkHandle, as: CWNetwork.self)
    return retainHandle(network?.value(forKey: "wlanChannel") as? CWChannel)
}

@_cdecl("cwrs_network_rssi_value")
public func cwrs_network_rssi_value(_ networkHandle: UnsafeMutableRawPointer?) -> Int {
    let network: CWNetwork? = borrowHandle(networkHandle, as: CWNetwork.self)
    let number = network?.value(forKey: "rssiValue") as? NSNumber
    return number?.intValue ?? 0
}

@_cdecl("cwrs_network_noise_measurement")
public func cwrs_network_noise_measurement(_ networkHandle: UnsafeMutableRawPointer?) -> Int {
    let network: CWNetwork? = borrowHandle(networkHandle, as: CWNetwork.self)
    let number = network?.value(forKey: "noiseMeasurement") as? NSNumber
    return number?.intValue ?? 0
}

@_cdecl("cwrs_network_information_element_data")
public func cwrs_network_information_element_data(_ networkHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let network: CWNetwork? = borrowHandle(networkHandle, as: CWNetwork.self)
    return retainData(network?.value(forKey: "informationElementData") as? Data)
}

@_cdecl("cwrs_network_country_code")
public func cwrs_network_country_code(_ networkHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let network: CWNetwork? = borrowHandle(networkHandle, as: CWNetwork.self)
    return retainString(network?.value(forKey: "countryCode") as? String)
}

@_cdecl("cwrs_network_beacon_interval")
public func cwrs_network_beacon_interval(_ networkHandle: UnsafeMutableRawPointer?) -> Int {
    let network: CWNetwork? = borrowHandle(networkHandle, as: CWNetwork.self)
    let number = network?.value(forKey: "beaconInterval") as? NSNumber
    return number?.intValue ?? 0
}

@_cdecl("cwrs_network_ibss")
public func cwrs_network_ibss(_ networkHandle: UnsafeMutableRawPointer?) -> Bool {
    let network: CWNetwork? = borrowHandle(networkHandle, as: CWNetwork.self)
    let number = network?.value(forKey: "ibss") as? NSNumber
    return number?.boolValue ?? false
}

@_cdecl("cwrs_network_supports_security")
public func cwrs_network_supports_security(_ networkHandle: UnsafeMutableRawPointer?, _ security: Int) -> Bool {
    guard
        let network: CWNetwork = borrowHandle(networkHandle, as: CWNetwork.self),
        let security = CWSecurity(rawValue: security)
    else {
        return false
    }

    return network.supportsSecurity(security)
}

@_cdecl("cwrs_network_supports_phy_mode")
public func cwrs_network_supports_phy_mode(_ networkHandle: UnsafeMutableRawPointer?, _ phyMode: Int) -> Bool {
    guard
        let network: CWNetwork = borrowHandle(networkHandle, as: CWNetwork.self),
        let phyMode = CWPHYMode(rawValue: phyMode)
    else {
        return false
    }

    return network.supportsPHYMode(phyMode)
}

@_cdecl("cwrs_network_equal")
public func cwrs_network_equal(_ lhsHandle: UnsafeMutableRawPointer?, _ rhsHandle: UnsafeMutableRawPointer?) -> Bool {
    guard
        let lhs: CWNetwork = borrowHandle(lhsHandle, as: CWNetwork.self),
        let rhs: CWNetwork = borrowHandle(rhsHandle, as: CWNetwork.self)
    else {
        return false
    }

    return lhs.isEqual(rhs)
}
