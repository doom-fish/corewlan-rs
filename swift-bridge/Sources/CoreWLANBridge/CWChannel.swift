import CoreWLAN
import Foundation

@_cdecl("cwrs_channel_number")
public func cwrs_channel_number(_ channelHandle: UnsafeMutableRawPointer?) -> Int {
    let channel: CWChannel? = borrowHandle(channelHandle, as: CWChannel.self)
    return channel?.channelNumber ?? 0
}

@_cdecl("cwrs_channel_width")
public func cwrs_channel_width(_ channelHandle: UnsafeMutableRawPointer?) -> Int {
    let channel: CWChannel? = borrowHandle(channelHandle, as: CWChannel.self)
    return channel.map { Int($0.channelWidth.rawValue) } ?? 0
}

@_cdecl("cwrs_channel_band")
public func cwrs_channel_band(_ channelHandle: UnsafeMutableRawPointer?) -> Int {
    let channel: CWChannel? = borrowHandle(channelHandle, as: CWChannel.self)
    return channel.map { Int($0.channelBand.rawValue) } ?? 0
}

@_cdecl("cwrs_channel_equal")
public func cwrs_channel_equal(_ lhsHandle: UnsafeMutableRawPointer?, _ rhsHandle: UnsafeMutableRawPointer?) -> Bool {
    guard
        let lhs: CWChannel = borrowHandle(lhsHandle, as: CWChannel.self),
        let rhs: CWChannel = borrowHandle(rhsHandle, as: CWChannel.self)
    else {
        return false
    }

    return lhs.isEqual(rhs)
}
