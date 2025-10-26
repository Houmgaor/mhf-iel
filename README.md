# MHF IELess Launcher

MHF default launcher requires IE to login. IE sucks.

This project reverse engineered the MHF launcher in order to make it possible to boot the game directly, without going through `mhf.exe` and `mhl.dll`.

If you're wondering 'Why use this instead of the original launcher?', here are some of the issues that are are solved by using a custom launcher:

- Not being locked to IE.
  - Should open a sea of possibilities on how to design the launcher.
  - Won't take 10 seconds for each request.
  - Game might boot under Linux/Steam deck when using Proton/Wine, since IE was the main reason those weren't even options.
- Not being locked to the weird way MHF connects to the server.
  - Allows launcher operations to be implemented using with HTTP(S), JSON, custom ports, etc.
- Not being locked to the operations and data model the original launcher uses.
  - Allows implementing new operations, such as adding separate buttons for 'Sign Up' and 'Login'.
  - Allows storing and displaying extra information. For example, it would be *possible* to get character portraits on the launcher window.
  - Removes the need to modify the launcher (since we're replacing it) and `mhfo-hd.dll` to remove GameGuard, since `mhfo-hd.dll` calls a function provided by the launcher to run GameGuard checks.

## Usage

If calling from another Rust project, make sure it itself is targeting `nightly-i686-pc-windows-msvc`, and just call `run` with the correct parameters. The idea at the moment is that most of these parameters will be returned from the [signv2server](https://github.com/ZeruLight/Erupe/tree/main/server/signv2server) endpoints, but this might change in the future.

You can also use the [CLI interface](mhf-iel-cli/README.md) to run this project from any other program, and without the `i686` limitation.

Feel free to create a ticket if you need another way to integrate this lib into your app (`.dll`, bindings for static linking, etc).

## Build

The Windows build is recommended as it has a more direct setup, but you can use the Linux build as weel.

First, add the build target:

```bash
rustup target add i686-pc-windows-msvc
```

### On Windows (recommended)

Simply build:

```bash
cargo build  # (optional) build the .rlib file
cargo build --package mhf-iel-cli --release  # Build the CLI .exe
```

### On Linux (cross-compile)

**Recommended:** Use `cargo-xwin` to build the MSVC target (best compatibility):

```bash
cargo install cargo-xwin xwin
xwin --accept-license --arch x86 splat --output ~/.cache/cargo-xwin/xwin
cargo xwin build --package mhf-iel-cli --target i686-pc-windows-msvc --release
```

**Alternative:** Use GNU target (may have compatibility issues with the game's MSVC DLLs):

```bash
cargo install cross
rustup target add i686-pc-windows-gnu
cross build --package mhf-iel-cli --target i686-pc-windows-gnu --release
```

The output is in `target/i686-pc-windows-msvc/release/` or `target/i686-pc-windows-gnu/release/`
