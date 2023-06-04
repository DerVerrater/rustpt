
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::Vec3;

pub trait Material {
    fn scatter(
        &self, ray_in: Ray, rec: HitRecord, attenuation: Vec3, scattered: Ray
    ) -> bool;
}

