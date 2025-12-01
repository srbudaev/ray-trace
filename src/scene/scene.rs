use crate::core::{Camera, HittableList, PointLight};
use crate::math::vec3::Point3;
use crate::math::vec3::Vec3;
use crate::scene::render::ray_color;
use crate::ppm::write_color;
use crate::core::Color;
use std::fs::File;
use std::io::{BufWriter, Write};

/// Render settings for controlling image output quality and parameters
#[derive(Clone, Debug)]
pub struct RenderSettings {
    pub width: i32,
    pub height: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
}

impl RenderSettings {
    /// Create new render settings with default values
    pub fn new(width: i32, height: i32, samples_per_pixel: i32, max_depth: i32) -> Self {
        Self {
            width,
            height,
            samples_per_pixel,
            max_depth,
        }
    }

    /// Create render settings with default values (800x600, 200 samples, depth 100)
    pub fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            samples_per_pixel: 32,
            max_depth: 10,
        }
    }

    /// Get aspect ratio
    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
}

/// Camera parameters stored separately to allow automatic updates when render settings change
#[derive(Clone, Debug)]
struct CameraParams {
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    vfov: f64,
    aperture: f64,
    focus_dist: f64,
}

/// Scene struct that encapsulates all scene components
pub struct Scene {
    camera: Camera,
    camera_params: CameraParams,
    pub render_settings: RenderSettings,
    pub lights: Vec<PointLight>,
    pub objects: HittableList,
}

impl Scene {
    /// Create a new empty scene with default settings
    pub fn new() -> Self {
        let camera_params = CameraParams {
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            vfov: 20.0,
            aperture: 0.02,
            focus_dist: 6.0,
        };
        let render_settings = RenderSettings::default();
        let aspect_ratio = render_settings.aspect_ratio();
        
        Self {
            camera: Camera::new(
                camera_params.lookfrom,
                camera_params.lookat,
                camera_params.vup,
                camera_params.vfov,
                aspect_ratio,
                camera_params.aperture,
                camera_params.focus_dist,
            ),
            camera_params,
            render_settings,
            lights: Vec::new(),
            objects: HittableList::new(),
        }
    }

    /// Add an object to the scene
    pub fn add_object(&mut self, object: Box<dyn crate::core::Hittable>) {
        self.objects.add(object);
    }

    /// Add a light source to the scene
    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    /// Set the camera for the scene
    pub fn set_camera(
        &mut self,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aperture: f64,
        focus_dist: f64,
    ) {
        self.camera_params = CameraParams {
            lookfrom,
            lookat,
            vup,
            vfov,
            aperture,
            focus_dist,
        };
        let aspect_ratio = self.render_settings.aspect_ratio();
        self.camera = Camera::new(
            self.camera_params.lookfrom,
            self.camera_params.lookat,
            self.camera_params.vup,
            self.camera_params.vfov,
            aspect_ratio,
            self.camera_params.aperture,
            self.camera_params.focus_dist,
        );
    }

    /// Set render settings for the scene
    /// Automatically updates camera aspect ratio if dimensions changed
    pub fn set_render_settings(&mut self, settings: RenderSettings) {
        self.render_settings = settings;
        // Automatically update camera with new aspect ratio
        let aspect_ratio = self.render_settings.aspect_ratio();
        self.camera = Camera::new(
            self.camera_params.lookfrom,
            self.camera_params.lookat,
            self.camera_params.vup,
            self.camera_params.vfov,
            aspect_ratio,
            self.camera_params.aperture,
            self.camera_params.focus_dist,
        );
    }

    /// Set render settings with individual parameters
    pub fn set_render_settings_params(
        &mut self,
        width: i32,
        height: i32,
        samples_per_pixel: i32,
        max_depth: i32,
    ) {
        self.set_render_settings(RenderSettings::new(width, height, samples_per_pixel, max_depth));
    }

    /// Get reference to objects (for rendering)
    pub fn objects(&self) -> &HittableList {
        &self.objects
    }

    /// Get reference to lights
    pub fn lights(&self) -> &[PointLight] {
        &self.lights
    }

    /// Get reference to camera
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Get reference to render settings
    pub fn render_settings(&self) -> &RenderSettings {
        &self.render_settings
    }

    /// Render the scene to a PPM file
    pub fn render_to_file(&self, filename: &str) -> std::io::Result<()> {
        let settings = self.render_settings();
        let width = settings.width;
        let height = settings.height;
        let samples_per_pixel = settings.samples_per_pixel;
        let max_depth = settings.max_depth;

        eprintln!("Rendering scene to {}...", filename);
        eprintln!("  Resolution: {}x{}", width, height);
        eprintln!("  Samples per pixel: {}", samples_per_pixel);
        eprintln!("  Max depth: {}", max_depth);

        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        // Write PPM header
        writeln!(&mut writer, "P3\n{} {}\n255", width, height)?;

        // Render each pixel
        for j in (0..height).rev() {
            eprint!("\rScanlines remaining: {} ", j);
            for i in 0..width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + crate::core::common::random_double()) / (width - 1) as f64;
                    let v = (j as f64 + crate::core::common::random_double()) / (height - 1) as f64;
                    let r = self.camera.get_ray(u, v);
                    pixel_color += ray_color(&r, self.objects(), self.lights(), max_depth);
                }
                write_color(&mut writer, pixel_color, samples_per_pixel);
            }
        }

        eprint!("\nDone rendering to {}.\n", filename);
        Ok(())
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

