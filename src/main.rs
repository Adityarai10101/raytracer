use raytracer::{
    camera::Camera,
    ray3::Ray3,
    utils,
    vec3::{Point3, Vec3}
};
use std::io;

fn main() -> io::Result<()> {
    let view_camera = Camera::new(16.0 / 9.0, 400, 1.0, 2.0, Point3::new(0.0, 0.0, 0.0));

    let image_width: usize = view_camera.image_width() as usize;
    let image_height: usize = view_camera.image_height() as usize;
    let mut image = vec![vec![Vec3::new(0.0, 0.0, 0.0); image_width]; image_height];
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = view_camera.pixel_00_location()
                + (i as f64 * view_camera.pixel_x_delta())
                + (j as f64 * view_camera.pixel_y_delta());
            let ray_direction = Ray3::new(
                view_camera.camera_center(),
                pixel_center - view_camera.camera_center(),
            );
            image[j][i] = view_camera.ray_color(&ray_direction);
        }
    }

    utils::write_to_ppm("output.ppm", &image)?;
    Ok(())
}
