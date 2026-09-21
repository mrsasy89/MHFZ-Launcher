# Windows build requirements (32-bit target)

The launcher's DLL-injection mechanism (`mhfo-hd.dll` loading via `LoadLibraryA`)
requires the launcher process itself to be built as **32-bit** (`i686-pc-windows-msvc`),
matching the architecture of the original 32-bit MHF client DLLs. Building with the
default 64-bit host target causes `LoadLibraryA` to fail with
`Os { code: 193, message: "%1 is not a valid Win32 application" }`.

Before building or testing on Windows:

```powershell
rustup target add i686-pc-windows-msvc
```

Then always use the dedicated 32-bit scripts, not the generic ones:

```powershell
npm run tauri:dev:win32
npm run tauri:build:win32
```

If linking fails afterward, ensure Visual Studio Build Tools has the
"MSVC v143 - VS 2022 C++ x64/x86 build tools" component installed
(the x86 linker is a separate sub-component from x64).
