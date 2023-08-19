
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3;
use crate::vec3::Vec3;

use rand::Rng;
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
    ) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_dir = rec.normal + Vec3::rand_unit_vector(srng);
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
                    dir: reflected + Vec3::rand_in_unit_sphere(srng) * *fuzz,
                };
                *attenuation = *albedo;
                return Vec3::dot(scattered.dir, rec.normal) > 0.0;
            },
            Material::Dielectric { index_refraction } => {
                *attenuation = Vec3::ones();
                let refraction_ratio = if rec.front_face { 1.0 / index_refraction } else { *index_refraction };
                
                let unit_direction = Vec3::as_unit(ray_in.dir);
                let cos_theta = vec3::min(Vec3::dot(-unit_direction, rec.normal), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let distrib_zero_one = Uniform::new(0.0, 1.0);
                let direction = if cannot_refract || Material::reflectance(cos_theta, refraction_ratio) > srng.sample(distrib_zero_one) {
                    Vec3::reflect(unit_direction, rec.normal)
                } else {
                    Vec3::refract(unit_direction, rec.normal, refraction_ratio)
                };
                *scattered = Ray {
                    orig: rec.p,
                    dir: direction 
                };
                return true;
            },
        }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}
