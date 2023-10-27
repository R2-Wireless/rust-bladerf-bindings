extern crate bindgen;
extern crate cargo_metadata;

use cargo_metadata::{Message, MetadataCommand};
use std::error::Error;
use std::io::BufReader;
use std::process::{Command, Stdio};

fn main() {
    let out_dir = get_out_dir().expect("Couldn't find lib dir");
    let header_dir = out_dir
        .join("bladeRF")
        .join("host")
        .join("libraries")
        .join("libbladeRF")
        .join("include");
    let header_dir = header_dir.to_str().unwrap();
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("bindings.h")
        .clang_arg(format!("-I{}", &header_dir))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = std::path::PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn get_out_dir() -> Result<std::path::PathBuf, Box<dyn Error>> {
    let metadata = MetadataCommand::new().exec()?;
    let package = match metadata.root_package() {
        Some(p) => p,
        None => return Err("cargo out-dir must be run from within a crate".into()),
    };
    let mut command = Command::new("cargo")
        .args(&["check", "--message-format=json", "--quiet"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let reader = BufReader::new(command.stdout.take().unwrap());
    for message in Message::parse_stream(reader) {
        match message? {
            Message::BuildScriptExecuted(script) => {
                if script.package_id == package.id {
                    return Ok(script.out_dir);
                }
            }
            _ => (),
        }
    }
    Err(format!("crate {} did not run a build script", package.name).into())
}
