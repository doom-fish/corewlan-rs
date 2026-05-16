import CoreWLAN
import Foundation

@_cdecl("cwrs_mutable_configuration_new")
public func cwrs_mutable_configuration_new() -> UnsafeMutableRawPointer? {
    retainHandle(CWMutableConfiguration())
}

@_cdecl("cwrs_mutable_configuration_with_configuration")
public func cwrs_mutable_configuration_with_configuration(_ configurationHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let configuration: CWConfiguration = borrowHandle(configurationHandle, as: CWConfiguration.self) else {
        return nil
    }

    return retainHandle(configuration.mutableCopy() as? CWMutableConfiguration)
}

@_cdecl("cwrs_mutable_configuration_set_network_profiles")
public func cwrs_mutable_configuration_set_network_profiles(
    _ configurationHandle: UnsafeMutableRawPointer?,
    _ profiles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) {
    guard let configuration: CWMutableConfiguration = borrowHandle(configurationHandle, as: CWMutableConfiguration.self) else {
        return
    }

    let values: [CWNetworkProfile] = collectObjects(profiles, count: count, as: CWNetworkProfile.self)
    configuration.networkProfiles = NSOrderedSet(array: values)
}

@_cdecl("cwrs_mutable_configuration_set_require_admin_association")
public func cwrs_mutable_configuration_set_require_admin_association(_ configurationHandle: UnsafeMutableRawPointer?, _ value: Bool) {
    let configuration: CWMutableConfiguration? = borrowHandle(configurationHandle, as: CWMutableConfiguration.self)
    configuration?.requireAdministratorForAssociation = value
}

@_cdecl("cwrs_mutable_configuration_set_require_admin_power")
public func cwrs_mutable_configuration_set_require_admin_power(_ configurationHandle: UnsafeMutableRawPointer?, _ value: Bool) {
    let configuration: CWMutableConfiguration? = borrowHandle(configurationHandle, as: CWMutableConfiguration.self)
    configuration?.requireAdministratorForPower = value
}

@_cdecl("cwrs_mutable_configuration_set_require_admin_ibss_mode")
public func cwrs_mutable_configuration_set_require_admin_ibss_mode(_ configurationHandle: UnsafeMutableRawPointer?, _ value: Bool) {
    let configuration: CWMutableConfiguration? = borrowHandle(configurationHandle, as: CWMutableConfiguration.self)
    configuration?.requireAdministratorForIBSSMode = value
}

@_cdecl("cwrs_mutable_configuration_set_remember_joined_networks")
public func cwrs_mutable_configuration_set_remember_joined_networks(_ configurationHandle: UnsafeMutableRawPointer?, _ value: Bool) {
    let configuration: CWMutableConfiguration? = borrowHandle(configurationHandle, as: CWMutableConfiguration.self)
    configuration?.rememberJoinedNetworks = value
}
