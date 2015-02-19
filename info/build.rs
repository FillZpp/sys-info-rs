extern crate gcc;

fn main() {
    if cfg!(target_os = "linux") {
        gcc::compile_library("libinfo.a", &["c/linux.c"]);
    } else if cfg!(target_os = "macos") {
        gcc::compile_library("libinfo.a", &["c/macos.c"]);
    } else if cfg!(target_os = "windows") {
        gcc::compile_library("libinfo.a", &["c/windows.c"]);
    } else {
        panic!("Unsupported system");
    }
}
