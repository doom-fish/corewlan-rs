import CoreWLAN
import Foundation

@_cdecl("cwrs_mutable_network_profile_new")
public func cwrs_mutable_network_profile_new() -> UnsafeMutableRawPointer? {
    retainHandle(CWMutableNetworkProfile())
}

@_cdecl("cwrs_mutable_network_profile_with_network_profile")
public func cwrs_mutable_network_profile_with_network_profile(_ profileHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let profile: CWNetworkProfile = borrowHandle(profileHandle, as: CWNetworkProfile.self) else {
        return nil
    }

    return retainHandle(profile.mutableCopy() as? CWMutableNetworkProfile)
}

@_cdecl("cwrs_mutable_network_profile_set_ssid_data")
public func cwrs_mutable_network_profile_set_ssid_data(
    _ profileHandle: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int
) {
    guard let profile: CWMutableNetworkProfile = borrowHandle(profileHandle, as: CWMutableNetworkProfile.self) else {
        return
    }

    profile.ssidData = dataFromBytes(bytes, length: length)
}

@_cdecl("cwrs_mutable_network_profile_set_security")
public func cwrs_mutable_network_profile_set_security(_ profileHandle: UnsafeMutableRawPointer?, _ security: Int) {
    guard
        let profile: CWMutableNetworkProfile = borrowHandle(profileHandle, as: CWMutableNetworkProfile.self),
        let security = CWSecurity(rawValue: security)
    else {
        return
    }

    profile.security = security
}
