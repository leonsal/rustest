fn main() {

    // Creates builder for glfw library
    let mut buildc = cc::Build::new();
    let src_dir = "src/".to_string();

    // Common files
    buildc.include(src_dir.clone() + "glfw/include");
    buildc.file(src_dir.clone() + "glfw/src/context.c");
    buildc.file(src_dir.clone() + "glfw/src/init.c");
    buildc.file(src_dir.clone() + "glfw/src/input.c");
    buildc.file(src_dir.clone() + "glfw/src/monitor.c");
    buildc.file(src_dir.clone() + "glfw/src/vulkan.c");
    buildc.file(src_dir.clone() + "glfw/src/window.c");
    buildc.file(src_dir.clone() + "glfw/src/osmesa_context.c");

    // Files to be compiled for Linux
    #[cfg(target_os="linux")]
    {
        buildc.file(src_dir.clone() + "glfw/src/linux_joystick.c");
        buildc.file(src_dir.clone() + "glfw/src/posix_time.c");
        buildc.file(src_dir.clone() + "glfw/src/posix_thread.c");
        buildc.file(src_dir.clone() + "glfw/src/xkb_unicode.c");
        buildc.file(src_dir.clone() + "glfw/src/egl_context.c");
    }

    // Files to be compiled for Linux && X11
    #[cfg(target_os="linux")]
    {
        buildc.define("_GLFW_X11", None);
        buildc.file(src_dir.clone() + "glfw/src/x11_window.c");
        buildc.file(src_dir.clone() + "glfw/src/x11_init.c");
        buildc.file(src_dir.clone() + "glfw/src/x11_monitor.c");
        buildc.file(src_dir.clone() + "glfw/src/glx_context.c");
    }

    // Files to be compiled for Linux && OpenGL
    #[cfg(target_os="linux")]
    {
        buildc.include(src_dir.clone() + "gl3w/include");
        buildc.file(src_dir.clone() + "gb/glfw_opengl.c");
        buildc.file(src_dir.clone() + "gl3w/src/gl3w.c");
    }

    // Files to be compiled for windows
    #[cfg(target_os="windows")]
    {
    }

    // Builds static lib
    buildc.compile("glfw");
}

