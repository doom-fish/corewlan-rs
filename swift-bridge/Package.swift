// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "CoreWLANBridge",
    platforms: [
        .macOS(.v10_13)
    ],
    products: [
        .library(
            name: "CoreWLANBridge",
            type: .static,
            targets: ["CoreWLANBridge"])
    ],
    targets: [
        .target(
            name: "CoreWLANBridge",
            path: "Sources/CoreWLANBridge")
    ]
)
