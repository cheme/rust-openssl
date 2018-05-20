use std::env;

fn main() {
    match env::var("DEP_OPENSSL_VERSION") {
        Ok(ref v) if v == "101" => {
            println!("cargo:rustc-cfg=ossl101");
            println!("cargo:rustc-cfg=ossl10x");
        }
        Ok(ref v) if v == "102" => {
            println!("cargo:rustc-cfg=ossl102");
            println!("cargo:rustc-cfg=ossl10x");
        }
        Ok(ref v) if v == "110" => {
            println!("cargo:rustc-cfg=ossl110");
        }
        Ok(ref v) if v == "111" => {
            println!("cargo:rustc-cfg=ossl110");
            println!("cargo:rustc-cfg=ossl111");
        }
        _ => panic!("Unable to detect OpenSSL version"),
    }

    if let Ok(_) = env::var("DEP_OPENSSL_LIBRESSL") {
        println!("cargo:rustc-cfg=libressl");
    }

    if let Ok(v) = env::var("DEP_OPENSSL_LIBRESSL_VERSION") {
        println!("cargo:rustc-cfg=libressl{}", v);
    }

    if let Ok(vars) = env::var("DEP_OPENSSL_CONF") {
        for var in vars.split(",") {
            println!("cargo:rustc-cfg=osslconf=\"{}\"", var);
        }
    }

    if let Ok(version) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&version, 16).unwrap();

        if version >= 0x1_01_00_07_0 {
            println!("cargo:rustc-cfg=ossl110g");
        }
    }

    if let Ok(version) = env::var("DEP_OPENSSL_LIBRESSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&version, 16).unwrap();

        if version >= 0x2_06_01_00_0 {
            println!("cargo:rustc-cfg=libressl261");
        }

        if version >= 0x2_07_00_00_0 {
            println!("cargo:rustc-cfg=libressl270");
        }
    }
}
