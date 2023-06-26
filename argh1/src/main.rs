use argh::FromArgs;

fn default_frames() -> usize {
    0
}

#[derive(FromArgs)]
/// Reach new heights.
struct Options {
    /// whether or not to jump
    #[argh(switch, short = 'j')]
    jump: bool,

    /// how high to go
    #[argh(option)]
    height: usize,

    /// number of frames to run
    #[argh(option, default = "default_frames()")]
    frames: usize,

    /// an optional nickname for the pilot
    #[argh(option)]
    pilot_nickname: Option<String>,
}

fn main() {
    let ops: Options = argh::from_env();
    println!("Options.jump:{}", ops.jump);
    println!("Options.height:{}", ops.height);
    println!("Options.frames:{}", ops.frames);
    //println!("Options.pilot:{}", ops.pilot_nickname);
}
