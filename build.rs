extern crate gcc;

use std::env;

fn main() {
    if cfg!(target_os = "linux") {
        let key = "TARGET";
		match env::var_os(key) {
			Some(val) => {
                let target = val.into_string().unwrap();
                if target.contains("windows") {
                    gcc::compile_library("libinfo.a", &["c/windows.c"]);
                } else {
                   gcc::compile_library("libinfo.a", &["c/linux.c"]); 
                }
            },
			None => gcc::compile_library("libinfo.a", &["c/linux.c"]),
		}
    } else if cfg!(target_os = "macos") {
        gcc::compile_library("libinfo.a", &["c/macos.c"]);
    } else if cfg!(target_os = "windows") {
        gcc::compile_library("libinfo.a", &["c/windows.c"]);
        println!("cargo:rustc-flags=-l psapi");
    } else {
        panic!("Unsupported system");
    }
}
