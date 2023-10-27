use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=lib-tag");
    let lib_tag =
        std::fs::read_to_string("lib-tag").expect("Should have been able to read the file");
    let lib_tag = lib_tag.trim();
    let _ = Command::new("rm").args(["-rf", "bladeRF"]).output();
    let _ = Command::new("mkdir")
        .args(["bladeRF"])
        .output()
        .expect("Wasn't able to build bladeRF");
    let _ = Command::new("git")
        .current_dir("bladeRF")
        .args(["init"])
        .output()
        .expect("Wasn't able to build bladeRF");
    let _ = Command::new("git")
        .current_dir("bladeRF")
        .args([
            "remote",
            "add",
            "origin",
            "https://github.com/Nuand/bladeRF.git",
        ])
        .output()
        .expect("Wasn't able to build bladeRF");
    let _ = Command::new("git")
        .current_dir("bladeRF")
        .args(["fetch", "--depth", "1", "origin", lib_tag])
        .output()
        .expect("Wasn't able to build bladeRF");
    let _ = Command::new("git")
        .current_dir("bladeRF")
        .args(["checkout", "FETCH_HEAD"])
        .output()
        .expect("Wasn't able to build bladeRF");

    let _ = Command::new("mkdir")
        .current_dir("bladeRF/host")
        .args(["-p", "build"])
        .output()
        .expect("Wasn't able to build bladeRF");
    let _ = Command::new("cmake")
        .current_dir("bladeRF/host/build")
        .args(["../"])
        .output()
        .expect("Wasn't able to build bladeRF");
    let _ = Command::new("make")
        .current_dir("bladeRF/host/build")
        .output()
        .expect("Wasn't able to build bladeRF");

    println!("cargo:rustc-env=DYLD_LIBRARY_PATH=./bladeRF/host/build/output");
    println!("cargo:rustc-env=LD_LIBRARY_PATH=./bladeRF/host/build/output");
    println!("cargo:rustc-link-search=./bladeRF/host/build/output");
    println!("cargo:rustc-link-lib=bladeRF");
}
