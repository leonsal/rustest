
fn main() {

    // Creates builder for c library
    let mut buildc = cc::Build::new();
    let srcdir = "src/cfiles/".to_string();

    // Files to be compiled for Linux
    #[cfg(target_os="linux")]
    {
        buildc.define("WAYLAND", None);
        buildc.file(srcdir.clone() + "foo.c");
    }

    // Files to be compiled for windows
    #[cfg(target_os="windows")]
    {
        buildc.file(srcdir + "lixo.c");
    }

    // Builds static lib
    buildc.compile("foo");

    // Creates builder for cpp library
    let mut buildcpp = cc::Build::new();
    buildcpp.cpp(true);
    buildcpp.file(srcdir + "bar.cpp");
    buildcpp.compile("bar");
}
