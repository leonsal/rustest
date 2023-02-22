use libc::c_int;
use std::os::raw::c_char;

#[repr(C)]
struct GBConfigOpenGL {
    es: bool
}

#[repr(C)]
struct GBConfigVulkan {
    validation_layer: bool
}

#[repr(C)]
struct GBConfig {
    debug_print_cmds: bool,
    unlimited_rate: bool,
    opengl: GBConfigOpenGL,
    vulkan: GBConfigVulkan,
}

extern "C" {
    fn gb_create_window(title: *const c_char, width: c_int, height: c_int, pcfg: &GBConfig);
//    gb_create_window(title: *const c_char, width: c_int, height: c_int, pcfg: &GBConfig);
 //   gb_window_destroy(&mut GBWindow);
 //    fn foo_none();
 //    fn foo_sum_ints(a: c_int, b: c_int) -> c_int;
 //    fn foo_sum_vec(pv: *const c_int, count: size_t) -> c_int;
 //    fn foo_print_points(points: *const Point, count: size_t);
 //    fn foo_mult_points(points: *mut Point, count: size_t);
	// fn cbar_sum_vec(v: *const c_int, count: size_t) -> c_int;
}

// gb_window_t gb_create_window(const char* title, int width, int height, gb_config_t* pcfg);
// void gb_window_destroy(gb_window_t win);
// gb_frame_info_t* gb_window_start_frame(gb_window_t bw, gb_frame_params_t* params);
// void gb_window_render_frame(gb_window_t win, gb_draw_list_t dl);
// void gb_set_cursor(gb_window_t win, int cursor);
// gb_texid_t gb_create_texture(gb_window_t win, int width, int height, const gb_rgba_t* data);
// void gb_delete_texture(gb_window_t win, gb_texid_t texid);



