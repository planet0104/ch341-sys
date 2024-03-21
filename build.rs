fn main() {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        println!("cargo:rustc-link-lib=static=CH341DLLA64");
        println!("cargo:rustc-link-search=native=LIB/CH341/amd64");
    }
    #[cfg(all(target_os = "windows", target_arch = "i686"))]
    {
        println!("cargo:rustc-link-lib=static=CH341DLL");
        println!("cargo:rustc-link-search=native=LIB/CH341/i386");
    }

    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    {
        println!("cargo:rustc-link-lib=static=CH341DLLA64");
        println!("cargo:rustc-link-search=native=LIB/CH341/arm64");
    }
}
