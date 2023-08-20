
use crate::primitives::{Vec3, Ray};
use crate::material::Material;

pub struct HitRecord{
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord{
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) -> (){
        self.front_face = Vec3::dot(r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

#[derive (Clone)]
pub enum Hittable {
    Sphere { center: Vec3, radius: f32, material: Option<Material> },
    HittableList { hittables: Vec<Hittable> }
}

impl Hittable {
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Hittable::HittableList { hittables } => {
                let mut might_return = HitRecord {
                    p: Vec3::zero(),
                    normal: Vec3::zero(),
                    material: None,
                    t: t_max,
                    front_face: false,
                };
                let mut hit_anything = false;

                for item in hittables {
                    if let Some(record) = item.hit(r, t_min, might_return.t){
                        hit_anything = true;
                        might_return = record;
                    }
                }
                if hit_anything{
                    return Some(might_return);
                } else { return None; }
            }

            Hittable::Sphere { center, radius, material } => {
                let oc = r.orig - *center;
                let a = r.dir.length_squared();
                let half_b = Vec3::dot(oc, r.dir);
                let c = oc.length_squared() - radius * radius;
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
                    normal: (r.at(root) - *center) / *radius,
                    material: *material,
                    t: root,
                    front_face: false,
                };
                let outward_normal = (record.p - *center) / *radius;
                record.set_face_normal(r, outward_normal);
                Some(record)
            }
        }
    }
    pub fn push(&mut self, item: Hittable) {
        if let Hittable::HittableList { hittables } = self {
            hittables.push(item);
        }
    }
}

