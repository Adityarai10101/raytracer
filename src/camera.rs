use crate::ray3::Ray3;
use crate::shape::hit_sphere;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    focal_length: f64,
    viewport_height: f64,
    viewport_width: f64,
    camera_center: Point3,

    // vector across the horizontal of the viewport
    viewport_u: Vec3,
    // vector down the verticle of the viewport (y axis in image frame)
    viewport_v: Vec3,

    // pixel to pixel delta vectors (based on image dimensions and viewport vectors)
    pixel_x_delta: Vec3,
    pixel_y_delta: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        focal_length: f64,
        viewport_height: f64,
        camera_center: Point3,
    ) -> Self {
        let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }

        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        // because in our camera frame, the x is right and the y is up and z is in the viewing
        // direction
        // but in our image frame, x is right y is down
        // so need to inverse the y
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_x_delta = viewport_u / image_width as f64;
        let pixel_y_delta = viewport_v / image_height as f64;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            focal_length,
            viewport_height,
            viewport_width,
            camera_center,
            viewport_u,
            viewport_v,
            pixel_x_delta,
            pixel_y_delta
        }
    }

    pub fn ray_color(
        &self,
        ray: &Ray3
    ) -> Vec3 {
        let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.6, ray);
        if t > 0.0 {
            let N = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
            return 0.5 * Vec3::new(N.x()+0.5, N.y()+0.5, N.z()+1.0);
        }
        let direction = ray.direction();
        let a = 0.5 * (direction.y() + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.1, 0.2, 0.3)
    }

    pub fn viewport_u_l(&self) -> Vec3 {
        self.camera_center - Vec3::new(0.0, 0.0, self.focal_length) - self.viewport_u/2.0 - self.viewport_v/2.0
    }

    pub fn pixel_00_location(&self) -> Point3 {
        self.viewport_u_l() + 0.5 * (self.pixel_x_delta + self.pixel_y_delta)
    }

    pub fn camera_center(&self) -> Vec3 {
        self.camera_center
    }
     
    pub fn image_height(&self) -> u32 {
        self.image_height
    }

    pub fn image_width(&self) -> u32 {
        self.image_width
    }

    pub fn pixel_x_delta(&self) -> Vec3 {
        self.pixel_x_delta
    }
    
    pub fn pixel_y_delta(&self) -> Vec3 {
        self.pixel_y_delta
    }
}
