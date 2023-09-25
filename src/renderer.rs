
use crate::primitives::{
    Vec2i,
    Vec2f,
    Vec3,
    Ray,
    Rect,
};
use crate::scene::{
    Hittable,
    Scene,
};

use rand::rngs::SmallRng;

use itertools::{self, Itertools};

const SKY_COLOR: Vec3 = Vec3 { x: 0.5, y: 0.7, z: 1.0};

pub struct RenderProperties {
    pub samples: u32, // samples are averaged results over a pixel
    pub bounces: u32, // bounces are how far the ray will travel (in hits not total distance)
}

fn to_uv(coord: Vec2i, img_size: Vec2i) -> Vec2f {
    let u = (coord.x as f32) / ((img_size.x - 1) as f32);
    let v = (coord.y as f32) / ((img_size.y - 1) as f32);
    Vec2f::new(u, v)
}

fn ray_color(
    r: Ray, surface: &Hittable, depth: u32,
    rng: &mut SmallRng,
) -> Vec3 {
    // recursion guard
    if depth == 0 {
        return Vec3::zero();
    }
    
    // cast a ray, interrogate hit record
    if let Some(record) = surface.hit(r, 0.001, f32::INFINITY){
        let mut scattered = Ray {
            orig: Vec3::zero(),
            dir: Vec3::zero(),
        };
        let mut attenuation = Vec3::zero();
        if record.material.scatter(
            r,
            &record,
            &mut attenuation,
            &mut scattered,
            rng
        ) {
            return attenuation * ray_color(
                scattered, surface, depth-1, rng
            );
        }
    } // TODO: explicit else block
    // Rust gets angry about the inner if{} block because it evaluates to ()
    // when the else path is taken. This is a problem for a function
    // that returns Vec3 and not ().

    { // when nothing is struck, return sky color
        let unitdir = Vec3::as_unit(r.dir);
        let t = 0.5 * (unitdir.y + 1.0);
        return Vec3::ones() * (1.0 - t) + SKY_COLOR * t
    }
}

fn sample_pixel(
    coord: Vec2i, // location in image/screen space
    scene: &Scene,  // scene we're drawing
    render_props: &RenderProperties,
    img_size: Vec2i,
    // Supplied by the execution environment (the thread)
    rng: &mut SmallRng,
) -> Vec3{
    (0..render_props.samples)
    .fold(
        Vec3::zero(),
        |color, _sample| -> Vec3 {
            let uv = to_uv(coord, img_size);
            let ray = scene.camera.get_ray(uv.x, uv.y, rng);
            if ray.dir.x.is_nan() {
                panic!("Ray dir.x is NAN");
            }
            color + ray_color(ray, &scene.world, render_props.bounces, rng)
        }
    )
}

pub struct Tile {
    _bounds: Rect,
    pub pixels: Vec<Vec3>,
}

impl Tile {
    pub fn render_tile(
        bounds: Rect,       // bounds of the region to render
        img_size: Vec2i,    // final image resolution (needed for proper UV mapping)
        scene: &Scene,
        properties: &RenderProperties, // TODO: Place image size in render properties?
        rng: &mut SmallRng,
    ) -> Self {
        let pixel_iter = (bounds.y..(bounds.y + bounds.h))
            .cartesian_product( bounds.x..(bounds.x + bounds.w));
        let pixels = pixel_iter.map(
            |coord| -> Vec3 {
                sample_pixel(
                    Vec2i{x: coord.1, y: coord.0},
                    scene,
                    properties,
                    img_size,
                    rng,
                )
            }
        ).collect();
        Self {
            _bounds: bounds,
            pixels
        }
    }
    pub fn render_line(
        bounds: Rect, y: i32, // bounding rect and line
        img_size: Vec2i,
        scene: &Scene,
        properties: &RenderProperties,
        rng: &mut SmallRng, // rng utils
    ) -> Self {
        let pixels = (0..bounds.w)
        .map ( |x| -> Vec3{
            sample_pixel(
                Vec2i{x, y},
                scene,
                properties,
                img_size,
                rng,
            )
        })
        .collect();
        Self { _bounds: bounds, pixels }
    }
}
