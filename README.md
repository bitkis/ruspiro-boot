# Boot crate for the RusPiRo kernel

This crate provides basic boot code that - when build into a kernel crate - will be executed as soon as the Raspberry Pi powers up. As the building of this crate and finally linking into the kernel image depends on several linker symbols to be present it is recommended to use the linker script provided when bulding the kernel.

[![Travis-CI Status](https://api.travis-ci.org/RusPiRo/ruspiro-boot.svg?branch=master)](https://travis-ci.org/RusPiRo/ruspiro-boot)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-boot.svg)](https://crates.io/crates/ruspiro-boot)
[![Documentation](https://docs.rs/ruspiro-boot/badge.svg)](https://docs.rs/ruspiro-boot)
[![License](https://img.shields.io/crates/l/ruspiro-boot.svg)](https://github.com/RusPiRo/ruspiro-boot#license)

## Hint

The usage of this crate does only make sense when building a Raspberry Pi 3 bare metal kernel. The 
baremetal bootstrapping provided by this crate can be build for either Aarch32 or Aarch64 target
architectures. It has been verified to cross compile from a Windows host machine successfully for
both architectures and the execution is tested on a Raspberry Pi 3 B+.

## Features

Feature            | Purpose
-------------------|--------------------------
``with_panic``     | Implement a default panic handler
``singlecore``     | Keep all cores except one in a "parked" state so the runtime only uses a single core.
``ruspiro_pi3``    | This is passed to the dependend crates to ensure they will be build properly for this target device.

## Usage

To use this crate simply add the following lines to your ``Cargo.toml`` file:
(hint: git dependency as long as the crate is not registered at crates.io)

```toml
[dependencies]
ruspiro-boot = { version = "0.3", features = ["ruspiro_pi3", "with_panic"] }
```

In the main rust file refer to this crate with this:

```rust
#[macro_use]
extern crate ruspiro_boot;
```

The usage of `extern crate` is mandatory to ensure the boot strapping is properly linked into the
final binary.

To successfully build a bare metal binary using this crate for the boot strapping part it is **highly recomended** to use the linker script provided by this crate. Based on the target architecture to be built it is either [link32.ld](link32.ld) or [link64.ld](link64.ld).
To conviniently refer to the linker scripts contained in this crate it's recommended to use a specific build script in your project that copies the required file to your current project folder and could then be referred to with the ``RUSTFLAG`` ``-C link-arg=-T./link<aarch>.ld``.
The build script is a simple ``build.rs`` rust file in your project root with the following contents:

```rust
use std::{env, fs, path::Path};

fn main() {
    // copy the linker script from the boot crate to the current directory
    // so it will be invoked by the linker
    let ld_source = env::var_os("DEP_RUSPIRO_BOOT_LINKERSCRIPT")
        .expect("error in ruspiro build, `ruspiro-boot` not a dependency?")
        .to_str()
        .unwrap()
        .replace("\\", "/");
    let src_file = Path::new(&ld_source);
    let trg_file = format!(
        "{}/{}",
        env::current_dir().unwrap().display(),
        src_file.file_name().unwrap().to_str().unwrap()
    );
    println!("Copy linker script from {:?}, to {:?}", src_file, trg_file);
    fs::copy(src_file, trg_file).unwrap();
}
```

To get started you could check out the template projects [here](https://www.github.com/RusPiRo/ruspiro_templates)

## License

Licensed under Apache License, Version 2.0, ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)