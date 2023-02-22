pub mod gb;

fn main() {

    println!("GUX");
    let cfg = gb::Config {
        debug_print_cmds: true,
        unlimited_rate: false,
        opengl: gb::ConfigOpenGL {
            es: false,
        },
        vulkan: gb::ConfigVulkan {
            validation_layer: false,
        }
    };

    let win = gb::create_window("Gux".to_string(), 1000, 800, &cfg);

    let fparams = gb::FrameParams {
        ev_timeout: 0.0,
        clear_color: gb::Vec4{x:0.5, y:0.5, z:0.5, w:1.0},
    };

    loop {
        let frame_info = gb::window_start_frame(win, &fparams);
        println!("close:{}", frame_info.win_close);
        if frame_info.win_close != 0 {
            break;
        }

        //gb::window_render_frame(win, 0);
    }
    gb::window_destroy(win);

}
