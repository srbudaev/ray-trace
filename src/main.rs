use ray_trace::*;
use ray_trace::scenes::{random_scene, scene_sphere, scene_plane_cube, scene_all_objects, scene_all_objects_alt_camera};
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    // Image - 800x600 as required for audit
    const IMAGE_WIDTH: i32 = 800;
    const IMAGE_HEIGHT: i32 = 600;
    const ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
    const SAMPLES_PER_PIXEL: i32 = 200;
    const MAX_DEPTH: i32 = 100;

    // Get configuration from command-line arguments
    let config = Config::from_args();
    let num_frames = config.num_frames;
    let scene_num = config.scene;

    // Select scene based on CLI argument
    let world = match scene_num {
        0 => random_scene(),
        1 => scene_sphere(),
        2 => scene_plane_cube(),
        3 => scene_all_objects(),
        4 => scene_all_objects_alt_camera(),
        _ => {
            eprintln!("Unknown scene number: {}. Using default scene 0.", scene_num);
            random_scene()
        }
    };

    // Adjust brightness based on scene (scene 2 needs lower brightness)
    let intensity = if scene_num == 2 {
        0.3  // Lower brightness for scene 2 (plane + cube)
    } else {
        0.75 // Normal brightness for other scenes
    };

    let lights = vec![
        PointLight::new(Point3::new(0.0, 5.0, 4.0), Color::new(1.0, 0.85, 0.6), intensity), // overhead lamp
    ];

    // --- Render Loop for Video ---
    for frame in 0..num_frames {
        // --- Calculate Camera Position for this frame ---
        // Different camera positions for different scenes
        let (lookat, lookfrom) = match scene_num {
            1 => {
                // Scene 1: Sphere - camera looking at sphere
                let lookat = Point3::new(0.0, 1.0, 0.0);
                let lookfrom = Point3::new(3.0, 2.0, 3.0);
                (lookat, lookfrom)
            }
            2 => {
                // Scene 2: Plane + Cube - camera looking at cube
                let lookat = Point3::new(0.0, 0.5, 0.0);
                let lookfrom = Point3::new(5.0, 5.0, 5.0);
                (lookat, lookfrom)
            }
            3 => {
                // Scene 3: All objects - camera looking at center, хороший обзор всех объектов
                let lookat = Point3::new(0.0, 0.6, 0.0);  // Смотрим на центр объектов
                let lookfrom = Point3::new(4.0, 2.5, 4.0); // Камера спереди-сбоку, выше
                (lookat, lookfrom)
            }
            4 => {
                // Scene 4: All objects - different camera angle, с другой стороны
                let lookat = Point3::new(0.0, 0.6, 0.0);   // Тот же центр
                let lookfrom = Point3::new(-4.0, 2.5, 4.0); // Камера с другой стороны
                (lookat, lookfrom)
            }
            _ => {
                // Default: original camera setup
                let lookat = Point3::new(0.85, 0.2, 1.35);
                let radius = 6.5;
                let start_angle_rad = 0.46;
                let angle_step = if num_frames > 1 {
                    crate::core::common::degrees_to_radians(360.0) / num_frames as f64
                } else {
                    0.0
                };
                let current_angle = start_angle_rad + frame as f64 * angle_step;
                let lookfrom = Point3::new(
                    lookat.x() + radius * current_angle.cos(),
                    3.5,
                    lookat.z() + radius * current_angle.sin()
                );
                (lookat, lookfrom)
            }
        };

        let vup = Point3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 6.0;
        let aperture = 0.02;

        // Adjust FOV based on scene for better visibility
        let vfov = match scene_num {
            3 | 4 => 30.0,  // Wider FOV for scenes with all objects
            _ => 20.0,      // Default FOV
        };

        let cam = Camera::new(
            lookfrom,
            lookat,
            vup,
            vfov,
            ASPECT_RATIO,
            aperture,
            dist_to_focus,
        );

        // Debug: print image and camera settings for verification
        eprintln!("DEBUG: IMAGE_WIDTH = {}, IMAGE_HEIGHT = {}, lookfrom_y = {}", IMAGE_WIDTH, IMAGE_HEIGHT, lookfrom.y());

        // --- Render a single frame ---
        let filename = format!("output/scene{}_frame_{:03}.ppm", scene_num, frame);
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
