use crate::vec3::Point3;
use crate::ray3::Ray3;

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray3) -> f64 {
    let center_diff = center - ray.origin();
    let a = ray.direction().dot(&ray.direction());
    let b = -2.0 * ray.direction().dot(&center_diff);
    let c = center_diff.dot(&center_diff) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    return ( -b - discriminant.sqrt() ) / 2.0 * a;
}
