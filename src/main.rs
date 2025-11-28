use ray_trace::*;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 200;
    const MAX_DEPTH: i32 = 100;

    // Animation
    const ROTATION_DEGREES: f64 = 360.0; // Full 360-degree rotation

    // Get number of frames from command-line argument
    let config = Config::from_args();
    let num_frames = config.num_frames;

    // World
    let world = random_scene();

    let intensity = 0.75;
    let lights = vec![
        PointLight::new(Point3::new(0.0, 5.0, 4.0), Color::new(1.0, 0.85, 0.6), intensity), // overhead lamp
    ];

    // --- Render Loop for Video ---
    for frame in 0..num_frames {
        // --- Calculate Camera Position for this frame ---
        let lookat = Point3::new(0.85, 0.2, 1.35);
        let vup = Point3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 6.0;
        let aperture = 0.02; //0.0 pinhole 0.05 noticably blur

        // Orbit parameters
        let radius = 6.5; // Distance from lookat point in the XZ plane
        let start_angle_rad = 0.46; // Initial angle to match the original view
        let angle_step = crate::core::common::degrees_to_radians(ROTATION_DEGREES) / num_frames as f64;
        let current_angle = start_angle_rad + frame as f64 * angle_step;

        let lookfrom = Point3::new(
            lookat.x() + radius * current_angle.cos(),
            3.5, // Keep camera height constant
            lookat.z() + radius * current_angle.sin()
        );

        let cam = Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            ASPECT_RATIO,
            aperture,
            dist_to_focus,
        );

        // Debug: print image and camera settings for verification
        eprintln!("DEBUG: IMAGE_WIDTH = {}, IMAGE_HEIGHT = {}, lookfrom_y = {}", IMAGE_WIDTH, IMAGE_HEIGHT, lookfrom.y());

        // --- Render a single frame ---
        let filename = format!("output/frame_{:03}.ppm", frame);
        eprintln!("\nRendering frame {}/{} to {}", frame + 1, num_frames, filename);
        let file = File::create(&filename).expect("Failed to create file.");
        let mut writer = BufWriter::new(file);

        writeln!(&mut writer, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT).expect("writing header");

        for j in (0..IMAGE_HEIGHT).rev() {
            eprint!("\rScanlines remaining: {} ", j);
            for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + crate::core::common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + crate::core::common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, &lights, MAX_DEPTH);
                }
                write_color(&mut writer, pixel_color, SAMPLES_PER_PIXEL);
            }
        }
    }

    eprint!("\nDone.\n");
}
