# RT

A physically-based ray tracer written in Rust that renders 3D scenes with realistic lighting, shadows, and materials.

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

| Option               | Short | Description                       | Default    |
| -------------------- | ----- | --------------------------------- | ---------- |
| `--scene <NUMBER>`   | `-s`  | Scene number to render (0-4)      | 0          |
| `--width <PIXELS>`   | `-w`  | Image width in pixels             | 800        |
| `--height <PIXELS>`  |       | Image height in pixels            | 600        |
| `--samples <NUMBER>` |       | Samples per pixel (quality)       | 200        |
| `--depth <NUMBER>`   |       | Maximum ray bounce depth          | 100        |
| `--output <FILE>`    | `-o`  | Output filename                   | output.ppm |
| `--frames <NUMBER>`  | `-f`  | Number of frames (for animations) | 1          |

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


## How to Create Elements (Objects)

### Step-by-Step Guide to Adding Objects

All scene elements are created in `src/scenes.rs`. Here's how to create each type of object:

#### 1. Creating a Sphere

```rust
use std::rc::Rc;

// Create material (choose one):
let material = Rc::new(Lambertian::new(Color::new(0.8, 0.2, 0.2))); 
let material = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.1, center, spot_dir, NumberType::Circle));

// Add sphere to scene
scene.add_object(Box::new(Sphere::new(
    Point3::new(0.0, 1.0, 0.0),  
    0.5,                         
    material,                
)));
```

**Parameters:**

- `Point3::new(x, y, z)` - Center position of the sphere
- `radius` - Size of the sphere
- `material` - Surface properties (color, reflectivity)

#### 2. Creating a Plane

```rust
let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

scene.add_object(Box::new(Plane::new(
    Point3::new(0.0, 0.0, 0.0), 
    Vec3::new(0.0, 1.0, 0.0),   
    ground_material,
)));
```

**Parameters:**

- First `Point3` - Any point on the plane (usually origin for ground)
- `Vec3` - Normal vector (0,1,0 for horizontal ground, 1,0,0 for vertical wall, etc.)

#### 3. Creating a Cuboid (Box)

```rust
let cube_material = Rc::new(Lambertian::new(Color::new(0.4, 0.6, 0.8)));

scene.add_object(Box::new(Cuboid::new(
    Point3::new(-0.5, 0.0, -0.5),  
    Point3::new(0.5, 1.0, 0.5),   
    cube_material,
)));
```

**Parameters:**

- First `Point3` - Minimum corner coordinates
- Second `Point3` - Maximum corner coordinates
- The cube is axis-aligned (cannot be rotated)

#### 4. Creating a Cylinder

```rust
let cylinder_material = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.2)));

scene.add_object(Box::new(Cylinder::new(
    Point3::new(1.5, 0.0, 0.0),   
    Vec3::new(0.0, 1.0, 0.0),     
    0.5,                           
    2.0,                         
    cylinder_material,
)));
```

**Parameters:**

- First `Point3` - Center of the bottom circle
- `Vec3` - Direction vector (axis of cylinder)
- `radius` - Radius of the cylinder
- `height` - Height along the axis

### Material Types

```rust
// Diffuse (matte) surface
Rc::new(Lambertian::new(Color::new(r, g, b)))

// Textured diffuse with noise pattern
Rc::new(LambertianNoise::new(Color::new(r, g, b), scale))

// Reflective metal surface
Rc::new(Metal::new(
    Color::new(r, g, b),  
    0.1,                   
    center,                
    spot_dir,              
    NumberType::Circle     
))

// Striped pattern (for billiard balls)
Rc::new(Striped::new(Color::new(r, g, b), fuzz, center, NumberType::Line))
```

**Color values** are RGB from 0.0 to 1.0:

- `Color::new(1.0, 0.0, 0.0)` - Pure red
- `Color::new(0.0, 1.0, 0.0)` - Pure green
- `Color::new(0.0, 0.0, 1.0)` - Pure blue
- `Color::new(1.0, 1.0, 1.0)` - White
- `Color::new(0.5, 0.5, 0.5)` - Gray

### Complete Scene Example

```rust
pub fn scene_my_custom_scene() -> Scene {
    let mut scene = Scene::new();

    // Add ground plane
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    scene.add_object(Box::new(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_material,
    )));

    // Add red sphere
    let sphere_material = Rc::new(Lambertian::new(Color::new(0.8, 0.2, 0.2)));
    scene.add_object(Box::new(Sphere::new(
        Point3::new(-1.5, 0.5, 0.0),
        0.5,
        sphere_material,
    )));

    // Add blue cube
    let cube_material = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 0.8)));
    scene.add_object(Box::new(Cuboid::new(
        Point3::new(-0.3, 0.0, -0.3),
        Point3::new(0.3, 0.8, 0.3),
        cube_material,
    )));

    // Add green cylinder
    let cylinder_material = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.2)));
    scene.add_object(Box::new(Cylinder::new(
        Point3::new(1.5, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.4,
        1.2,
        cylinder_material,
    )));

    // Set camera (see Camera section below)
    scene.set_camera(
        Point3::new(5.0, 3.0, 5.0),
        Point3::new(0.0, 0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        30.0,
        0.02,
        6.0,
    );

    // Add light (see Lighting section below)
    scene.add_light(PointLight::new(
        Point3::new(0.0, 5.0, 4.0),
        Color::new(1.0, 0.85, 0.6),
        0.75,
    ));

    scene
}
```

**Then register the scene:**

1. In `src/lib.rs`, add to exports:

```rust
pub use scenes::{..., scene_my_custom_scene};
```

2. In `src/main.rs`, add to match statement:

```rust
let mut scene = match config.scene {
    // ... existing scenes
    5 => scene_my_custom_scene(),
    _ => { ... }
};
```

3. Update `src/cli.rs` documentation to list the new scene number.

## How to Move the Camera

The camera position and orientation are set using `scene.set_camera()` in your scene function.

### Camera Parameters Explained

```rust
scene.set_camera(
    lookfrom,    // Point3: Camera position (x, y, z)
    lookat,      // Point3: Point the camera looks at (x, y, z)
    vup,         // Vec3: Up direction (usually (0, 1, 0))
    vfov,        // f64: Field of view in degrees (20-60 typical)
    aperture,    // f64: Lens aperture for depth of field
    focus_dist,  // f64: Focus distance
);
```

**Parameter Details:**

1. **lookfrom** (`Point3::new(x, y, z)`) - Where the camera is positioned

   - x: left (-) / right (+)
   - y: down (-) / up (+)
   - z: back (-) / forward (+)

2. **lookat** (`Point3::new(x, y, z)`) - The point the camera aims at

   - Usually the center of your scene or main object

3. **vup** (`Vec3::new(0.0, 1.0, 0.0)`) - Which way is "up" for the camera

   - Almost always `(0.0, 1.0, 0.0)` for normal upright view
   - Change for tilted or inverted views

4. **vfov** - Vertical field of view angle in degrees

   - 20° - Narrow, telephoto lens effect
   - 30-40° - Normal view
   - 60-90° - Wide angle, fish-eye effect

5. **aperture** - Controls depth of field blur

   - 0.0 - Everything in focus (pinhole camera)
   - 0.02-0.05 - Slight blur (typical)
   - 0.1+ - Strong blur, only focus point is sharp

6. **focus_dist** - Distance at which objects are in perfect focus
   - Usually the distance from lookfrom to lookat

### Camera Movement Examples

#### Basic Positions

```rust
// Front view - looking from the front
scene.set_camera(
    Point3::new(0.0, 2.0, 5.0),    // Camera in front
    Point3::new(0.0, 1.0, 0.0),    // Looking at center
    Vec3::new(0.0, 1.0, 0.0),      // Up is up
    30.0, 0.02, 5.0,
);

// Side view - looking from the right
scene.set_camera(
    Point3::new(5.0, 2.0, 0.0),    // Camera on the side
    Point3::new(0.0, 1.0, 0.0),    // Looking at center
    Vec3::new(0.0, 1.0, 0.0),
    30.0, 0.02, 5.0,
);

// Top-down view - looking straight down
scene.set_camera(
    Point3::new(0.0, 10.0, 0.0),   // Camera directly above
    Point3::new(0.0, 0.0, 0.0),    // Looking down at origin
    Vec3::new(0.0, 0.0, -1.0),     // Forward is "up" (important!)
    45.0, 0.02, 10.0,
);

// Diagonal view - classic 3/4 view
scene.set_camera(
    Point3::new(5.0, 3.0, 5.0),    // Camera at angle
    Point3::new(0.0, 1.0, 0.0),    // Looking at center
    Vec3::new(0.0, 1.0, 0.0),
    30.0, 0.02, 7.0,
);
```

#### Advanced Camera Positions

```rust
// Close-up with wide field of view
scene.set_camera(
    Point3::new(2.0, 1.0, 2.0),    // Close to object
    Point3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    60.0,                           // Wide FOV for drama
    0.05,                           // More blur
    2.5,                            // Focus close
);

// Low angle (looking up)
scene.set_camera(
    Point3::new(3.0, 0.5, 3.0),    // Camera low to ground
    Point3::new(0.0, 2.0, 0.0),    // Looking up at objects
    Vec3::new(0.0, 1.0, 0.0),
    40.0, 0.02, 4.0,
);

// Orbiting view (circular path around scene)
let angle = 45.0_f64.to_radians();  // Change angle to orbit
let radius = 8.0;                    // Distance from center
let height = 3.0;                    // Camera height
scene.set_camera(
    Point3::new(
        radius * angle.cos(),        // X position on circle
        height,                      // Y position (constant)
        radius * angle.sin()         // Z position on circle
    ),
    Point3::new(0.0, 1.0, 0.0),     // Always look at center
    Vec3::new(0.0, 1.0, 0.0),
    20.0, 0.02, 6.0,
);
```

### Quick Camera Movement Guide

**To move camera closer/further:**

- Change the `lookfrom` values to be closer/further from `lookat`
- Or change the `vfov` (larger = wider view = looks further)

**To change viewing angle:**

- Keep `lookat` the same, move `lookfrom` around it in a circle
- Example: `(5, 2, 0)` vs `(0, 2, 5)` vs `(-5, 2, 0)` - all look at origin from different sides

**To look at a different object:**

- Change `lookat` to the object's position
- Adjust `focus_dist` to the distance between camera and object

**To create depth of field effect:**

- Increase `aperture` (0.1 or higher)
- Make sure `focus_dist` matches the distance to your subject

## How to Change Brightness (Lighting)

Brightness in the ray tracer is controlled by light sources. You can adjust intensity, position, and color of lights.

### Adding Light Sources

```rust
scene.add_light(PointLight::new(
    Point3::new(0.0, 5.0, 4.0),   // Light position (x, y, z)
    Color::new(1.0, 0.85, 0.6),   // Light color (r, g, b)
    0.75,                          // Intensity (brightness)
));
```

### Light Parameters

1. **Position** (`Point3::new(x, y, z)`) - Where the light is located

   - Higher y values = light comes from above
   - Position affects shadows and how objects are lit

2. **Color** (`Color::new(r, g, b)`) - Color tint of the light

   - `(1.0, 1.0, 1.0)` - Pure white light
   - `(1.0, 0.85, 0.6)` - Warm/yellowish light (like sunlight)
   - `(0.6, 0.8, 1.0)` - Cool/bluish light (like moonlight)
   - Values from 0.0 to 1.0

3. **Intensity** (0.0 to 1.0+) - How bright the light is
   - `0.3` - Dim, low brightness (like scene 2)
   - `0.75` - Normal brightness (like most scenes)
   - `1.0` - Full brightness
   - `1.5+` - Very bright (can over-expose)

### Brightness Examples

```rust
// Dim lighting (moody scene)
scene.add_light(PointLight::new(
    Point3::new(0.0, 5.0, 4.0),
    Color::new(1.0, 0.85, 0.6),
    0.3,                            // Low intensity
));

// Normal bright lighting
scene.add_light(PointLight::new(
    Point3::new(0.0, 5.0, 4.0),
    Color::new(1.0, 1.0, 1.0),     // White light
    0.75,                           // Medium-high intensity
));

// Very bright lighting (high-key)
scene.add_light(PointLight::new(
    Point3::new(0.0, 5.0, 4.0),
    Color::new(1.0, 1.0, 1.0),
    1.2,                            // High intensity
));

// Colored lighting (blue moonlight)
scene.add_light(PointLight::new(
    Point3::new(-3.0, 8.0, 2.0),
    Color::new(0.5, 0.6, 1.0),     // Blue tint
    0.6,
));

// Warm sunset lighting
scene.add_light(PointLight::new(
    Point3::new(5.0, 3.0, 5.0),
    Color::new(1.0, 0.5, 0.3),     // Orange/red tint
    0.8,
));
```

### Multiple Light Sources

You can add multiple lights for more complex lighting:

```rust
// Main light (key light)
scene.add_light(PointLight::new(
    Point3::new(3.0, 5.0, 4.0),
    Color::new(1.0, 0.9, 0.7),
    0.8,
));

// Fill light (softer, from other side)
scene.add_light(PointLight::new(
    Point3::new(-2.0, 3.0, 3.0),
    Color::new(0.7, 0.8, 1.0),
    0.4,
));

// Back light (rim lighting)
scene.add_light(PointLight::new(
    Point3::new(0.0, 4.0, -3.0),
    Color::new(1.0, 1.0, 1.0),
    0.3,
));
```

### Tips for Adjusting Brightness

**To make scene brighter overall:**

- Increase light intensity value (0.75 → 1.0)
- Add additional light sources
- Move lights closer to objects
- Use whiter light colors

**To make scene darker/moodier:**

- Decrease light intensity (0.75 → 0.3)
- Use fewer lights
- Use colored/tinted lights instead of white
- Position lights further from objects

**To change lighting mood:**

- Warm (cozy): `Color::new(1.0, 0.85, 0.6)` with intensity 0.7
- Cool (stark): `Color::new(0.8, 0.9, 1.0)` with intensity 0.6
- Dramatic: Single strong light from one side, intensity 1.0
- Soft: Multiple weak lights from different angles, intensity 0.3-0.5 each

## Output Format

The ray tracer outputs images in PPM (Portable Pixmap) format. To convert to other formats:

```bash
# View directly (macOS)
open output.ppm
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

## example scenes:

## random_scene with warm light
cargo run --release -- --scene 0 --bg "0.6 0.26 0.07" --width 800 --height 600

daylight:
cargo run --release -- --scene 0 --bg "0.9 0.9 1.0" --width 800 --height 600

## scene_01
light:
cargo run --release -- --scene 1 --bg "0.9 0.9 0.9" --width 800 --height 600
darker:
cargo run --release -- --scene 1 --bg "0.1 0.1 0.1" --width 800 --height 600

## scene_02

lighter:
cargo run --release -- --scene 2 --width 800 --height 600 // default background: light blue

darker:
cargo run --release -- --scene 2 --bg "0.1 0.1 0.2" --width 800 --height 600

## scene_03
lighter:
cargo run --release -- --scene 3 --width 800 --height 600 // default background: light blue

darker:
cargo run --release -- --scene 3 --bg "0.1 0.1 0.2" --width 800 --height 600

## scene_04 

cargo run --release -- --scene 4 --bg "0.1 0.1 0.2" --width 800 --height 600

