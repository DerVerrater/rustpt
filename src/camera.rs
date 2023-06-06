
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

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let vp_height = 2.0 * h;
        let vp_width = aspect_ratio * vp_height;

        let w = Vec3::as_unit(lookfrom - lookat);
        let u = Vec3::as_unit(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let orig = lookfrom;
        let horiz = u * vp_width;
        let verti = v * vp_height;
        let lower_left_corner = orig - horiz / 2.0 - verti / 2.0 - w;

        Camera{
            origin: orig,
            lower_left_corner,
            horizontal: horiz,
            vertical: verti,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let dir = self.lower_left_corner
                + self.horizontal * s
                + self.vertical * t 
                - self.origin;
        Ray{
            orig: self.origin,
            dir,
        }
    }
}
