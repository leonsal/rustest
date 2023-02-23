use libc::c_int;
use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Clone,Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

// Vector with 4 components
#[repr(C)]
#[derive(Clone,Copy, Default)]
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
#[derive(Clone,Default)]
pub struct Vertex {
    pub pos: Vec2,
    pub uv: Vec2,
    pub col: RGBA,
}

type Texid = usize;

#[repr(C)]
#[derive(Clone,Copy)]
pub struct DrawCmd {
    pub clip_rect: Vec4,
    pub texid: Texid,
    pub idx_offset: u32,
    pub vtx_offset: u32,
    pub elem_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct DrawListC {
    buf_cmd: *const DrawCmd,
    cmd_count: u32,
    buf_idx: *const u32,
    idx_count: u32,
    buf_vtx: *const Vertex,
    vtx_count: u32,
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

// DrawList contains lists of commands and buffers for the graphics backend
pub struct DrawList {
 	buf_cmd: Vec<DrawCmd>, // Buffer with draw commands
 	buf_idx: Vec<u32>,
 	buf_vtx: Vec<Vertex>,
 	path:    Vec<Vec2>,
 }

impl DrawList {

    pub fn new() -> DrawList {
        DrawList{
            buf_cmd: Vec::new(),
            buf_idx: Vec::new(),
            buf_vtx: Vec::new(),
            path: Vec::new(),
        }
    }

    //pub fn new_cmd(&self, idx_count: u32, vtx_count: u32) -> (DrawCmd, &[u32], &[Vertex]) {
    pub fn new_cmd<'a>(&'a mut self, idx_count: u32, vtx_count: u32) -> (&mut DrawCmd, &mut[u32], &mut[Vertex]) {

        // Reserve space for indices
	    let idx_offset = self.buf_idx.len();
        self.buf_idx.resize(idx_offset + idx_count as usize, 0);

        // Reserve space for vertices
        let vtx_offset = self.buf_vtx.len();
        self.buf_vtx.resize(vtx_offset + vtx_count as usize, Vertex::default());

	    // Creates and appends new command to the DrawList command buffer
	    let cmd = DrawCmd {
            clip_rect: Vec4{x:0., y:0., z:10000., w:10000.},
            texid: 0,
            idx_offset: idx_offset as u32,
            vtx_offset: vtx_offset as u32,
            elem_count: idx_count,
        };
        self.buf_cmd.push(cmd);

        // Reference 
        let last_cmd = self.buf_cmd.len()-1;
        (&mut self.buf_cmd[last_cmd],
         &mut self.buf_idx[idx_offset..idx_offset + idx_count as usize], 
         &mut self.buf_vtx[vtx_offset..vtx_offset + vtx_count as usize],
         )
    }

    pub fn add_list(&mut self, src: &DrawList) {

        // Append indices
        let idx_offset = self.buf_idx.len();
        self.buf_idx.extend_from_slice(&src.buf_idx);

        // Append vertices
        let vtx_offset = self.buf_vtx.len();
        self.buf_vtx.extend_from_slice(&src.buf_vtx);

        // Append commands adjusting offset
        for c in &src.buf_cmd {
            let mut cmd = *c;
            cmd.idx_offset += idx_offset as u32;
            cmd.vtx_offset += vtx_offset as u32;
            self.buf_cmd.push(cmd);
        }
    }


// // AddList appends the specified DrawList to this one.
// // The added DrawList is not modified.
// func (dl *DrawList) AddList(src *DrawList) {
//
// 	// Append vertices
// 	vtxOffset := len(dl.bufVtx)
// 	dl.bufVtx = append(dl.bufVtx, src.bufVtx...)
//
// 	// Append indices
// 	idxOffset := len(dl.bufIdx)
// 	dl.bufIdx = append(dl.bufIdx, src.bufIdx...)
//
// 	// Append commands adjusting offsets
// 	for i := 0; i < len(src.bufCmd); i++ {
// 		cmd := src.bufCmd[i]
// 		cmd.idxOffset += uint32(idxOffset)
// 		cmd.vtxOffset += uint32(vtxOffset)
// 		dl.bufCmd = append(dl.bufCmd, cmd)
// 	}
// }

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
    fn gb_window_render_frame(win: *mut Window, dl: DrawListC);
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

pub fn window_render_frame(win: *mut Window, dl: &DrawList) {

    unsafe {
        let dlc = DrawListC {
            buf_cmd: dl.buf_cmd.as_ptr(),
            cmd_count: dl.buf_cmd.len() as u32,
            buf_idx: dl.buf_idx.as_ptr(),
            idx_count: dl.buf_idx.len() as u32,
            buf_vtx: dl.buf_vtx.as_ptr(),
            vtx_count: dl.buf_vtx.len() as u32,
        };
        gb_window_render_frame(win, dlc);
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

pub fn make_color(r: u8, g: u8, b: u8, a: u8) -> RGBA {

    (a as u32)<<24 | (b as u32) <<16 | (g as u32) << 8 | r  as u32
}


