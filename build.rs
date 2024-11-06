use std::env;
use std::path::Path;

const NDDSHOME_ENVVAR: &'static str = "NDDSHOME";
const CONNEXTDDS_ARCH_ENVVAR: &'static str = "CONNEXTDDS_ARCH";
const ARCHSTR_DEFAULT: &'static str = "x64Linux4gcc7.3.0";

fn main() {
    // Read the environment variables
    let nddshome = env::var(NDDSHOME_ENVVAR).expect(&format!("{NDDSHOME_ENVVAR} envvar is not set"));
    let archstr = env::var(CONNEXTDDS_ARCH_ENVVAR).unwrap_or(ARCHSTR_DEFAULT.to_owned());

    // Ensure the path is valid
    let nddshome = Path::new(&nddshome)
        .canonicalize()
        .expect("Failed to get absolute path");
    if !nddshome.exists() {
        panic!("The path specified in {NDDSHOME_ENVVAR} does not exist");
    }

    println!("cargo:rustc-link-lib=dylib=nddscd");
    println!("cargo:rustc-link-search=native={}/lib/{archstr}", nddshome.display());
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}/lib/{archstr}", nddshome.display());
}
