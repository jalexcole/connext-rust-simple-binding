use std::env;
use std::path::PathBuf;

fn main() {
    let debug = "d"; // TODO: Check if debug symbols are being used. This will be neccessary to switch between bindings.

    
    // let ndds_home = env::var("NDDSHome")
    //     .expect("NDDSHome environment variable is not set");

    // println!("NDDSHome is: {}", ndds_home);

    let nddshome = "/Applications/rti_connext_dds-6.1.1";
    let include_path = format!("{}/include", nddshome);
    // Adding subdirectories
    let include_ndds = format!("{}/ndds", include_path);
    let include_osapi = format!("{}/osapi", include_ndds);
    let include_log = format!("{}/log", include_path);
    let include_dds_c = format!("{}/dds_c", include_ndds);
    let bindings = bindgen::Builder::default()
        .header(format!("{}/ndds/ndds_c.h", include_path))
        .raw_line("#[allow(non_camel_case_types)]")
        .raw_line("#[allow(non_snake_case)]")
        .raw_line("#[allow(non_upper_case_globals)]")
        .raw_line("#[allow(dead_code)]")
        .raw_line("#[allow(improper_ctypes)]")
        .clang_arg("-DRTI_UNIX")
        .clang_arg(format!("-I{}", include_path))
        .clang_arg(format!("-I{}", include_ndds))
        .clang_arg(format!("-I{}", include_osapi))
        .clang_arg(format!("-I{}", include_log)) // Add log directory
        .clang_arg(format!("-I{}", include_dds_c))
        // .clang_arg("-include")  // Force include core types header
        // .clang_arg(format!("{}/ndds/osapi/rtitypes.h", include_ndds))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");



    let target = env::var("TARGET").unwrap();

    let natives = if target.contains("apple") && target.contains("x86") {
        "x64Darwin17clang9.0"
    } else if target.contains("linux") && target.contains("x86") {
        "x64Linux4gcc7.3.0"
    } else {
        panic!("Unsupported target: {}", target)
    };

    println!(
        "cargo:rustc-link-search=native={}/lib/{}",
        nddshome, natives
    );
    // TODO: While static compilation is preferred in rust, there needs to be a config flag to allow for dynamic linking.
    println!("cargo:rustc-link-lib=static=nddscz{debug}");
    println!("cargo:rustc-link-lib=static=nddscorez{debug}");
    println!("cargo:rustc-link-lib=static=nddsmetpz{debug}");
    println!("cargo:rustc-link-lib=static=nddssecurityz{debug}");
    println!("cargo:rustc-link-lib=static=nddstransporttcpz{debug}");
    println!("cargo:rustc-link-lib=static=rticonnextmsgcz{debug}");
    println!("cargo:rustc-link-lib=static=rtiddsconnectorluaz{debug}");
    println!("cargo:rustc-link-lib=static=rtidlcz{debug}");
    println!("cargo:rustc-link-lib=static=rtimonitoringz{debug}");
    println!("cargo:rustc-link-lib=static=rtipersistenceservicez{debug}");
    println!("cargo:rustc-link-lib=static=rtirecordingservicecorez{debug}");
    println!("cargo:rustc-link-lib=static=rtiroutingservicez{debug}");
    println!("cargo:rustc-link-lib=static=rtirsinfrastructurez{debug}");
    println!("cargo:rustc-link-lib=static=rtisqlitez{debug}");
    println!("cargo:rustc-link-lib=static=rtistorageutilsz{debug}");
    println!("cargo:rustc-link-lib=static=rtixml2z{debug}");
}
