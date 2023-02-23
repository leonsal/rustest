pub mod gb;

fn main() {
    println!("GUX");
    let cfg = gb::Config {
        debug_print_cmds: false,
        unlimited_rate: false,
        opengl: gb::ConfigOpenGL { es: false },
        vulkan: gb::ConfigVulkan {
            validation_layer: true,
        },
    };

    let win = gb::create_window("Gux".to_string(), 1000, 800, &cfg);
    println!("create_window():{:p}", win);

    let fparams = gb::FrameParams {
        ev_timeout: 0.0,
        clear_color: gb::Vec4 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
            w: 1.0,
        },
    };

    let dl = gb::DrawList {
        buf_cmd: std::ptr::null_mut(),
        cmd_count: 0,
        buf_idx: std::ptr::null_mut(),
        idx_count: 0,
        buf_vtx: std::ptr::null_mut(),
        vtx_count: 0,
    };

    loop {
        let frame_info = gb::window_start_frame(win, &fparams);
        if frame_info.win_close != 0 {
            break;
        }
        if frame_info.ev_count > 0 {
            println!("events:{}", frame_info.ev_count);
        }

        gb::window_render_frame(win, dl);
    }
    gb::window_destroy(win);
}
