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
    ///   5 = Custom scene (edit in src/scenes.rs)
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

    /// Background color (RGB values 0.0-1.0, space-separated)
    /// 
    /// Example: --bg 0.5 0.7 1.0 for sky blue
    /// Default: sky blue (0.5, 0.7, 1.0)
    #[arg(long = "bg", value_parser = parse_color, default_value = "0.5 0.7 1.0")]
    pub bg_color: (f64, f64, f64),

    /// Add spheres to custom scene (x y z radius color: r g b [glass: ior] [matte: opacity])
    /// 
    /// Example: --sphere "0 1 0 0.5 color: 0.8 0.3 0.3"
    /// Glass example: --sphere "0 1 0 0.5 color: 1 1 1 glass: 1.5"
    /// Matte translucent: --sphere "0 1 0 0.5 color: 0.8 0.6 0.9 matte: 0.3"
    /// Can be specified multiple times to add multiple spheres
    #[arg(long = "sphere", value_parser = parse_sphere)]
    pub spheres: Vec<SphereSpec>,

    /// Camera position for custom scene (x y z)
    /// 
    /// Example: --camera-pos 5 3 6
    /// Default: (5, 3, 6)
    #[arg(long = "camera-pos", value_parser = parse_vec3)]
    pub camera_pos: Option<(f64, f64, f64)>,

    /// Camera look-at point for custom scene (x y z)
    /// 
    /// Example: --camera-lookat 0 0.8 0
    /// Default: (0, 0.8, 0)
    #[arg(long = "camera-lookat", value_parser = parse_vec3)]
    pub camera_lookat: Option<(f64, f64, f64)>,
}

/// Sphere specification: position (x,y,z), radius, color (r,g,b)
#[derive(Debug, Clone)]
pub struct SphereSpec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f64,
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub material: MaterialType,
}

#[derive(Debug, Clone)]
pub enum MaterialType {
    Lambertian,
    Glass { index_of_refraction: f64 },
    Translucent { opacity: f64 },
}

/// Parse a color string in format "r g b" where each component is 0.0-1.0
fn parse_color(s: &str) -> Result<(f64, f64, f64), String> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(format!("Expected 3 space-separated values, got {}. Example: 0.5 0.7 1.0", parts.len()));
    }
    
    let r = parts[0].parse::<f64>()
        .map_err(|e| format!("Invalid red component: {}", e))?;
    let g = parts[1].parse::<f64>()
        .map_err(|e| format!("Invalid green component: {}", e))?;
    let b = parts[2].parse::<f64>()
        .map_err(|e| format!("Invalid blue component: {}", e))?;
    
    if !(0.0..=1.0).contains(&r) || !(0.0..=1.0).contains(&g) || !(0.0..=1.0).contains(&b) {
        return Err("Color components must be between 0.0 and 1.0".to_string());
    }
    
    Ok((r, g, b))
}

/// Parse a sphere specification: x y z radius color: r g b [glass: ior] [matte: opacity]
fn parse_sphere(s: &str) -> Result<SphereSpec, String> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    
    // Can be 8 parts (lambertian), 10 parts (glass or matte)
    if parts.len() != 8 && parts.len() != 10 {
        return Err(format!(
            "Expected 8 values (x y z radius color: r g b), or 10 values (x y z radius color: r g b glass: ior / matte: opacity), got {}. Example: 0 1 0 0.5 color: 0.8 0.3 0.3",
            parts.len()
        ));
    }
    
    let x = parts[0].parse::<f64>()
        .map_err(|e| format!("Invalid x coordinate: {}", e))?;
    let y = parts[1].parse::<f64>()
        .map_err(|e| format!("Invalid y coordinate: {}", e))?;
    let z = parts[2].parse::<f64>()
        .map_err(|e| format!("Invalid z coordinate: {}", e))?;
    let radius = parts[3].parse::<f64>()
        .map_err(|e| format!("Invalid radius: {}", e))?;
    
    // Check for "color:" keyword
    if parts[4] != "color:" {
        return Err(format!(
            "Expected 'color:' keyword at position 5, got '{}'. Example: 0 1 0 0.5 color: 0.8 0.3 0.3",
            parts[4]
        ));
    }
    
    let r = parts[5].parse::<f64>()
        .map_err(|e| format!("Invalid red component: {}", e))?;
    let g = parts[6].parse::<f64>()
        .map_err(|e| format!("Invalid green component: {}", e))?;
    let b = parts[7].parse::<f64>()
        .map_err(|e| format!("Invalid blue component: {}", e))?;
    
    if radius <= 0.0 {
        return Err("Radius must be positive".to_string());
    }
    
    if !(0.0..=1.0).contains(&r) || !(0.0..=1.0).contains(&g) || !(0.0..=1.0).contains(&b) {
        return Err("Color components must be between 0.0 and 1.0".to_string());
    }
    
    let material = if parts.len() == 10 {
        if parts[8] == "glass:" {
            // Clear glass
            let ior = parts[9].parse::<f64>()
                .map_err(|e| format!("Invalid index of refraction: {}", e))?;
            if ior < 1.0 {
                return Err("Index of refraction must be >= 1.0".to_string());
            }
            MaterialType::Glass { index_of_refraction: ior }
        } else if parts[8] == "matte:" {
            // Matte translucent
            let opacity = parts[9].parse::<f64>()
                .map_err(|e| format!("Invalid opacity: {}", e))?;
            if !(0.0..=1.0).contains(&opacity) {
                return Err("Opacity must be between 0.0 and 1.0".to_string());
            }
            MaterialType::Translucent { opacity }
        } else {
            return Err(format!(
                "Expected 'glass:' or 'matte:' keyword at position 9, got '{}'. Example: 0 1 0 0.5 color: 1 1 1 matte: 0.3",
                parts[8]
            ));
        }
    } else {
        MaterialType::Lambertian
    };
    
    Ok(SphereSpec { x, y, z, radius, r, g, b, material })
}

/// Parse a 3D vector: x y z
fn parse_vec3(s: &str) -> Result<(f64, f64, f64), String> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(format!("Expected 3 space-separated values (x y z), got {}. Example: 5 3 6", parts.len()));
    }
    
    let x = parts[0].parse::<f64>()
        .map_err(|e| format!("Invalid x coordinate: {}", e))?;
    let y = parts[1].parse::<f64>()
        .map_err(|e| format!("Invalid y coordinate: {}", e))?;
    let z = parts[2].parse::<f64>()
        .map_err(|e| format!("Invalid z coordinate: {}", e))?;
    
    Ok((x, y, z))
}

impl Config {
    /// Parse configuration from command line arguments
    pub fn from_args() -> Self {
        Self::parse()
    }
}
