use std::{env, fs};

fn main() {
    let uefi_target = env::current_exe().unwrap().with_file_name("uefi.img");
    fs::copy(env!("UEFI_IMAGE"), &uefi_target).unwrap();

    println!("UEFI disk image at: {}", uefi_target.display());
}
