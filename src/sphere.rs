
use crate::vec3::Vec3;
use crate::hittable::{
    Hittable,
    HitRecord,
};
use crate::material::Material;
use crate::ray::Ray;

pub struct Sphere{
    pub center: Vec3,
    pub radius: f32,
    pub material: Option<Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>{
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;
        
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        
        // nearest root that lies within tolerance
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let mut record = HitRecord{
            p: r.at(root),
            normal: (r.at(root) - self.center) / self.radius,
            material: self.material,
            t: root,
            front_face: false,
        };
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(r, outward_normal);
        Some(record)
    }
}

