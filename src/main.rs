

mod vec3;
mod ray;
mod camera;
mod material;
mod hittable;
mod sphere;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::hittable::{
    Hittable,
    HittableList,
};

use crate::camera::Camera;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use rand::distributions::Uniform;

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image = (
        400,
        (400.0 / aspect_ratio) as i32
    );
    let samples_per_pixel = 100;
    let max_depth = 50;
    
    // world

    let mut world = HittableList::new();
    world.add(
        Box::new(
            Sphere{
                center: Vec3{ x: 0.0, y: 0.0, z: -1.0},
                radius: 0.5
                material: None,
            }
        )
    );
    world.add(
        Box::new(
            Sphere{
                center: Vec3{ x: 0.0, y: -100.5, z: -1.0 },
                radius: 100.0,
                material: None,
            }
        )
    );
    // camera

    let cam = Camera::new();

    // render
    let mut small_rng = SmallRng::from_entropy();
    let distrib = Uniform::new(0.0, 1.0);
	println!("P3\n{} {}\n255", image.0, image.1);
    for y in (0..image.1).rev() {
        eprintln!("Scanlines remaining: {}", image.1 - y);
        for x in 0..image.0 {
            let mut color = Vec3::zero();
            for _ in 0..samples_per_pixel {
                let u = ((x as f32) + small_rng.sample(distrib)) / ((image.0 - 1) as f32);
                let v = ((y as f32) + small_rng.sample(distrib)) / ((image.1 - 1) as f32);
                let ray = cam.get_ray(u, v);
                color+= ray_color(ray, &world, max_depth, &mut small_rng, distrib);
            }
            println!("{}", color.print_ppm(samples_per_pixel));
        }
    }
    eprintln!("Done!");
}

fn ray_color(r: Ray, world: &HittableList, depth: u32, srng: &mut SmallRng, distrib: Uniform<f32> ) -> Vec3 {
    // recursion depth guard
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f32::INFINITY){
        let target = rec.p + rec.normal + Vec3::rand_unit_vector(srng, distrib);
        return ray_color(
            Ray{
                orig: rec.p,
                dir: target - rec.p,
            },
            world, depth, srng, distrib
        ) * 0.5;
    }
    let unitdir = Vec3::as_unit(&r.dir);
    let t = 0.5 * (unitdir.y + 1.0);
    return Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32{
    let oc = ray.orig - center;
    let a = ray.dir.length_squared();
    let half_b = Vec3::dot(oc, ray.dir);
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

