import CoreWLAN
import Foundation

@_cdecl("cwrs_network_profile_new")
public func cwrs_network_profile_new() -> UnsafeMutableRawPointer? {
    retainHandle(CWNetworkProfile())
}

@_cdecl("cwrs_network_profile_with_network_profile")
public func cwrs_network_profile_with_network_profile(_ profileHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let profile: CWNetworkProfile = borrowHandle(profileHandle, as: CWNetworkProfile.self) else {
        return nil
    }

    return retainHandle(profile.copy() as? CWNetworkProfile)
}

@_cdecl("cwrs_network_profile_ssid")
public func cwrs_network_profile_ssid(_ profileHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let profile: CWNetworkProfile? = borrowHandle(profileHandle, as: CWNetworkProfile.self)
    return retainString(profile?.ssid)
}

@_cdecl("cwrs_network_profile_ssid_data")
public func cwrs_network_profile_ssid_data(_ profileHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let profile: CWNetworkProfile? = borrowHandle(profileHandle, as: CWNetworkProfile.self)
    return retainData(profile?.ssidData)
}

@_cdecl("cwrs_network_profile_security")
public func cwrs_network_profile_security(_ profileHandle: UnsafeMutableRawPointer?) -> Int {
    let profile: CWNetworkProfile? = borrowHandle(profileHandle, as: CWNetworkProfile.self)
    return profile.map { Int($0.security.rawValue) } ?? Int.max
}

@_cdecl("cwrs_network_profile_equal")
public func cwrs_network_profile_equal(_ lhsHandle: UnsafeMutableRawPointer?, _ rhsHandle: UnsafeMutableRawPointer?) -> Bool {
    guard
        let lhs: CWNetworkProfile = borrowHandle(lhsHandle, as: CWNetworkProfile.self),
        let rhs: CWNetworkProfile = borrowHandle(rhsHandle, as: CWNetworkProfile.self)
    else {
        return false
    }

    return lhs.isEqual(rhs)
}
