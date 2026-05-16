import CoreWLAN
import Foundation

@_cdecl("cwrs_configuration_new")
public func cwrs_configuration_new() -> UnsafeMutableRawPointer? {
    retainHandle(CWConfiguration())
}

@_cdecl("cwrs_configuration_with_configuration")
public func cwrs_configuration_with_configuration(_ configurationHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let configuration: CWConfiguration = borrowHandle(configurationHandle, as: CWConfiguration.self) else {
        return nil
    }

    return retainHandle(configuration.copy() as? CWConfiguration)
}

@_cdecl("cwrs_configuration_network_profiles")
public func cwrs_configuration_network_profiles(_ configurationHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let configuration: CWConfiguration? = borrowHandle(configurationHandle, as: CWConfiguration.self)
    return retainOrderedSet(configuration?.networkProfiles)
}

@_cdecl("cwrs_configuration_require_admin_association")
public func cwrs_configuration_require_admin_association(_ configurationHandle: UnsafeMutableRawPointer?) -> Bool {
    let configuration: CWConfiguration? = borrowHandle(configurationHandle, as: CWConfiguration.self)
    return configuration?.requireAdministratorForAssociation ?? false
}

@_cdecl("cwrs_configuration_require_admin_power")
public func cwrs_configuration_require_admin_power(_ configurationHandle: UnsafeMutableRawPointer?) -> Bool {
    let configuration: CWConfiguration? = borrowHandle(configurationHandle, as: CWConfiguration.self)
    return configuration?.requireAdministratorForPower ?? false
}

@_cdecl("cwrs_configuration_require_admin_ibss_mode")
public func cwrs_configuration_require_admin_ibss_mode(_ configurationHandle: UnsafeMutableRawPointer?) -> Bool {
    let configuration: CWConfiguration? = borrowHandle(configurationHandle, as: CWConfiguration.self)
    return configuration?.requireAdministratorForIBSSMode ?? false
}

@_cdecl("cwrs_configuration_remember_joined_networks")
public func cwrs_configuration_remember_joined_networks(_ configurationHandle: UnsafeMutableRawPointer?) -> Bool {
    let configuration: CWConfiguration? = borrowHandle(configurationHandle, as: CWConfiguration.self)
    return configuration?.rememberJoinedNetworks ?? false
}

@_cdecl("cwrs_configuration_equal")
public func cwrs_configuration_equal(_ lhsHandle: UnsafeMutableRawPointer?, _ rhsHandle: UnsafeMutableRawPointer?) -> Bool {
    guard
        let lhs: CWConfiguration = borrowHandle(lhsHandle, as: CWConfiguration.self),
        let rhs: CWConfiguration = borrowHandle(rhsHandle, as: CWConfiguration.self)
    else {
        return false
    }

    return lhs.isEqual(rhs)
}
