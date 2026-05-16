import CoreWLAN
import Foundation
import ObjectiveC

@frozen
public struct WiFiClientDelegateCallbacks {
    public var clientConnectionInterrupted: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
    public var clientConnectionInvalidated: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
    public var powerStateDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?
    public var ssidDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?
    public var bssidDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?
    public var countryCodeDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?
    public var linkDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?
    public var linkQualityDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?, Int, Double) -> Void)?
    public var modeDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?
    public var scanCacheUpdated: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?
}

private final class WiFiClientDelegateBridge: NSObject, CWEventDelegate {
    let callbacks: WiFiClientDelegateCallbacks
    let context: UnsafeMutableRawPointer?
    let releaseContext: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?

    init(
        callbacks: WiFiClientDelegateCallbacks,
        context: UnsafeMutableRawPointer?,
        releaseContext: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
    ) {
        self.callbacks = callbacks
        self.context = context
        self.releaseContext = releaseContext
    }

    deinit {
        releaseContext?(context)
    }

    func clientConnectionInterrupted() {
        callbacks.clientConnectionInterrupted?(context)
    }

    func clientConnectionInvalidated() {
        callbacks.clientConnectionInvalidated?(context)
    }

    func powerStateDidChangeForWiFiInterface(withName interfaceName: String) {
        withOptionalCString(interfaceName) { callbacks.powerStateDidChange?(context, $0) }
    }

    func ssidDidChangeForWiFiInterface(withName interfaceName: String) {
        withOptionalCString(interfaceName) { callbacks.ssidDidChange?(context, $0) }
    }

    func bssidDidChangeForWiFiInterface(withName interfaceName: String) {
        withOptionalCString(interfaceName) { callbacks.bssidDidChange?(context, $0) }
    }

    func countryCodeDidChangeForWiFiInterface(withName interfaceName: String) {
        withOptionalCString(interfaceName) { callbacks.countryCodeDidChange?(context, $0) }
    }

    func linkDidChangeForWiFiInterface(withName interfaceName: String) {
        withOptionalCString(interfaceName) { callbacks.linkDidChange?(context, $0) }
    }

    func linkQualityDidChangeForWiFiInterface(withName interfaceName: String, rssi: Int, transmitRate: Double) {
        withOptionalCString(interfaceName) {
            callbacks.linkQualityDidChange?(context, $0, rssi, transmitRate)
        }
    }

    func modeDidChangeForWiFiInterface(withName interfaceName: String) {
        withOptionalCString(interfaceName) { callbacks.modeDidChange?(context, $0) }
    }

    func scanCacheUpdatedForWiFiInterface(withName interfaceName: String) {
        withOptionalCString(interfaceName) { callbacks.scanCacheUpdated?(context, $0) }
    }
}

private var delegateAssociationKey: UInt8 = 0

@_cdecl("cwrs_wifi_client_shared")
public func cwrs_wifi_client_shared() -> UnsafeMutableRawPointer? {
    let selector = NSSelectorFromString("sharedWiFiClient")
    let client = CWWiFiClient.perform(selector)?.takeUnretainedValue() as? CWWiFiClient
    return retainHandle(client)
}

@_cdecl("cwrs_wifi_client_new")
public func cwrs_wifi_client_new() -> UnsafeMutableRawPointer? {
    retainHandle(CWWiFiClient())
}

@_cdecl("cwrs_wifi_client_interface")
public func cwrs_wifi_client_interface(_ clientHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let client: CWWiFiClient = borrowHandle(clientHandle, as: CWWiFiClient.self) else {
        return nil
    }

    let interface = client.perform(NSSelectorFromString("interface"))?.takeUnretainedValue() as? CWInterface
    return retainHandle(interface)
}

@_cdecl("cwrs_wifi_client_interface_with_name")
public func cwrs_wifi_client_interface_with_name(
    _ clientHandle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let client: CWWiFiClient = borrowHandle(clientHandle, as: CWWiFiClient.self) else {
        return nil
    }

    let interfaceName = stringFromUtf8(name) as NSString?
    let interface = client.perform(NSSelectorFromString("interfaceWithName:"), with: interfaceName)?.takeUnretainedValue() as? CWInterface
    return retainHandle(interface)
}

@_cdecl("cwrs_wifi_client_interfaces")
public func cwrs_wifi_client_interfaces(_ clientHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let client: CWWiFiClient = borrowHandle(clientHandle, as: CWWiFiClient.self) else {
        return nil
    }

    let interfaces = client.perform(NSSelectorFromString("interfaces"))?.takeUnretainedValue() as? NSArray
    return retainHandle(interfaces)
}

@_cdecl("cwrs_wifi_client_interface_names")
public func cwrs_wifi_client_interface_names(_ clientHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let client: CWWiFiClient = borrowHandle(clientHandle, as: CWWiFiClient.self) else {
        return nil
    }

    let selector = NSSelectorFromString("interfaceNames")
    if client.responds(to: selector) {
        let names = client.perform(selector)?.takeUnretainedValue() as? NSArray
        return retainHandle(names)
    }

    let names = CWWiFiClient.perform(selector)?.takeUnretainedValue() as? NSArray
    return retainHandle(names)
}

@_cdecl("cwrs_wifi_client_start_monitoring_event")
public func cwrs_wifi_client_start_monitoring_event(
    _ clientHandle: UnsafeMutableRawPointer?,
    _ eventType: Int,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard
        let client: CWWiFiClient = borrowHandle(clientHandle, as: CWWiFiClient.self),
        let eventType = CWEventType(rawValue: eventType)
    else {
        return false
    }

    do {
        try client.startMonitoringEvent(with: eventType)
        return true
    } catch {
        setNSError(error, out: errorOut)
        return false
    }
}

@_cdecl("cwrs_wifi_client_stop_monitoring_event")
public func cwrs_wifi_client_stop_monitoring_event(
    _ clientHandle: UnsafeMutableRawPointer?,
    _ eventType: Int,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard
        let client: CWWiFiClient = borrowHandle(clientHandle, as: CWWiFiClient.self),
        let eventType = CWEventType(rawValue: eventType)
    else {
        return false
    }

    do {
        try client.stopMonitoringEvent(with: eventType)
        return true
    } catch {
        setNSError(error, out: errorOut)
        return false
    }
}

@_cdecl("cwrs_wifi_client_stop_monitoring_all_events")
public func cwrs_wifi_client_stop_monitoring_all_events(
    _ clientHandle: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let client: CWWiFiClient = borrowHandle(clientHandle, as: CWWiFiClient.self) else {
        return false
    }

    do {
        try client.stopMonitoringAllEvents()
        return true
    } catch {
        setNSError(error, out: errorOut)
        return false
    }
}

@_cdecl("cwrs_wifi_client_has_delegate")
public func cwrs_wifi_client_has_delegate(_ clientHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let client: CWWiFiClient = borrowHandle(clientHandle, as: CWWiFiClient.self) else {
        return false
    }

    return objc_getAssociatedObject(client, &delegateAssociationKey) != nil
}

@_cdecl("cwrs_wifi_client_set_delegate")
public func cwrs_wifi_client_set_delegate(
    _ clientHandle: UnsafeMutableRawPointer?,
    _ clientConnectionInterrupted: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?,
    _ clientConnectionInvalidated: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?,
    _ powerStateDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?,
    _ ssidDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?,
    _ bssidDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?,
    _ countryCodeDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?,
    _ linkDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?,
    _ linkQualityDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?, Int, Double) -> Void)?,
    _ modeDidChange: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?,
    _ scanCacheUpdated: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void)?,
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
) -> Bool {
    guard let client: CWWiFiClient = borrowHandle(clientHandle, as: CWWiFiClient.self) else {
        return false
    }

    let callbacks = WiFiClientDelegateCallbacks(
        clientConnectionInterrupted: clientConnectionInterrupted,
        clientConnectionInvalidated: clientConnectionInvalidated,
        powerStateDidChange: powerStateDidChange,
        ssidDidChange: ssidDidChange,
        bssidDidChange: bssidDidChange,
        countryCodeDidChange: countryCodeDidChange,
        linkDidChange: linkDidChange,
        linkQualityDidChange: linkQualityDidChange,
        modeDidChange: modeDidChange,
        scanCacheUpdated: scanCacheUpdated
    )

    let delegate = WiFiClientDelegateBridge(
        callbacks: callbacks,
        context: context,
        releaseContext: releaseContext
    )
    client.delegate = delegate
    objc_setAssociatedObject(client, &delegateAssociationKey, delegate, .OBJC_ASSOCIATION_RETAIN_NONATOMIC)
    return true
}

@_cdecl("cwrs_wifi_client_clear_delegate")
public func cwrs_wifi_client_clear_delegate(
    _ clientHandle: UnsafeMutableRawPointer?,
    _ context: UnsafeMutableRawPointer?
) {
    guard let client: CWWiFiClient = borrowHandle(clientHandle, as: CWWiFiClient.self) else {
        return
    }

    if let context,
       let delegate = objc_getAssociatedObject(client, &delegateAssociationKey) as? WiFiClientDelegateBridge,
       delegate.context != context {
        return
    }

    client.delegate = nil
    objc_setAssociatedObject(client, &delegateAssociationKey, nil, .OBJC_ASSOCIATION_RETAIN_NONATOMIC)
}
