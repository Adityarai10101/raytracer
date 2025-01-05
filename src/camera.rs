use crate::ray3::Ray3;
use crate::object::*;
use crate::vec3::{Point3, Vec3, Color3};
use crate::rand::Rand;

pub fn make_spheres() -> Vec<Box<dyn HittableMaterial>> {
    let mut objects: Vec<Box<dyn HittableMaterial>> = vec![
        // Ground sphere (yellow Lambertian)
        Box::new(LambertianSphere::new(
            Point3::new(0.0, -100.5, -1.0),
            95.0,
            Color3::new(0.8, 0.8, 0.0),
        )),
    ];

    for j in -10..10 as i32 {
        for i in -10..10 as i32 {
            objects.push(
                Box::new(LambertianSphere::new(
                    Point3::new(j as f64, -4.0, i as f64),
                    0.5,
                    Color3::new((j + 10) as f64 / 20.0, (i +10) as f64 / 20.0, 1.0),
                ))
            );
        }
    }
    return objects;
}


pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    focal_length: f64,
    viewport_height: f64,
    viewport_width: f64,
    camera_center: Point3,
    samples_per_pixel: u8,

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
            samples_per_pixel: 2, 
            viewport_u,
            viewport_v,
            pixel_x_delta,
            pixel_y_delta
        }
    }

    pub fn smile(
        &self
    ) -> Vec<Vec<Vec3>> {
        let mut randomizer = Rand::new_with_nanos();
        let image_width: usize = self.image_width() as usize;
        let image_height: usize = self.image_height() as usize;
        let mut image = vec![vec![Vec3::new(0.0, 0.0, 0.0); image_width]; image_height];
        for j in 0..self.image_height as usize {
            println!("{:#?}", j);
            for i in 0..image_width as usize {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0); // this is rgb. can be changed for
                                                                 // tint. lower value = reduce that
                                                                 // specific thing (ie anti-red
                                                                 // tint) higher = more ( ie red or
                                                                 // green tint). all lower =
                                                                 // darker, all higher = lighter
                for sample in 0..self.samples_per_pixel as usize {
                    let offset_i = randomizer.next_with_range(-0.5, 0.5) as f64;
                    let offset_j = randomizer.next_with_range(-0.5, 0.5) as f64;

                    let pixel_center = self.pixel_00_location()
                        + ((i as f64 + offset_i) * self.pixel_x_delta())
                        + ((j as f64 + offset_j) * self.pixel_y_delta());
                        
                    let ray_direction = Ray3::new(
                        self.camera_center(),
                        pixel_center - self.camera_center(),
                    );
                    
                    pixel_color = pixel_color + self.ray_color(&ray_direction, 50, &mut randomizer);
                }
                image[j][i] = pixel_color / (self.samples_per_pixel) as f64
            }
        }
        return image;
    }

    pub fn ray_color(
        &self,
        ray: &Ray3,
        max_depth: i32,
        randomizer: &mut Rand
    ) -> Vec3 {
        if max_depth <= 0 {
            return Color3::new(0.0, 0.0, 0.0);
        }
        // let objects:  = vec![
        //     LambertianSphere::new(Point3::new(0.0, 0.0, -1.0), 0.5),
        //     MetalSphere::new(Point3::new(0.0, -100.5, -1.0), 100.0),
        //     // Sphere::new(Point3::new(-1.0, -1.0, -1.3), 0.6)
        // ];
        let objects = make_spheres();
        let mut t = -1.0;
        let mut hit_i = 0;
        for i in 0..objects.len() {
            let t_temp = objects[i].hit_it(ray);
            if (t == -1.0 && t_temp > 0.000001) || (t_temp < t && t_temp > 0.000001) {
                t = t_temp;
                hit_i = i;
            }
        }
        if t > 0.0 {
            let mut ray_scattered = Ray3::empty_new();
            let attenuation = objects[hit_i].scatter(ray, t, &mut ray_scattered, randomizer);
            return attenuation * self.ray_color(&ray_scattered, max_depth - 1, randomizer);
            // if objects[hit_i].scatter(
            // let mut N = (ray.at(t) - objects[hit_i].center).unit_vector();
            // for diffusion coloring (image 7 on the guide)
            // if N.dot(&ray.direction()) > 0.0 { // so if they are in the same direction
            //     N = Vec3::new(0.0, 0.0, 0.0) - N; // make em be in different directions
            // }
            // let direction_vector = N + Vec3::random_unit_vector(randomizer);
            // let direction_vector = Vec3::random_on_hemisphere(&N, randomizer);
            // return 0.5 * self.ray_color(&Ray3::new(ray.at(t), direction_vector), max_depth - 1, randomizer);
            // for normal coloring
            // if N.dot(&ray.direction()) > 0.0 {
            //     N = Vec3::new(0.0, 0.0, 0.0) - N;
            // }
            // return 0.5 * Vec3::new(N.x()+1.0, N.y()+1.0, N.z()+1.0);
        }

        let direction = ray.direction();
        let a = 0.5 * (direction.y() + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
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
