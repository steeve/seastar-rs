extern crate cc;
extern crate pkg_config;

fn main() {
    pkg_config::Config::new()
        .statik(true)
        .probe("seastar")
        .unwrap();

    println!("cargo:rustc-link-search=native=/usr/local/lib64");
    println!("cargo:rustc-link-lib=static=seastar");

    for lib in vec!["boost_program_options", "boost_thread", "cares", "cryptopp", "fmt"] {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    cc::Build::new()
        .cpp(true)
        .flag("-Wno-unused-result")
        .flag("-std=gnu++17")
        .file("src/seastar.cc")
        .compile("seastar-sys");
}
