use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::process::Command;

#[test]
fn test() {
    // clean environment
    fs::remove_file(Path::new("tests").join("foo").join("Cargo.lock"))
        .expect("failed to delete tests/foo/Cargo.lock");
    fs::remove_dir_all(Path::new("tests").join("foo").join("target"))
        .expect("failed to delete tests/foo/target/");

    // build test crate (tests inputs and meta())
    assert!(Command::new("cargo")
                .arg("build")
                .args(&["--features", "feat"])
                .current_dir(Path::new("tests").join("foo"))
                .status().expect("failed to execute cargo build")
                .success());

    // check build script output (tests the rest of the outputs)
    let dep_build_dir = env::current_dir().unwrap()
                                          .join("tests")
                                          .join("foo")
                                          .join("target")
                                          .join("debug")
                                          .join("build");
    let mut build_script_output = String::new();
    for entry in fs::read_dir(dep_build_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() && path.file_name().and_then(|p| p.to_str()).map(|s| s.starts_with("foo-")) == Some(true) {
            let _ = File::open(path.join("output")).map(|mut f| f.read_to_string(&mut build_script_output).unwrap());
        }
    }
    assert_eq!(build_script_output.trim(), r#"
cargo:rustc-link-lib=dylib=whatever
cargo:rustc-link-search=all=wherever
cargo:rustc-flags=-l however
cargo:rustc-cfg=whoever
cargo:rerun-if-changed=whichever
cargo:warning=success
    "#.trim());
}

