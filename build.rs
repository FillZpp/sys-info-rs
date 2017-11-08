extern crate gcc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let target_os = target.split('-').nth(2).unwrap();

    match target_os {
        "linux" => gcc::compile_library("libinfo.a", &["c/linux.c"]),
        "darwin" => gcc::compile_library("libinfo.a", &["c/macos.c"]),
        "windows" => {
            gcc::compile_library("libinfo.a", &["c/windows.c"]);
            println!("cargo:rustc-flags=-l psapi");
        },
        _ => panic!("Unsupported system")
    };
}
