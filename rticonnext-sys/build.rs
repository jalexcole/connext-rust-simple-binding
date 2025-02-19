use std::env;
use std::path::PathBuf;

fn main() {

    let debug_enabled = match cfg!(debug_assertions) {
        true => "d",
        false => "",
    };
    
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


    // https://doc.rust-lang.org/beta/rustc/platform-support.html
    let rti_architecture = match target.as_str() {
        "aarch64-apple-darwin" => "arm64Darwin20clang12.0",
        "x86_64-apple-darwin" => "x64Darwin17clang9.0",
        "x86_64-unknown-linux-gnu" =>  "x64Linux4gcc7.3.0", // TODO: Only supports redhat. Will need to support all linux distros.
        "i686-unknown-linux-gnu" => todo!(),
        "aarch64-unknown-linux-gnu" => todo!(),
        "x86_64-pc-windows-msvc" => todo!(),
        "x86_64-pc-windows-gnu" => todo!(),
        "aarch64-pc-windows-msvc"=> todo!(),

        // Other Archetectures
        _ => panic!("Unsupported target: {}", target),
    };

    

    println!(
        "cargo:rustc-link-search=native={}/lib/{}",
        nddshome, rti_architecture
    );
    // TODO: While static compilation is preferred in rust, there needs to be a config flag to allow for dynamic linking.
    println!("cargo:rustc-link-lib=nddsc{debug_enabled}");
    println!("cargo:rustc-link-lib=nddscore{debug_enabled}");
    println!("cargo:rustc-link-lib=nddsmetp{debug_enabled}");
    println!("cargo:rustc-link-lib=nddssecurity{debug_enabled}");
    println!("cargo:rustc-link-lib=nddstransporttcp{debug_enabled}");
    println!("cargo:rustc-link-lib=rticonnextmsgc{debug_enabled}");
    println!("cargo:rustc-link-lib=rtiddsconnectorlua{debug_enabled}");
    println!("cargo:rustc-link-lib=rtidlc{debug_enabled}");
    println!("cargo:rustc-link-lib=rtimonitoring{debug_enabled}");
    println!("cargo:rustc-link-lib=rtipersistenceservice{debug_enabled}");
    println!("cargo:rustc-link-lib=rtirecordingservicecore{debug_enabled}");
    println!("cargo:rustc-link-lib=rtiroutingservice{debug_enabled}");
    println!("cargo:rustc-link-lib=rtirsinfrastructure{debug_enabled}");
    println!("cargo:rustc-link-lib=rtisqlite{debug_enabled}");
    println!("cargo:rustc-link-lib=rtistorageutils{debug_enabled}");
    println!("cargo:rustc-link-lib=rtixml2{debug_enabled}");
}
