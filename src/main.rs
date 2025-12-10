use ray_trace::*;
use ray_trace::scenes::{random_scene, scene_sphere, scene_plane_cube, scene_all_objects, scene_all_objects_alt_camera, scene_custom_with_params};

fn main() {
    // Get configuration from command-line arguments
    let config = Config::from_args();
    
    // Select scene based on CLI argument
    let mut scene = match config.scene {
        0 => random_scene(),
        1 => scene_sphere(),
        2 => scene_plane_cube(),
        3 => scene_all_objects(),
        4 => scene_all_objects_alt_camera(),
        5 => scene_custom_with_params(&config.spheres, config.camera_pos, config.camera_lookat),
        _ => {
            eprintln!("Unknown scene number: {}. Using default scene 0.", config.scene);
            random_scene()
        }
    };

    // Apply CLI render settings if provided
    if config.width.is_some() || config.height.is_some() || config.samples.is_some() || config.depth.is_some() {
        let width = config.width.unwrap_or(scene.render_settings().width);
        let height = config.height.unwrap_or(scene.render_settings().height);
        let samples = config.samples.unwrap_or(scene.render_settings().samples_per_pixel);
        let depth = config.depth.unwrap_or(scene.render_settings().max_depth);
        
        scene.set_render_settings_params(width, height, samples, depth);
    }

    // Apply background color from CLI
    let bg_color = Color::new(config.bg_color.0, config.bg_color.1, config.bg_color.2);
    scene.set_background(bg_color);

    // --- Render Loop for Video ---
    // for frame in 0..config.num_frames {
    //     // Handle camera animation for scene 0 (random_scene) with multiple frames
    //     if config.scene == 0 && config.num_frames > 1 {
    //         let lookat = Point3::new(0.85, 0.2, 1.35);
    //         let radius = 6.5;
    //         let start_angle_rad = 0.46;
    //         let angle_step = crate::core::common::degrees_to_radians(360.0) / config.num_frames as f64;
    //         let current_angle = start_angle_rad + frame as f64 * angle_step;
    //         let lookfrom = Point3::new(
    //             lookat.x() + radius * current_angle.cos(),
    //             3.5,
    //             lookat.z() + radius * current_angle.sin()
    //         );
            
    //         scene.set_camera(
    //             lookfrom,
    //             lookat,
    //             Vec3::new(0.0, 1.0, 0.0),
    //             20.0,
    //             0.02,
    //             6.0,
    //         );
    //     }

        // --- Render a single frame ---
        let filename = config.output.unwrap_or("output.ppm".to_string());

        // Render using Scene's built-in method
        if let Err(e) = scene.render_to_file(&filename) {
            eprintln!("Error rendering scene: {}", e);
            std::process::exit(1);
        }
    //}

    eprint!("\nDone.\n");
}
