use std::env;
use std::path::Path;
use std::process::Command;

const NDDSHOME_ENVVAR: &'static str = "NDDSHOME";
const CONNEXTDDS_ARCH_ENVVAR: &'static str = "CONNEXTDDS_ARCH";
const ARCHSTR_DEFAULT: &'static str = "x64Linux4gcc7.3.0";
const IDL: &'static str = "res/hello.idl";

fn main() {
    println!("cargo::rerun-if-changed={}", IDL);
    
    // Read the environment variables
    let nddshome =
        env::var(NDDSHOME_ENVVAR).expect(&format!("{NDDSHOME_ENVVAR} envvar is not set"));
    let archstr = env::var(CONNEXTDDS_ARCH_ENVVAR).unwrap_or(ARCHSTR_DEFAULT.to_owned());
    let outdir = env::var("OUT_DIR").unwrap();

    // Ensure the path is valid
    let nddshome = Path::new(&nddshome)
        .canonicalize()
        .expect("Failed to get absolute path");
    if !nddshome.exists() {
        panic!("The path specified in {NDDSHOME_ENVVAR} does not exist");
    }

    // Run rtiddsgen
    if !Command::new(format!("{}/bin/rtiddsgen", nddshome.display()))
        .arg("-language")
        .arg("C")
        .arg("-d")
        .arg(&outdir)
        .arg(IDL)
        .status()
        .expect("rtiddsgen command failed to start")
        .success()
    {
        panic!("rtiddsgen failed to execute")
    }

    // Build the generated files
    cc::Build::new()
        .include(format!("{}/include", nddshome.display()))
        .include(format!("{}/include/ndds", nddshome.display()))
        .include(&outdir)
        .define("RTI_LINUX", None)
        .define("RTI_UNIX", None)
        .file(format!("{outdir}/hello.c"))
        .file(format!("{outdir}/helloPlugin.c"))
        .file(format!("{outdir}/helloSupport.c"))
        .compile("libtypesupport.a");

    // Add DDS libraries
    println!("cargo::rustc-link-lib=dylib=nddscd");
    println!("cargo::rustc-link-lib=dylib=nddscored");
    println!(
        "cargo::rustc-link-search=native={}/lib/{archstr}",
        nddshome.display()
    );
    println!(
        "cargo::rustc-link-arg=-Wl,-rpath,{}/lib/{archstr}",
        nddshome.display()
    );
}
