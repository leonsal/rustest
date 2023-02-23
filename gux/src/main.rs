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


	// // Creates image with one white opaque pixel
	// var rect [1]RGBA
	// rect[0] = MakeColor(255, 255, 255, 255)
	//
	// // Creates and transfer 1 pixel opaque white texture needed for all commands
	// texId := win.CreateTexture(1, 1, &rect[0])
    let pixel = gb::make_color(255,255,255,255);
    let texid = gb::create_texture(win, 1, 1, &pixel);
    println!("texid:{texid}");

    let fparams = gb::FrameParams {
        ev_timeout: 0.0,
        clear_color: gb::Vec4 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
            w: 1.0,
        },
    };

    let mut draw_list1 = gb::DrawList::new();
    {
        let (cmd, idx_buf, vtx_buf) = draw_list1.new_cmd(6, 4);
        cmd.clip_rect = gb::Vec4{x:0., y:0., z: 4000., w:4000.};
        cmd.texid = texid;
        idx_buf[0] = 0;
        idx_buf[1] = 1;
        idx_buf[2] = 2;
        idx_buf[3] = 2;
        idx_buf[4] = 3;
        idx_buf[5] = 0;
        vtx_buf[0] = gb::Vertex{pos: gb::Vec2{x:10., y:10.}, uv: gb::Vec2{x:0., y:0.}, col: 0xFF_FF_00_00};
        vtx_buf[1] = gb::Vertex{pos: gb::Vec2{x:10., y:100.}, uv: gb::Vec2{x:0., y:0.}, col: 0xFF_FF_00_00};
        vtx_buf[2] = gb::Vertex{pos: gb::Vec2{x:200., y:100.}, uv: gb::Vec2{x:0., y:0.}, col: 0xFF_FF_00_00};
        vtx_buf[3] = gb::Vertex{pos: gb::Vec2{x:200., y:10.}, uv: gb::Vec2{x:0., y:0.}, col: 0xFF_FF_00_00};
    }
    {
        let (cmd, idx_buf, vtx_buf) = draw_list1.new_cmd(6, 4);
        cmd.clip_rect = gb::Vec4{x:0., y:0., z: 4000., w:4000.};
        cmd.texid = texid;
        idx_buf[0] = 0;
        idx_buf[1] = 1;
        idx_buf[2] = 2;
        idx_buf[3] = 2;
        idx_buf[4] = 3;
        idx_buf[5] = 0;
        vtx_buf[0] = gb::Vertex{pos: gb::Vec2{x:500., y:0.}, uv: gb::Vec2{x:0., y:0.}, col: 0xFF_00_00_FF};
        vtx_buf[1] = gb::Vertex{pos: gb::Vec2{x:500., y:250.}, uv: gb::Vec2{x:0., y:0.}, col: 0xFF_00_00_FF};
        vtx_buf[2] = gb::Vertex{pos: gb::Vec2{x:750., y:250.}, uv: gb::Vec2{x:0., y:0.}, col: 0xFF_00_00_FF};
        vtx_buf[3] = gb::Vertex{pos: gb::Vec2{x:750., y:0.}, uv: gb::Vec2{x:0., y:0.}, col: 0xFF_00_00_FF};
    }

	// {
	// 	cmd, idxBuf, vtxBuf := drawList1.NewDrawCmd(6, 4)
	// 	cmd.ClipRect = Vec4{0, 0, 4000, 4000}
	// 	cmd.TexID = texId
	// 	idxBuf[0] = 0
	// 	idxBuf[1] = 1
	// 	idxBuf[2] = 2
	// 	idxBuf[3] = 2
	// 	idxBuf[4] = 3
	// 	idxBuf[5] = 0
	// 	vtxBuf[0] = Vertex{Pos: Vec2{500, 0}, Col: 0xFF_00_00_FF}
	// 	vtxBuf[1] = Vertex{Pos: Vec2{500, 250}, Col: 0xFF_00_00_FF}
	// 	vtxBuf[2] = Vertex{Pos: Vec2{750, 250}, Col: 0xFF_00_00_FF}
	// 	vtxBuf[3] = Vertex{Pos: Vec2{750, 0}, Col: 0xFF_00_00_FF}
	// }

    loop {
        let frame_info = gb::window_start_frame(win, &fparams);
        if frame_info.win_close != 0 {
            break;
        }
        if frame_info.ev_count > 0 {
            println!("events:{}", frame_info.ev_count);
        }

        gb::window_render_frame(win, &draw_list1);
    }
    gb::window_destroy(win);
}
