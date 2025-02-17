use std::env;
use std::path::PathBuf;

fn main() {
    let ndds_home = env::var("NDDSHome")
        .expect("NDDSHome environment variable is not set");
    
    println!("NDDSHome is: {}", ndds_home);
    
    let nddshome  = ndds_home;
    let include_path = format!("{}/include", nddshome);
    // Adding subdirectories
    let include_ndds = format!("{}/ndds", include_path);
    let include_osapi = format!("{}/osapi", include_ndds);
    let include_log = format!("{}/log", include_path);
    let include_dds_c = format!("{}/dds_c", include_ndds);
    let bindings = bindgen::Builder::default()
        .header(format!("{}/ndds/ndds_c.h", include_path))
        .clang_arg("-DRTI_UNIX")
        .clang_arg(format!("-I{}", include_path))
        .clang_arg(format!("-I{}", include_ndds))
        .clang_arg(format!("-I{}", include_osapi))
        .clang_arg(format!("-I{}", include_log))  // Add log directory
        .clang_arg(format!("-I{}", include_dds_c))
        // .clang_arg("-include")  // Force include core types header
        // .clang_arg(format!("{}/ndds/osapi/rtitypes.h", include_ndds))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native={}/lib", nddshome);
    println!("cargo:rustc-link-lib=nddsc");
    println!("cargo:rustc-link-lib=nddscore");
}