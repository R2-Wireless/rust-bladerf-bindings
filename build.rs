use std::process::Command;

static LIB_TAG: &str = "9fc57e6c7feabde04feebd62cc5b8bb83223728c";

fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let _ = Command::new("rm")
        .current_dir(&out_dir)
        .args(["-rf", "bladeRF"])
        .output();
    let _ = Command::new("mkdir")
        .current_dir(&out_dir)
        .args(["bladeRF"])
        .output()
        .expect("Wasn't able to build bladeRF");
    let _ = Command::new("git")
        .current_dir(out_dir.join("bladeRF"))
        .args(["init"])
        .output()
        .expect("Wasn't able to build bladeRF");
    let _ = Command::new("git")
        .current_dir(out_dir.join("bladeRF"))
        .args([
            "remote",
            "add",
            "origin",
            "https://github.com/Nuand/bladeRF.git",
        ])
        .output()
        .expect("Wasn't able to build bladeRF");
    let _ = Command::new("git")
        .current_dir(out_dir.join("bladeRF"))
        .args(["fetch", "--depth", "1", "origin", LIB_TAG])
        .output()
        .expect("Wasn't able to build bladeRF");
    let _ = Command::new("git")
        .current_dir(out_dir.join("bladeRF"))
        .args(["checkout", "FETCH_HEAD"])
        .output()
        .expect("Wasn't able to build bladeRF");

    let _ = Command::new("mkdir")
        .current_dir(out_dir.join("bladeRF").join("host"))
        .args(["-p", "build"])
        .output()
        .expect("Wasn't able to build bladeRF");
    let _ = Command::new("cmake")
        .current_dir(out_dir.join("bladeRF").join("host").join("build"))
        .args([
            "-DCMAKE_BUILD_TYPE=Release",
            "-DINSTALL_UDEV_RULES=ON",
            "-DENABLE_BACKEND_LIBUSB=TRUE",
            "../",
        ])
        .output()
        .expect("Wasn't able to build bladeRF");
    let _ = Command::new("make")
        .current_dir(out_dir.join("bladeRF").join("host").join("build"))
        .output()
        .expect("Wasn't able to build bladeRF");

    let lib_output = out_dir
        .join("bladeRF")
        .join("host")
        .join("build")
        .join("output");
    let lib_output = match lib_output.to_str() {
        Some(x) => x,
        _ => panic!("Should exist"),
    };
    println!("cargo:rustc-env=DYLD_LIBRARY_PATH={}", &lib_output);
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", &lib_output);
    println!("cargo:rustc-link-search={}", &lib_output);
    println!("cargo:rustc-link-lib=bladeRF");
}
