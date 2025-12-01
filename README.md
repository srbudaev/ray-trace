# RT

A physically-based ray tracer written in Rust that renders 3D scenes with realistic lighting, shadows, and materials.

## Features

- **Multiple geometric primitives**: Spheres, planes, cuboids, and cylinders
- **Advanced materials**: Lambertian (diffuse), metal (reflective), striped patterns, and Perlin noise textures
- **Realistic lighting**: Point lights with soft shadows and specular highlights
- **Camera controls**: Adjustable field of view, depth of field, and positioning
- **Customizable rendering**: Configurable resolution, samples per pixel, and ray depth
- **Multiple pre-built scenes**: From simple geometry tests to complex billiard ball arrangements

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo

### Build

```bash
# Clone the repository
git clone <https://github.com/srbudaev/ray-trace.git>
cd ray-trace

# Build the project
cargo build --release
```

## Usage

### Basic Command

```bash
cargo run --release -- [OPTIONS]⬇️
```

### CLI Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--scene <NUMBER>` | `-s` | Scene number to render (0-4) | 0 |
| `--width <PIXELS>` | `-w` | Image width in pixels | 800 |
| `--height <PIXELS>` | | Image height in pixels | 600 |
| `--samples <NUMBER>` | | Samples per pixel (quality) | 200 |
| `--depth <NUMBER>` | | Maximum ray bounce depth | 100 |
| `--output <FILE>` | `-o` | Output filename | output.ppm |
| `--frames <NUMBER>` | `-f` | Number of frames (for animations) | 1 |

### Available Scenes

- **Scene 0**: Random scene with billiard balls on a textured plane
- **Scene 1**: Simple sphere test scene
- **Scene 2**: Plane with a cube
- **Scene 3**: All objects scene (sphere, cube, cylinder, plane)
- **Scene 4**: All objects with alternative camera angle

### Examples

#### Render the default scene
```bash
cargo run --release -- --scene 0
```

#### Render a specific scene with custom output
```bash
cargo run --release -- --scene 3 --output my_render.ppm
```

#### High-quality render
```bash
cargo run --release -- --scene 4 --width 1920 --height 1080 --samples 500 --depth 50
```

#### Quick test render (lower quality, faster)
```bash
cargo run --release -- --scene 1 --width 400 --height 300 --samples 10 --depth 5
```

#### Render all scenes
```bash
for i in {0..4}; do
    cargo run --release -- --scene $i --output scene_$i.ppm
done
```

### Quality Settings Guide

**Samples per pixel** controls image quality vs render time:
- **10-50**: Quick tests, noisy images
- **100-200**: Normal quality, good balance
- **500-1000**: High quality, smooth images, slow

**Max depth** controls ray bounces:
- **5-10**: Fast, may miss some reflections
- **50-100**: Recommended for most scenes
- **100+**: Diminishing returns, very slow

## Project Structure

```
ray-trace/
├── src/
│   ├── main.rs              # Entry point and scene selection
│   ├── lib.rs               # Module exports
│   ├── cli.rs               # Command-line argument parsing
│   ├── scenes.rs            # Scene definitions
│   ├── ppm.rs               # PPM image format output
│   ├── core/
│   │   ├── camera.rs        # Camera implementation
│   │   ├── hittable.rs      # Ray-object intersection trait
│   │   ├── hittable_list.rs # Collection of hittable objects
│   │   ├── light.rs         # Point light implementation
│   │   ├── material.rs      # Material properties and scattering
│   │   └── common.rs        # Utility functions
│   ├── math/
│   │   ├── vec3.rs          # 3D vector mathematics
│   │   └── ray.rs           # Ray implementation
│   ├── scene/
│   │   ├── scene.rs         # Scene management
│   │   └── render.rs        # Ray tracing algorithm
│   └── shapes/
│       ├── sphere.rs        # Sphere primitive
│       ├── plane.rs         # Plane primitive
│       ├── cuboid.rs        # Cuboid/box primitive
│       └── cylinder.rs      # Cylinder primitive
├── Cargo.toml
└── README.md
```

## Adding Custom Scenes

To add your own scene, edit `src/scenes.rs`:

```rust
pub fn scene_my_custom_scene() -> Scene {
    let mut scene = Scene::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    scene.add_object(Box::new(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_material,
    )));

    let sphere_material = Rc::new(Lambertian::new(Color::new(0.8, 0.2, 0.2)));
    scene.add_object(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        0.5,
        sphere_material,
    )));

    scene.set_camera(
        Point3::new(5.0, 3.0, 5.0),    
        Point3::new(0.0, 1.0, 0.0),   
        Vec3::new(0.0, 1.0, 0.0),     
        30.0,                          
        0.02,                    
        6.0, 
    );

    scene.add_light(PointLight::new(
        Point3::new(0.0, 5.0, 4.0),
        Color::new(1.0, 0.85, 0.6),
        0.75,
    ));

    scene
}
```

Then register it in `src/main.rs` and `src/lib.rs`.

## Camera Controls

The camera is controlled via `scene.set_camera()` with these parameters:

- **lookfrom** (`Point3`): Camera position in 3D space
- **lookat** (`Point3`): Point the camera is looking at
- **vup** (`Vec3`): Up direction (typically `(0, 1, 0)`)
- **vfov** (`f64`): Vertical field of view in degrees (20-60 typical)
- **aperture** (`f64`): Lens aperture for depth of field (0.01-0.1)
- **focus_dist** (`f64`): Focus distance

### Camera Examples

```rust
// Top-down view
scene.set_camera(
    Point3::new(0.0, 10.0, 0.0),
    Point3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, 0.0, -1.0),
    45.0, 0.02, 10.0,
);

// Close-up with wide FOV
scene.set_camera(
    Point3::new(2.0, 1.0, 2.0),
    Point3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    60.0, 0.02, 3.0,
);

// Orbiting view
let angle = 45.0_f64.to_radians();
let radius = 8.0;
scene.set_camera(
    Point3::new(radius * angle.cos(), 3.0, radius * angle.sin()),
    Point3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    20.0, 0.02, 6.0,
);
```

## Output Format

The ray tracer outputs images in PPM (Portable Pixmap) format. To convert to other formats:

```bash
# Convert to PNG using ImageMagick
convert output.ppm output.png

# Convert to JPEG
convert output.ppm output.jpg

# View directly (macOS)
open output.ppm

# View directly (Linux)
xdg-open output.ppm
```

## Performance Tips

1. **Use release mode**: Always use `--release` for production renders
2. **Start with low samples**: Test with 10-50 samples before final render
3. **Reduce resolution**: Test at 400x300 or 800x600 before full resolution
4. **Adjust depth**: Most scenes work fine with depth 10-50
5. **Profile renders**: Time different settings to find the sweet spot

## Dependencies

- `rand` - Random number generation
- `clap` - Command-line argument parsing
