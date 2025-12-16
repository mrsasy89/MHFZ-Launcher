// Helper Windows che fa DLL injection come mhf-iel
// Compila con: cargo build --target i686-pc-windows-gnu

use std::ffi::CString;

#[cfg(target_os = "windows")]
use winapi::um::libloaderapi::LoadLibraryA;

fn main() {
    // TODO: Implement mhf-iel logic
    // 1. Parse args (token, server, etc.)
    // 2. LoadLibrary("mhfo-hd.dll")
    // 3. Inject memory blobs
    // 4. Call entry point
}
