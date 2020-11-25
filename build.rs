
use std::io;
use std::fs;
use std::path::Path;

fn find_libzmq<P: AsRef<Path>>(path: P) -> io::Result<Option<String>> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        if let Some(name) = entry?.file_name().to_str() {
            if name.starts_with("libzmq") {
                if let Some(pos) = name.rfind(".") {
                    return Ok(Some((&name[..pos]).into()));
                }
            }
        }
    }
    Ok(None)
}


fn main() {
    let fldr = r"\Utils";
    let name = find_libzmq(fldr).expect("Could not read directory").expect("Not found!");
    println!("cargo:rustc-link-search={}", fldr);
    println!("cargo:rustc-link-lib={}", name);
}
