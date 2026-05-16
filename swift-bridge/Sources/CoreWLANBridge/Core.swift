import Foundation

final class AnyHandleBox {
    let value: Any

    init(_ value: Any) {
        self.value = value
    }
}

@inline(__always)
func retainHandle<T>(_ value: T?) -> UnsafeMutableRawPointer? {
    guard let value else {
        return nil
    }

    return Unmanaged.passRetained(AnyHandleBox(value)).toOpaque()
}

@inline(__always)
func borrowHandle<T>(_ handle: UnsafeMutableRawPointer?, as _: T.Type = T.self) -> T? {
    guard let handle else {
        return nil
    }

    let pointer = handle.assumingMemoryBound(to: AnyHandleBox.self)
    return Unmanaged<AnyHandleBox>.fromOpaque(UnsafeRawPointer(pointer)).takeUnretainedValue().value as? T
}

@inline(__always)
func retainString(_ value: String?) -> UnsafeMutableRawPointer? {
    guard let value else {
        return nil
    }

    return retainHandle(NSString(string: value))
}

@inline(__always)
func retainData(_ value: Data?) -> UnsafeMutableRawPointer? {
    guard let value else {
        return nil
    }

    return retainHandle(value as NSData)
}

@inline(__always)
func retainArray(_ value: [Any]?) -> UnsafeMutableRawPointer? {
    guard let value else {
        return nil
    }

    return retainHandle(NSArray(array: value))
}

@inline(__always)
func retainSet<T>(_ value: Set<T>?) -> UnsafeMutableRawPointer? {
    guard let value else {
        return nil
    }

    return retainHandle(NSSet(array: Array(value)))
}

@inline(__always)
func retainOrderedSet(_ value: NSOrderedSet?) -> UnsafeMutableRawPointer? {
    retainHandle(value)
}

@inline(__always)
func stringFromUtf8(_ value: UnsafePointer<CChar>?) -> String? {
    guard let value else {
        return nil
    }

    return String(cString: value)
}

@inline(__always)
func dataFromBytes(_ bytes: UnsafePointer<UInt8>?, length: Int) -> Data? {
    guard let bytes else {
        return nil
    }

    return Data(bytes: bytes, count: length)
}

@inline(__always)
func duplicatedCString(_ value: String?) -> UnsafeMutablePointer<CChar>? {
    guard let value else {
        return nil
    }

    return strdup(value)
}

@inline(__always)
func withOptionalCString<R>(_ value: String?, _ body: (UnsafePointer<CChar>?) -> R) -> R {
    guard let value else {
        return body(nil)
    }

    return value.withCString(body)
}

@inline(__always)
func setNSError(_ error: Error, out errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?) {
    errorOut?.pointee = retainHandle(error as NSError)
}

@inline(__always)
func collectObjects<T>(_ handles: UnsafePointer<UnsafeMutableRawPointer?>?, count: Int, as _: T.Type = T.self) -> [T] {
    guard let handles else {
        return []
    }

    let buffer = UnsafeBufferPointer(start: handles, count: count)
    return buffer.compactMap { borrowHandle($0, as: T.self) }
}

@_cdecl("cwrs_retain")
public func cwrs_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }

    _ = Unmanaged<AnyObject>.fromOpaque(UnsafeRawPointer(handle)).retain()
    return handle
}

@_cdecl("cwrs_release")
public func cwrs_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }

    Unmanaged<AnyObject>.fromOpaque(UnsafeRawPointer(handle)).release()
}

@_cdecl("cwrs_free_buffer")
public func cwrs_free_buffer(_ buffer: UnsafeMutableRawPointer?) {
    free(buffer)
}

@_cdecl("cwrs_string_copy_utf8")
public func cwrs_string_copy_utf8(_ stringHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let string: NSString? = borrowHandle(stringHandle, as: NSString.self)
    return duplicatedCString(string as String?)
}

@_cdecl("cwrs_data_copy_bytes")
public func cwrs_data_copy_bytes(
    _ dataHandle: UnsafeMutableRawPointer?,
    _ lengthOut: UnsafeMutablePointer<Int>?
) -> UnsafeMutablePointer<UInt8>? {
    guard let data: NSData = borrowHandle(dataHandle, as: NSData.self) else {
        lengthOut?.pointee = 0
        return nil
    }

    lengthOut?.pointee = data.length
    guard data.length > 0, let rawBuffer = malloc(data.length) else {
        return nil
    }

    data.getBytes(rawBuffer, length: data.length)
    return rawBuffer.assumingMemoryBound(to: UInt8.self)
}

@_cdecl("cwrs_array_count")
public func cwrs_array_count(_ arrayHandle: UnsafeMutableRawPointer?) -> Int {
    let array: NSArray? = borrowHandle(arrayHandle, as: NSArray.self)
    return array?.count ?? 0
}

@_cdecl("cwrs_array_object_at_index")
public func cwrs_array_object_at_index(_ arrayHandle: UnsafeMutableRawPointer?, _ index: Int) -> UnsafeMutableRawPointer? {
    guard let array: NSArray = borrowHandle(arrayHandle, as: NSArray.self), index >= 0, index < array.count else {
        return nil
    }

    return retainHandle(Optional(array.object(at: index)))
}

@_cdecl("cwrs_ordered_set_count")
public func cwrs_ordered_set_count(_ setHandle: UnsafeMutableRawPointer?) -> Int {
    let orderedSet: NSOrderedSet? = borrowHandle(setHandle, as: NSOrderedSet.self)
    return orderedSet?.count ?? 0
}

@_cdecl("cwrs_ordered_set_object_at_index")
public func cwrs_ordered_set_object_at_index(_ setHandle: UnsafeMutableRawPointer?, _ index: Int) -> UnsafeMutableRawPointer? {
    guard let orderedSet: NSOrderedSet = borrowHandle(setHandle, as: NSOrderedSet.self), index >= 0, index < orderedSet.count else {
        return nil
    }

    return retainHandle(Optional(orderedSet.object(at: index)))
}

@_cdecl("cwrs_set_copy_all_objects")
public func cwrs_set_copy_all_objects(_ setHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let set: NSSet = borrowHandle(setHandle, as: NSSet.self) else {
        return nil
    }

    return retainHandle(NSArray(array: set.allObjects))
}

@_cdecl("cwrs_error_code")
public func cwrs_error_code(_ errorHandle: UnsafeMutableRawPointer?) -> Int {
    let error: NSError? = borrowHandle(errorHandle, as: NSError.self)
    return error?.code ?? 0
}

@_cdecl("cwrs_error_domain")
public func cwrs_error_domain(_ errorHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let error: NSError? = borrowHandle(errorHandle, as: NSError.self)
    return duplicatedCString(error?.domain)
}

@_cdecl("cwrs_error_description")
public func cwrs_error_description(_ errorHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let error: NSError? = borrowHandle(errorHandle, as: NSError.self)
    return duplicatedCString(error?.localizedDescription)
}
