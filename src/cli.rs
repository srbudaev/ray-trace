use clap::Parser;

/// Ray tracer CLI configuration
#[derive(Parser, Debug)]
#[command(name = "ray-trace")]
#[command(
    about = "A ray tracer for rendering 3D scenes",
    long_about = "Ray tracer that supports multiple scenes with customizable rendering parameters.\n\nExamples:\n  cargo run -- --scene 1\n  cargo run -- --scene 1 --width 1920 --height 1080\n  cargo run -- --scene 1 --samples 50 --depth 10\n  cargo run -- --scene 1 --output my_scene.ppm"
)]
pub struct Config {
    /// Scene number to render
    /// 
    /// Available scenes:
    ///   0 = Random scene (billiard balls)
    ///   1 = Sphere scene
    ///   2 = Plane + Cube scene
    ///   3 = All objects scene (sphere, cube, cylinder, plane)
    ///   4 = All objects scene with alternative camera angle
    #[arg(short = 's', long = "scene", default_value_t = 0)]
    pub scene: usize,

    /// Number of frames to render (useful for animations)
    #[arg(short = 'f', long = "frames", default_value_t = 1)]
    pub num_frames: usize,

    /// Image width in pixels (default: 800)
    #[arg(short = 'w', long = "width")]
    pub width: Option<i32>,

    /// Image height in pixels (default: 600)
    #[arg(long = "height")]
    pub height: Option<i32>,

    /// Samples per pixel - higher values give better quality but slower rendering (default: 200)
    /// 
    /// Recommended values:
    ///   - Quick test: 10-50
    ///   - Normal quality: 100-200
    ///   - High quality: 500-1000
    #[arg(long = "samples")]
    pub samples: Option<i32>,

    /// Maximum ray depth - how many times a ray can bounce (default: 100)
    /// 
    /// Lower values render faster but may miss some reflections/refractions.
    /// Typical values: 10-100
    #[arg(long = "depth")]
    pub depth: Option<i32>,

    /// Output filename
    /// 
    /// For single frame: exact filename (e.g., "output.ppm")
    /// For multiple frames: base filename, frame number will be appended (e.g., "frame" -> "frame_000.ppm")
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,
}

impl Config {
    /// Parse configuration from command line arguments
    pub fn from_args() -> Self {
        Self::parse()
    }
}
