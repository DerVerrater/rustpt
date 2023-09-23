
mod primitives;
mod renderer;
mod scene;

use crate::primitives::Vec3;
use crate::scene::{
    Camera, 
    Hittable,
    Material,
};
use crate::renderer::RenderCommand;

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use rand::distributions::Uniform;

use std::thread;

fn main() {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let image = (
        1920,
        (1920.0 / aspect_ratio) as i32
    );
    let samples_per_pixel: u32 = 10;
    let max_depth = 50;

    // random generator
    let mut small_rng = SmallRng::seed_from_u64(0);

    // world
    let world = random_scene(&mut small_rng);

    // camera

    let cam = Camera::new(
        Vec3::new(13.0, 2.0, 3.0), // lookfrom
        Vec3::zero(), // lookat
        Vec3::new(0.0, 1.0, 0.0), // vup
        20.0,
        aspect_ratio, 
        0.1, // aperture
        10.0, // dist_to_focus
    );

    // render
    // The render loop should now be a job submission mechanism
    // Iterate lines, submitting them as tasks to the thread.
	println!("P3\n{} {}\n255", image.0, image.1);
    let context = renderer::RenderContext {
        camera: cam,
        image,
        max_depth,
        samples_per_pixel,
        world,
    };

    thread::scope(|s| {
        let (mut dispatcher, scanline_receiver) = renderer::Dispatcher::new(&small_rng, 12);

        s.spawn(move || {
            for y in (0..image.1).rev() {
                eprintln!("Submitting scanline: {}", y);
                let job = RenderCommand::Line { line_num: y, context: context.clone() };
                dispatcher.submit_job(job).unwrap();
            }

            dispatcher.submit_job(RenderCommand::Stop).unwrap();
            // ... also I happen to know there are 4 threads.
        });

        /*
         * Store received results in the segments buffer.
         * Some will land before their previous segments and will need to be held
         * until the next-to-write arrives.
         *
         * Elements are sorted in reverse order so that they can be popped from the
         * Vec quickly.
         *
         * The queue is scanned every single time a new item is received. In the
         * happy path where the received item is next-up, it'll be buffered, checked
         * and then printed. In the case where it isn't, it'll get buffered and
         * stick around for more loops. When the next-to-write finally lands, it
         * means the n+1 element is up, now. If that element is already in the buffer
         * we want to write it out. Hence the loop that scans the whole buffer each
         * receive.
         *
         * TODO: There could be an up-front conditional that checks to see if the
         * received item *is* the next-to-write and skip the buffering step.
         * But I need to make the concept work at all, first.
         */
        let mut raster_segments = Vec::<renderer::RenderResult>::new();
        let mut sl_output_index = image.1-1; // scanlines count down, start at image height.
        while let Ok(scanline) = scanline_receiver.recv() {
            eprintln!("Received scanline: {}", scanline.line_num);

            raster_segments.push(scanline);
            raster_segments.sort_by( |a, b| b.cmp(a) );

            loop {
                if raster_segments.len() == 0 { break; } // can this ever happen? Not while every
                                                         // single element gets pushed to the
                                                         // buffer first. With the happy path
                                                         // short-circuit noted above, it could.

                let last_ind = raster_segments.len() - 1;
               if raster_segments[last_ind].line_num == sl_output_index{
                    let scanline = raster_segments.pop().unwrap();
                    print_scanline(scanline, samples_per_pixel);
                    sl_output_index -= 1;
                } else {
                    break;
                }
            }
        }
        eprintln!("Size of raster_segments at finish: {}", raster_segments.len());
    });
    
    
    // TODO: Dispatcher shutdown mechanism. Right now, we might technically be leaking threads.
    eprintln!("Done!");
}

fn print_scanline(scanline: renderer::RenderResult, samples_per_pixel: u32){
    eprintln!("Printing scanline num: {}", scanline.line_num);
    for color in &scanline.line {
        println!("{}", color.print_ppm(samples_per_pixel));
    }
}

fn random_scene(srng: &mut SmallRng) -> Hittable {
    let mat_ground = Material::Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) };
    let mut world = Hittable::HittableList { hittables : Vec::<Hittable>::new() };
    
    world.push( Hittable::Sphere { center: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: mat_ground });
    
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
                            material: sphere_material,
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
                            material: material,
                        }
                    );
                } else {
                    // glass
                    let material = Material::Dielectric { index_refraction: 1.5 };
                    world.push(
                        Hittable::Sphere{
                            center,
                            radius: 0.2,
                            material: material,
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
        material: material1
    });

    let material2 = Material::Lambertian { albedo: Vec3::new(0.4, 0.2, 0.1) };
    world.push( Hittable::Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2
    });

    let material3 = Material::Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 };
    world.push( Hittable::Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3
    });

    return world;
}


