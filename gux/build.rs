
fn main() {

    // Creates builder for gux library
    let mut buildc = cc::Build::new();
    buildc.flag_if_supported("-O2");
    buildc.flag_if_supported("-Wno-unused-parameter");
    buildc.flag_if_supported("-Wno-missing-field-initializers");
    buildc.flag_if_supported("-Wno-sign-compare");

    let src_dir = "src/".to_string();
    let mut libs: Vec<String> = Vec::new();

    // GLFW Common files
    buildc.include(src_dir.clone() + "glfw/include");
    buildc.file(src_dir.clone() + "glfw/src/context.c");
    buildc.file(src_dir.clone() + "glfw/src/init.c");
    buildc.file(src_dir.clone() + "glfw/src/input.c");
    buildc.file(src_dir.clone() + "glfw/src/monitor.c");
    buildc.file(src_dir.clone() + "glfw/src/vulkan.c");
    buildc.file(src_dir.clone() + "glfw/src/window.c");
    buildc.file(src_dir.clone() + "glfw/src/osmesa_context.c");

    // GLFW files for Linux
    #[cfg(target_os = "linux")]
    {
        buildc.file(src_dir.clone() + "glfw/src/linux_joystick.c");
        buildc.file(src_dir.clone() + "glfw/src/posix_time.c");
        buildc.file(src_dir.clone() + "glfw/src/posix_thread.c");
        buildc.file(src_dir.clone() + "glfw/src/xkb_unicode.c");
        buildc.file(src_dir.clone() + "glfw/src/egl_context.c");
    }

    // GLFW files for Linux && X11
    #[cfg(target_os = "linux")]
    {
        //#cgo linux,!wayland LDFLAGS: -lX11 -lXrandr -lXxf86vm -lXi -lXcursor -lm -lXinerama -ldl -lrt
        buildc.define("_GLFW_X11", None);
        buildc.file(src_dir.clone() + "glfw/src/x11_window.c");
        buildc.file(src_dir.clone() + "glfw/src/x11_init.c");
        buildc.file(src_dir.clone() + "glfw/src/x11_monitor.c");
        buildc.file(src_dir.clone() + "glfw/src/glx_context.c");
        libs.push("X11".to_string());
        libs.push("Xrandr".to_string());
        libs.push("Xi".to_string());
        libs.push("Xinerama".to_string());
    }

    // Gux files for vulkan
    #[cfg(feature = "vulkan")]
    {
        buildc.file(src_dir.clone() + "gb/volk.c");
        buildc.file(src_dir.clone() + "gb/glfw_vulkan.c");
    }

    // Gux files for opengl (default)
    #[cfg(not(feature = "vulkan"))]
    {
        buildc.include(src_dir.clone() + "gl3w/include");
        buildc.file(src_dir.clone() + "gb/glfw_opengl.c");
        buildc.file(src_dir.clone() + "gl3w/src/gl3w.c");
    }

    // Files to be compiled for windows
    #[cfg(target_os = "windows")]
    {}

    // Compile and generates library and AFTER requests additional external libraries to be linked.
    buildc.compile("gux");
    for l in &libs {
        link_lib(l);
    }
}

fn link_lib(name: &str) {
    println!("cargo:rustc-link-lib={name}");
}
