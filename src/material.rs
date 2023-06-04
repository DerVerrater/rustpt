
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::Vec3;

use rand::Rng;
use rand::rngs::SmallRng;
use rand::distributions::Uniform;



#[derive(Copy, Clone, Debug)]
pub enum Material{
    Lambertian{ albedo: Vec3 },
    Metal{ albedo:Vec3 },
}

impl Material {
    pub fn scatter(
        &self,
        ray_in: Ray,
        rec: HitRecord,
        attenuation: &mut Vec3,
        scattered:&mut Ray,
        srng: &mut SmallRng,
        distrib: Uniform<f32>,
    ) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_dir = rec.normal + Vec3::rand_unit_vector(srng, distrib);
                // The compiler might be smart enough to compute this ^^^ just once. In which case,
                // I don't need to do this weird dance. Oh well. It'll work.
                let scatter_dir = if scatter_dir.near_zero() {  // if near zero,
                    rec.normal                                  // replace with normal
                } else {
                    scatter_dir                                 // else preserve current
                };

                //TODO: Revisit this out-parameter pattern
                // It's a side effect of C++'s obtuse move semantics (and the RTIOW author not
                // using them at all)
                *scattered = Ray{
                    orig: rec.p,
                    dir: scatter_dir
                };
                *attenuation = *albedo; // deref on both sides? Wacky
                return true;
            },
            Material::Metal { albedo } => {
                let reflected = Vec3::reflect(
                    Vec3::as_unit(&ray_in.dir),
                    rec.normal
                );
                *scattered = Ray{
                    orig: rec.p,
                    dir: reflected,
                };
                *attenuation = *albedo;
                return Vec3::dot(scattered.dir, rec.normal) > 0.0;
            },
            _ => return false,
        }
    }
}
