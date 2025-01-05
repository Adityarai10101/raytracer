use crate::vec3::*;
use crate::ray3::Ray3;
use crate::rand::Rand;

pub trait HittableMaterial {
    fn hit_it(&self, ray: &Ray3) -> f64;
    fn scatter(&self, ray_in: &Ray3, intersection_point_t: f64, return_ray: &mut Ray3, randomizer: &mut Rand) -> Vec3;
}

pub struct LambertianSphere {
    pub center: Point3,
    pub radius: f64,
    pub albedo: Color3// this is a color
}

impl LambertianSphere {
    pub fn new(center: Point3, radius: f64, albedo: Color3) -> Self {
        Self { center, radius, albedo }
    }
}

impl HittableMaterial for LambertianSphere {
    fn hit_it(&self, ray: &Ray3) -> f64 {
        let center_diff = self.center - ray.origin();
        let a = ray.direction().dot(&ray.direction());
        let b = -2.0 * ray.direction().dot(&center_diff);
        let c = center_diff.dot(&center_diff) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return -1.0;
        }
        return ( -b - discriminant.sqrt() ) / 2.0 * a;
    }
    fn scatter(&self, ray_in: &Ray3, intersection_point_t: f64, return_ray: &mut Ray3, randomizer: &mut Rand) -> Vec3 {
        // returns the albedo color shit
        let mut N = (ray_in.at(intersection_point_t) - self.center).unit_vector();
        if N.dot(&ray_in.direction()) > 0.0 {
            N = Vec3::new(0.0, 0.0, 0.0) - N;
        }
        let mut scatter_direction_vector = N + Vec3::random_unit_vector(randomizer);
        if scatter_direction_vector.near_zero() {
            scatter_direction_vector = N;
        }
        *return_ray = Ray3::new(ray_in.at(intersection_point_t), scatter_direction_vector);
        return self.albedo;
    }
}

pub struct MetalSphere {
    pub center: Point3,
    pub radius: f64,
    pub albedo: Color3, // this is a color
    pub fuzz: f64, // how much to fuzz the reflections
}

impl MetalSphere {
    pub fn new(center: Point3, radius: f64, albedo: Color3, fuzz: f64) -> Self {
        Self { center, radius, albedo, fuzz }
    }
}

impl HittableMaterial for MetalSphere {
    fn hit_it(&self, ray: &Ray3) -> f64 {
        let center_diff = self.center - ray.origin();
        let a = ray.direction().dot(&ray.direction());
        let b = -2.0 * ray.direction().dot(&center_diff);
        let c = center_diff.dot(&center_diff) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return -1.0;
        }
        return ( -b - discriminant.sqrt() ) / 2.0 * a;
    }
    fn scatter(&self, ray_in: &Ray3, intersection_point_t: f64, return_ray: &mut Ray3, randomizer: &mut Rand) -> Vec3 {
        // returns the albedo color shit
        let mut N = (ray_in.at(intersection_point_t) - self.center).unit_vector();
        if N.dot(&ray_in.direction()) > 0.0 {
            N = Vec3::new(0.0, 0.0, 0.0) - N;
        }
        let reflected = reflect(&ray_in.direction(), &N).unit_vector() + (self.fuzz * Vec3::random_unit_vector(randomizer));
        
        *return_ray = Ray3::new(ray_in.at(intersection_point_t), reflected);
        return self.albedo;
    }
}


pub struct GlassSphere {
    pub center: Point3,
    pub radius: f64,
    pub albedo: Color3,
    pub refraction_index: f64,
}

impl GlassSphere {
    pub fn new(center: Point3, radius: f64, albedo: Color3, refraction_index: f64) -> Self {
        Self { center, radius, albedo, refraction_index }
    }
    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl HittableMaterial for GlassSphere {
    fn hit_it(&self, ray: &Ray3) -> f64 {
        let center_diff = self.center - ray.origin();
        let a = ray.direction().dot(&ray.direction());
        let b = -2.0 * ray.direction().dot(&center_diff);
        let c = center_diff.dot(&center_diff) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return -1.0;
        }
        return ( -b - discriminant.sqrt() ) / 2.0 * a;
    }
    fn scatter(&self, ray_in: &Ray3, intersection_point_t: f64, return_ray: &mut Ray3, randomizer: &mut Rand) -> Vec3 {
        // returns the albedo color shit
        let attenuation = Color3::new(1.0, 1.0, 1.0);
        let mut ri = 1.0 / self.refraction_index;

        let mut N = (ray_in.at(intersection_point_t) - self.center).unit_vector();
        if N.dot(&ray_in.direction()) > 0.0 {
            N = Vec3::new(0.0, 0.0, 0.0) - N;
            ri = self.refraction_index;
        }

        let unit_direction = ray_in.direction(); // should already be a unit vec
        let cos_theta = f64::min((-unit_direction).dot(&N), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if (cannot_refract || self.reflectance(cos_theta, ri) > randomizer.next() as f64){
            reflect(&unit_direction, &N)
        } else {
            refract(&unit_direction, &N, ri)
        };
        *return_ray = Ray3::new(ray_in.at(intersection_point_t), direction);
        return self.albedo;
    }
}
