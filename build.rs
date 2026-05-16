fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/ffi/corewlan_shim.m");

    cc::Build::new()
        .file("src/ffi/corewlan_shim.m")
        .flag("-fobjc-arc")
        .compile("corewlan_shim");

    println!("cargo:rustc-link-lib=framework=CoreWLAN");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=objc");
}
