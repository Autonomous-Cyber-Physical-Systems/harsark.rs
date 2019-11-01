use std::{
    env,
    error::Error,
    // fmt,
    path::PathBuf,
};

use cc::Build;

// #[derive(Debug)]
// struct TargetArchError(String);

// impl Error for TargetArchError {}
// impl fmt::Display for TargetArchError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "There is an error: {}", self.0)
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    let target: String = env::var("TARGET").unwrap();
    let asm_file: Option<String> = match target.as_str() {
        // "thumbv6m-none-eabi" => Some("thumbv6m-none-eabi.s".to_string()),
        // "thumbv7m-none-eabi" => Some("thumbv6m-none-eabi.s".to_string()),
        "thumbv7em-none-eabi" => Some("thumbv7em-none-eabi.s".to_string()),
        _ => None,
    };
    if let Some(ref file) = asm_file {
        Build::new().file(file).compile("asm");
    } else {
        // return Result::Err(Box::new(
        // 	TargetArchError(format!("Unsupported target {}", target).into())));
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=thumbv6m-none-eabi.s");
    println!("cargo:rerun-if-changed=thumbv7em-none-eabi");

    Ok(())
}
