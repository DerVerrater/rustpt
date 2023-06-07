
use crate::vec3::Vec3;
use crate::ray::Ray;
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

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList{
    hittables: Vec<Box<dyn Hittable>>,
}

impl HittableList{
    pub fn new() -> HittableList {
        HittableList { 
            hittables: Vec::<Box<dyn Hittable>>::new()
        }
    }
    pub fn add(&mut self, hittable: Box<dyn Hittable> ) -> () {
        self.hittables.push(hittable);
    }
//    pub fn clear(&mut self) -> () {
//        self.hittables.clear();
//    }
}

impl Hittable for HittableList{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>{
        let mut might_return = HitRecord {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            material: None,
            t: t_max,
            front_face: false,
        };
        let mut hit_anything = false;

        for item in &self.hittables {
            if let Some(record) = item.hit(r, t_min, might_return.t){
                hit_anything = true;
                might_return = record;
            }
        }
        if hit_anything{
            return Some(might_return);
        } else { return None; }
    }
}


