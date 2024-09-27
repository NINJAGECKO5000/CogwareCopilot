use std::{env, fs, process};

fn main() {
    println!("cargo:rustc-link-lib=static=EGL");
    println!("cargo:rustc-link-lib=static=GLESv2");
    println!("cargo:rustc-link-search=native=/home/ninja/Desktop/CogwareCopilot");
    let ld_script_path = match env::var("LD_SCRIPT_PATH") {
        Ok(var) => var,
        _ => process::exit(0),
    };

    let files = fs::read_dir(ld_script_path).unwrap();
    files
        .filter_map(Result::ok)
        .filter(|d| {
            if let Some(e) = d.path().extension() {
                e == "ld"
            } else {
                false
            }
        })
        .for_each(|f| println!("cargo:rerun-if-changed={}", f.path().display()));
}
