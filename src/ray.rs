
use crate::vec3::Vec3;
use crate::hittable::{
    Hittable,
    HitRecord,
};

#[derive(Copy)]
#[derive(Clone)]
pub struct Ray{
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray{
    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + self.dir*t
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn check_lerp(){
        let ray = Ray{
            orig: Vec3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 1.0, 0.0)
        };
        let half = ray.at(0.5);
        assert_eq!(
            half,
            Vec3::new(0.5, 0.5, 0.0)
        );
    }
}
