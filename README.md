# Nexus CodeLLDB adapter

Nexus builds and distributes the native adapter from this fork. LLVM/LLDB is an
external toolchain dependency; the adapter package does not contain another LLVM
installation or a Python runtime. Launch it with the chosen `liblldb` path through
the adapter's `--liblldb` argument.

## Build

Use Rust 1.91.0, CMake 3.21 or newer, Ninja, and the selected LLVM toolchain.
Windows additionally needs the MSVC libraries, Windows SDK, and DIA SDK.
Run from a configured development environment with Cargo and LLVM tools on PATH.

```sh
cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Release \
  -DLLVM_ROOT=/path/to/llvm -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++
cmake --build build --target adapter
cmake --build build --target check
cmake --build build --target package
```

On Windows, select `clang-cl` for both C and C++ compilers. The Windows adapter
requires LLVM 23's `SBFile::OpenFdFromHandle` API. LLVM itself is not built here.
If the LLVM installation lacks LLDB headers, set `LLVM_SOURCE_DIR` to matching
upstream sources. Only their headers and the upstream API-header generator are
used. Alternatively, supply `LLDB_INCLUDE_DIRS` explicitly. `LLDB_LIBRARY` can
override the shared-library location within the selected toolchain.

The `check` target runs the Rust unit suites. The upstream DAP integration test
sources remain under `tests`; they are separate from these unit checks.

CPack produces `nexus-codelldb-<platform>.zip`, containing the adapter, launcher,
support scripts, license, and (on Windows) the DIA library. Use `PLATFORM_ID` to
set the package platform name. There is no full or bootstrap VSIX build in this
fork; the Nexus extension owns VS Code integration.

The CI workflow provisions LLVM 23 as a build prerequisite: the Windows 23.1.4
release and LLVM's official Ubuntu APT packages. That toolchain is not uploaded
with the adapter artifacts.

## Adapter features
- Conditional breakpoints, function breakpoints, logpoints,
- Hardware data access breakpoints (watchpoints),
- Launch debuggee in integrated or external terminal,
- Disassembly view with instruction-level stepping,
- [Step Into Targets](https://code.visualstudio.com/updates/v1_46#_step-into-targets).
- Caller exclusion for breakpoints.
- Memory view.
- Loaded modules view,
- Python scripting,
- HTML rendering for advanced visualizations,
- Workspace-level defaults for launch configurations,
- Remote debugging,
- Reverse debugging (experimental, requires a compatible backend).

For full details please see [User's Manual](MANUAL.md).<br>

# Languages
The primary focus of this project are the C++ and Rust languages, for which CodeLLDB includes built-in visualizers for
vectors, strings, maps, and other standard library types.<br>
That said, it is usable with most other compiled languages whose compiler generates compatible debugging information,
such as Ada, Fortran, Kotlin Native, Nim, Objective-C, Pascal, [Swift](https://github.com/vadimcn/codelldb/wiki/Swift)
and Zig.

# Supported Platforms

## Host
- Linux with glibc 2.18+ for x86_64, aarch64 or armhf.
- MacOS 10.12+ for x86_64 and 11.0+ for arm64.
- Windows 10 and 11 for x86_64. [See Windows notes in wiki!](https://github.com/vadimcn/codelldb/wiki/Windows)

## Target
CodeLLDB supports AArch64, ARM, AVR, MSP430, RISCV, X86 architectures and may be used to debug on embedded platforms
via [remote debugging](MANUAL.md#remote-debugging).

# More information
- [CodeLLDB User's Manual](MANUAL.md) - how to use this extension.
- [Debugging in VS Code](https://code.visualstudio.com/docs/editor/debugging) - if you are new to VSCode debugging.
- [LLDB Tutorial](https://lldb.llvm.org/use/tutorial.html) - all of LLDB's CLI commands and scripting features may be used in CodeLLDB.
- [Wiki pages](https://github.com/vadimcn/codelldb/wiki) - [troubleshooting](https://github.com/vadimcn/codelldb/wiki/Troubleshooting) and other tips and tricks.
- [Discussions](https://github.com/vadimcn/codelldb/discussions) - for questions and discussions.

# Screenshots

C++ debugging with data visualization ([Howto](https://github.com/vadimcn/codelldb/wiki/Data-visualization)):<br>
![source](images/plotting.png)
<br>
<br>
Rust debugging:<br>
![source](images/source.png)


