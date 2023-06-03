
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::Vec3;

pub struct Material;

pub Scatter {
    fn scatter(
        ray_in: Ray, rec: HitRecord, attenuation: Vec3, scattered: Ray
    ) -> bool;
}

