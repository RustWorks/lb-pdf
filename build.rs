fn main() {
    let mut current_dir = std::env::current_dir().unwrap();

    if macos() {
        current_dir.push("pdfium-macos");
    } else if ios() {
        current_dir.push("pdfium-ios");
    } else if ios_sim() {
        current_dir.push("pdfium-ios-sim");
    } else if android() {
        todo!("@smail get android static lib from: https://github.com/paulocoutinhox/pdfium-lib")
    }

    let current_dir = current_dir.display();

    println!("cargo:rustc-link-lib=static=pdfium");
    println!("cargo:rustc-link-search=native={current_dir}");
}

fn macos() -> bool {
    std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos"
}

fn ios() -> bool {
    std::env::var("TARGET").unwrap() == "aarch64-apple-ios"
}

fn ios_sim() -> bool {
    std::env::var("TARGET").unwrap() == "aarch64-apple-ios-sim"
}

fn android() -> bool {
    std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "android"
}