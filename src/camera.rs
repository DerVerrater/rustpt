
/*
 *  let viewport = (aspect_ratio * 2.0, 2.0);
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport.1, 0.0);

    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);
 */

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::degrees_to_radians;

use rand::rngs::SmallRng;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3, v: Vec3, /*w: Vec3,*/
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let vp_height = 2.0 * h;
        let vp_width = aspect_ratio * vp_height;

        let w = Vec3::as_unit(lookfrom - lookat);
        let u = Vec3::as_unit(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let orig = lookfrom;
        let horiz = u * vp_width * focus_dist;
        let verti = v * vp_height * focus_dist;
        let lower_left_corner = orig - horiz / 2.0 - verti / 2.0 - w * focus_dist;

        Camera{
            origin: orig,
            lower_left_corner,
            horizontal: horiz,
            vertical: verti,
            u, v, /* w,*/
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, srng: &mut SmallRng) -> Ray {
        let rd = Vec3::rand_in_unit_disk(srng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        let dir = self.lower_left_corner
                + self.horizontal * s
                + self.vertical * t 
                - self.origin - offset;
        Ray{
            orig: self.origin + offset,
            dir,
        }
    }
}
