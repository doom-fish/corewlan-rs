import CoreLocation
import Foundation

@_cdecl("cwrs_location_authorization_status")
public func cwrs_location_authorization_status() -> Int32 {
    guard CLLocationManager.locationServicesEnabled() else {
        return -1
    }

    if #available(macOS 11.0, *) {
        return CLLocationManager().authorizationStatus.rawValue
    }

    return CLLocationManager.authorizationStatus().rawValue
}
