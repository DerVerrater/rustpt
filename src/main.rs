

mod vec3;
mod ray;
mod camera;
mod material; mod hittable;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::material::Material;

use crate::camera::Camera;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use rand::distributions::Uniform;

use std::thread;
use std::sync::mpsc;

fn main() {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let image = (
        400,
        (400.0 / aspect_ratio) as i32
    );
    let samples_per_pixel: u32 = 10;
    let max_depth = 50;

    // random generator
    let mut small_rng = SmallRng::seed_from_u64(0);

    // world
    let world = random_scene(&mut small_rng);

    // camera

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::zero();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus
    );
    
    // thread messaging channels
    // Render output pipe endpoints
    let (render_tx, render_rx) = mpsc::sync_channel::<(i32, Vec<Vec3>)>(1); // TODO: Figure out good names for the ends of the output pipe
    let (job_tx, job_rx) = mpsc::channel::<RenderCommand>();

    // Threads exist for the whole duration of the (main function) program.
    let thread_handle = thread::spawn(move || {
        let mut srng = small_rng.clone();
        while let Ok(job) = job_rx.recv() {
            match job {
                RenderCommand::Stop => {
                    break;
                }
                RenderCommand::Line { line_num, context } => {
                    let line = render_line(line_num, &mut srng, context);
                    let result = (line_num, line);
                    render_tx.send(result).unwrap();
                }
            }
        }
    });

    // render
    // The render loop should now be a job submission mechanism
    // Iterate lines, submitting them as tasks to the thread.
	println!("P3\n{} {}\n255", image.0, image.1);
    let context = RenderContext {
        camera: cam,
        image,
        max_depth,
        samples_per_pixel,
        world,
    };
    for y in (0..image.1).rev() {
        eprintln!("Submitting scanline: {}", y);
        let job = RenderCommand::Line { line_num: y, context: context.clone() };
        job_tx.send(job).unwrap();
    }
    job_tx.send(RenderCommand::Stop).unwrap();

    while let Ok(line) = render_rx.recv() {
        //TODO: sort results once multiple threads are introduced.
        let (linenum, colors) = line;
        eprintln!("Received scanline: {}", linenum);
        for color in colors {
            println!("{}", color.print_ppm(samples_per_pixel));
        }
    }
    thread_handle.join().unwrap();
    eprintln!("Done!");
}

#[derive (Clone)]
struct RenderContext{
    image: (i32, i32),
    samples_per_pixel: u32,
    max_depth: u32,
    world: Hittable,
    camera: Camera,
}

enum RenderCommand{
    Stop,
    Line { line_num: i32, context: RenderContext },
}

fn render_line(y: i32, small_rng: &mut SmallRng, context: RenderContext  ) -> Vec<Vec3> {
    let distrib_zero_one = Uniform::new(0.0, 1.0);
    let distrib_plusminus_one = Uniform::new(-1.0, 1.0);
    let mut line = Vec::<Vec3>::new();
    for x in 0..context.image.0 {
        let mut color = Vec3::zero();
        for _ in 0..context.samples_per_pixel {
            let u = ((x as f32) + small_rng.sample(distrib_zero_one)) / ((context.image.0 - 1) as f32);
            let v = ((y as f32) + small_rng.sample(distrib_zero_one)) / ((context.image.1 - 1) as f32);
            let ray = context.camera.get_ray(u, v, small_rng);
            color+= ray_color(ray, &context.world, context.max_depth, small_rng, distrib_plusminus_one);
        }
        line.push(color);
    }
    return line;
}

fn ray_color(r: Ray, world: &Hittable, depth: u32, srng: &mut SmallRng, distrib: Uniform<f32> ) -> Vec3 {
    // recursion depth guard
    if depth == 0 {
        return Vec3::zero();
    }

    if let Some(rec) = world.hit(r, 0.001, f32::INFINITY){
        let mut scattered = Ray {
            orig: Vec3::zero(),
            dir: Vec3::zero()
        };
        let mut attenuation = Vec3::zero();
        match rec.material {
            Some(mat) => {
                if mat.scatter(r, rec, &mut attenuation, &mut scattered, srng) {
                    return attenuation * ray_color(scattered, world, depth-1, srng, distrib);
                };
            },
            None => return Vec3::zero(),
        }
    }

    let unitdir = Vec3::as_unit(r.dir);
    let t = 0.5 * (unitdir.y + 1.0);
    return Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn random_scene(srng: &mut SmallRng) -> Hittable {
    let mat_ground = Material::Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) };
    let mut world = Hittable::HittableList { hittables : Vec::<Hittable>::new() };
    
    world.push( Hittable::Sphere { center: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: Some(mat_ground) });
    
    let distrib_zero_one = Uniform::new(0.0, 1.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = srng.sample(distrib_zero_one);
            let center = Vec3 {
                x: a as f32 + 0.9 * srng.sample(distrib_zero_one),
                y: 0.2,
                z: b as f32 + 0.9 * srng.sample(distrib_zero_one),
            };
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::rand(srng, distrib_zero_one) * Vec3::rand(srng, distrib_zero_one);
                    let sphere_material = Material::Lambertian { albedo };
                    world.push(
                        Hittable::Sphere {
                            center,
                            radius: 0.2,
                            material: Some(sphere_material),
                        }
                    );
                } else if choose_mat < 0.95 {
                    // metal
                    let distr_albedo = Uniform::new(0.5, 1.0);
                    let distr_fuzz = Uniform::new(0.0, 0.5);

                    let albedo = Vec3::rand(srng, distr_albedo);
                    let fuzz = srng.sample(distr_fuzz);
                    let material = Material::Metal { albedo, fuzz };
                    world.push(
                        Hittable::Sphere {
                            center,
                            radius: 0.2,
                            material: Some(material),
                        }
                    );
                } else {
                    // glass
                    let material = Material::Dielectric { index_refraction: 1.5 };
                    world.push(
                        Hittable::Sphere{
                            center,
                            radius: 0.2,
                            material: Some(material),
                        }
                    );

                };
            }
        }
    }

    let material1 = Material::Dielectric { index_refraction: 1.5 };
    world.push( Hittable::Sphere{
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Some(material1)
    });

    let material2 = Material::Lambertian { albedo: Vec3::new(0.4, 0.2, 0.1) };
    world.push( Hittable::Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Some(material2)
    });

    let material3 = Material::Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 };
    world.push( Hittable::Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Some(material3)
    });

    return world;
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

