use raytracer::{
    camera::Camera,
    ray3::Ray3,
    utils,
    vec3::{Point3, Vec3},
    rand::Rand
};
use std::io;

fn main() -> io::Result<()> {
    let view_camera = Camera::new(16.0 / 9.0, 400, 0.5, 5.0, Point3::new(0.0, 0.0, 0.0));
    let image = view_camera.smile();

    utils::write_to_ppm("output.ppm", &image)?;

    Ok(())
}
