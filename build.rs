// build.rs

use bootloader::DiskImageBuilder;
use std::{env, path::PathBuf};

fn main() {
    let kernel_path = env::var("CARGO_BIN_FILE_KERNEL").unwrap();
    let builder = DiskImageBuilder::new(PathBuf::from(kernel_path));
    let uefi_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("kernel-uefi.img");

    builder.create_uefi_image(&uefi_path).unwrap();

    println!("cargo:rustc-env=UEFI_IMAGE={}", uefi_path.display());
}
