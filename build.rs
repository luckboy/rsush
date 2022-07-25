use std::env;
use std::path;
use std::process::Command;
use std::process::exit;

fn main()
{
    let profile = env::var("PROFILE").unwrap();
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let sep = path::MAIN_SEPARATOR;
    if profile == String::from("debug") {
        let target_dir = format!("{}{}target{}debug", cargo_manifest_dir, sep, sep);
        let rustc = env::var("RUSTC").unwrap_or(String::from("rustc"));
        let rustflags = env::var("RUSTFLAGS").unwrap_or(String::new());
        let rustflags_vec = rustflags.split_whitespace();
        let mut cmd = Command::new(rustc);
        cmd.args(rustflags_vec);
        cmd.arg("-o").arg(format!("{}{}rsush_test", target_dir, sep));
        cmd.arg(format!("{}{}src{}test_bin{}rsush_test.rs", cargo_manifest_dir, sep, sep, sep));
        let status = cmd.status().unwrap();
        println!("cargo:rerun-if-changed={}{}src{}test_bin{}rsush_test.rs", cargo_manifest_dir, sep, sep, sep);
        if status.success() {
            exit(0);
        } else {
            exit(1);
        }
    }
}
