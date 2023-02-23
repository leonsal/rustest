use libc::c_int;
use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

// Vector with 4 components
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
pub struct ConfigOpenGL {
    pub es: bool,
}

#[repr(C)]
pub struct ConfigVulkan {
    pub validation_layer: bool,
}

#[repr(C)]
pub struct Config {
    pub debug_print_cmds: bool,
    pub unlimited_rate: bool,
    pub opengl: ConfigOpenGL,
    pub vulkan: ConfigVulkan,
}

#[repr(C)]
pub struct Window {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct FrameParams {
    pub ev_timeout: f32,
    pub clear_color: Vec4,
}

// Single generic event
#[repr(C)]
pub struct Event {
    pub ev_type: u32,
    pub argint: [i32; 4],
    pub argfloat: [f32; 2],
}

#[repr(C)]
pub struct FrameInfo {
    pub win_close: u32,
    pub win_size: Vec2,
    pub fb_size: Vec2,
    pub fb_scale: Vec2,
    pub ev_cap: u32,
    pub ev_count: u32,
    pub events: *const Event,
}

type RGBA = u32;

// Vertex info
#[repr(C)]
pub struct Vertex {
    pub pos: Vec2,
    pub uv: Vec2,
    pub col: RGBA,
}

type Texid = usize;

#[repr(C)]
pub struct DrawCmd {
    pub clip_rect: Vec4,
    pub texid: Texid,
    pub idx_offset: u32,
    pub vtx_offset: u32,
    pub elem_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DrawList {
    pub buf_cmd: *mut DrawCmd,
    pub cmd_count: u32,
    pub buf_idx: *mut u32,
    pub idx_count: u32,
    pub buf_vtx: *mut Vertex,
    pub vtx_count: u32,
}

// Cursor types
#[repr(C)]
pub enum Cursor {
    Default,
    Arrow,
    IBeam,
    Crosshair,
    Hand,
    HResize,
    VResise,
    _Count,
}

extern "C" {
    fn gb_create_window(
        title: *const c_char,
        width: c_int,
        height: c_int,
        pcfg: &Config,
    ) -> &mut Window;
    fn gb_window_destroy(win: *mut Window);
    fn gb_window_start_frame(win: *mut Window, params: &FrameParams) -> &FrameInfo;
    fn gb_window_render_frame(win: *mut Window, dl: DrawList);
    fn gb_set_cursor(win: *mut Window, cursor: Cursor);
    fn gb_create_texture(win: *mut Window, width: c_int, height: c_int, data: *const RGBA)
        -> Texid;
    fn gb_delete_texture(win: *mut Window, texid: Texid);
}

pub fn create_window(title: String, width: i32, height: i32, cfg: &Config) -> *mut Window {
    unsafe {
        let ctitle = CString::new(title).expect("CString::new failed");
        gb_create_window(ctitle.as_ptr(), width, height, cfg)
    }
}

pub fn window_destroy(win: *mut Window) {
    unsafe {
        gb_window_destroy(win);
    }
}

pub fn window_start_frame(win: *mut Window, params: &FrameParams) -> &FrameInfo {
    unsafe { gb_window_start_frame(win, params) }
}

pub fn window_render_frame(win: *mut Window, dl: DrawList) {
    unsafe {
        gb_window_render_frame(win, dl);
    }
}

pub fn window_set_cursor(win: *mut Window, cursor: Cursor) {
    unsafe {
        gb_set_cursor(win, cursor);
    }
}

pub fn create_texture(win: *mut Window, width: i32, height: i32, data: &RGBA) -> Texid {
    unsafe {
        gb_create_texture(win, width, height, data)
    }
}

pub fn delete_texture(win: *mut Window, texid: Texid) {
    unsafe {
        gb_delete_texture(win, texid);
    }
}



