use winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    // Explicitly point to the windres executable for the gnu toolchain
    if std::env::var("TARGET").map_or(false, |t| t.ends_with("-gnu")) {
        res.set_windres_path("x86_64-w64-mingw32-windres");
    }
    res.compile().unwrap();
}