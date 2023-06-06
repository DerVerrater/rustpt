
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3;
use crate::vec3::Vec3;

use rand::rngs::SmallRng;
use rand::distributions::Uniform;



#[derive(Copy, Clone, Debug)]
pub enum Material{
    Lambertian { albedo: Vec3 },
    Metal { albedo:Vec3, fuzz: f32 },
    Dielectric { index_refraction: f32 },
}

impl Material {
    pub fn scatter(
        &self,
        ray_in: Ray,
        rec: HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
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
            Material::Metal { albedo, fuzz } => {
                let reflected = Vec3::reflect(
                    Vec3::as_unit(ray_in.dir),
                    rec.normal
                );
                *scattered = Ray{
                    orig: rec.p,
                    dir: reflected + Vec3::rand_in_unit_sphere(srng, distrib) * *fuzz,
                };
                *attenuation = *albedo;
                return Vec3::dot(scattered.dir, rec.normal) > 0.0;
            },
            Material::Dielectric { index_refraction } => {
                *attenuation = Vec3::ones();
                let refraction_ratio = if rec.front_face { 1.0 / index_refraction } else { *index_refraction };
                
                let unit_direction = Vec3::as_unit(ray_in.dir);
                let refracted = Vec3::refract(unit_direction, rec.normal, refraction_ratio);

                *scattered = Ray {
                    orig: rec.p,
                    dir: refracted
                };
                return true;
            },
        }
    }
}
