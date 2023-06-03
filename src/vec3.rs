
use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Neg,
};
use std::fmt;
use std::fmt::Display;

use rand::Rng;
use rand::rngs::SmallRng;
use rand::distributions::Uniform;

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Debug)]
pub struct Vec3{
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vec3{
	pub fn new(x: f32, y: f32, z: f32) -> Vec3{
		Vec3{x, y, z}
	}

    pub fn zero() -> Vec3{
        Vec3{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn ones() -> Vec3{
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0
        }
    }

    pub fn rand(srng: &mut SmallRng, distrib: Uniform<f32>) -> Vec3 {
        Vec3{
            x: srng.sample(distrib),
            y: srng.sample(distrib),
            z: srng.sample(distrib),
        }
    }

    pub fn rand_in_unit_sphere(srng: &mut SmallRng, distrib: Uniform<f32>) -> Vec3 {
        loop {
            let p = Vec3::rand(srng, distrib);
            if p.length_squared() >= 1.0 { continue; }
            else { return p; }
        }
    }

    pub fn rand_unit_vector(srng: &mut SmallRng, distrib: Uniform<f32>) -> Vec3 {
        return Vec3::as_unit(&Vec3::rand_in_unit_sphere(srng, distrib));
    }

	pub fn length(&self) -> f32 {
		self.length_squared().sqrt()
	}

	pub fn length_squared(&self) -> f32 {
		(self.x*self.x) + (self.y*self.y) + (self.z*self.z)
	}
    
    // roughly equivalent to the `void write_color(...)` in the book
    pub fn print_ppm(&self, samples_per_pixel: u32) -> String {

        let scale = 1.0 / samples_per_pixel as f32;
        
        // now with gamma correction
        let r = (self.x * scale).sqrt();
        let g = (self.y * scale).sqrt();
        let b = (self.y * scale).sqrt();
        
        let ir = (Vec3::clamp(r, 0.0, 0.999) * 256.0) as i32;
        let ig = (Vec3::clamp(g, 0.0, 0.999) * 256.0) as i32;
        let ib = (Vec3::clamp(b, 0.0, 0.999) * 256.0) as i32;
        format!("{} {} {}", ir, ig, ib)
    }

    pub fn dot(left: Vec3, right: Vec3) -> f32{
        left.x * right.x +
        left.y * right.y +
        left.z * right.z
    }
    
    pub fn cross(u: Vec3, v: Vec3) -> Vec3{
        Vec3{
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x
        }
    }
    
    pub fn as_unit(v: &Vec3) -> Vec3 {
        let len = v.length();
        *v / len
    }

    fn clamp(input: f32, min: f32, max: f32) -> f32 {
        if input < min {
            return min;
        } else if input > max {
            return max;
        } else {
            return input;
        }
    }
}
impl Add for Vec3 {
	type Output = Vec3;
	fn add(self, other: Vec3) -> Vec3 {
		Vec3{
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl AddAssign for Vec3 {
	fn add_assign(&mut self, other: Vec3){
		*self = Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z
		};
	}
}

impl Sub for Vec3 {
	type Output = Vec3;
	fn sub(self, other: Vec3) -> Vec3 {
		Vec3 {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl SubAssign for Vec3 {
	fn sub_assign(&mut self, other: Vec3){
		*self = Self {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z
		};
	}
}

impl Mul<Vec3> for Vec3 {
	type Output = Vec3;
	fn mul(self, other: Vec3) -> Vec3 {
		Vec3 {
			x: self.x * other.x,
			y: self.y * other.y,
			z: self.z * other.z,
		}
	}
}

impl Mul<f32> for Vec3{
	type Output = Vec3;
	fn mul(self, other: f32) -> Vec3 {
		Vec3 {
			x: self.x * other,
			y: self.y * other,
			z: self.z * other,
		}
	}
	
}

impl MulAssign<Vec3> for Vec3 {
	fn mul_assign(&mut self, other: Vec3){
		*self = Self {
			x: self.x * other.x,
			y: self.y * other.y,
			z: self.z * other.z
		};
	}
}

impl MulAssign<f32> for Vec3{
     fn mul_assign(&mut self, other: f32){
		*self = Self {
			x: self.x * other,
			y: self.y * other,
			z: self.z * other
		};
	}   
}

impl Div<Vec3> for Vec3 {
	type Output = Vec3;
	fn div(self, other: Vec3) -> Vec3 {
		Vec3 {
			x: self.x / other.x,
			y: self.y / other.y,
			z: self.z / other.z,
		}
	}
}

impl Div<f32> for Vec3 {
	type Output = Vec3;
	fn div(self, other: f32) -> Vec3 {
		Vec3 {
			x: self.x / other,
			y: self.y / other,
			z: self.z / other,
		}
	}
}

impl DivAssign<Vec3> for Vec3 {
	fn div_assign(&mut self, other: Vec3){
		*self = Self {
			x: self.x / other.x,
			y: self.y / other.y,
			z: self.z / other.z
		};
	}
}

impl DivAssign<f32> for Vec3 {
	fn div_assign(&mut self, other: f32){
		*self = Self {
			x: self.x / other,
			y: self.y / other,
			z: self.z / other
		};
	}
}

impl Neg for Vec3{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Display for Vec3 {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let str = format!("{} {} {}", self.x, self.y, self.z);
		fmt.write_str(&str)?;
		Ok(())
		
	}
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_add(){
        let v1 = Vec3::new(1.0, 1.0, 0.0);
        let v2 = Vec3::new(0.0, 0.0, 1.0);

        let expected = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!( v1+v2, expected );
    }

    #[test]
    fn test_add_assign(){
        let mut v1 = Vec3::new(0.0, 1.0, 1.0);
        let v2 = Vec3::new(1.0, 0.0, 0.0);

        let expected = Vec3::new(1.0, 1.0, 1.0);

        v1+=v2;
        assert_eq!( v1, expected );
    }

    #[test]
    fn test_sub(){
        let v1 = Vec3::new(1.0, 1.0, 0.0);
        let v2 = Vec3::new(0.0, 0.0, 1.0);

        let expected = Vec3::new(1.0, 1.0, -1.0);

        assert_eq!( v1-v2, expected );
    }

    #[test]
    fn test_sub_assign(){
        let mut v1 = Vec3::new(0.0, 1.0, 1.0);
        let v2 = Vec3::new(1.0, 0.0, 0.0);

        let expected = Vec3::new(-1.0, 1.0, 1.0);

        v1-=v2;
        assert_eq!( v1, expected );
    }

    #[test]
    fn test_mul_vec(){
        let v1 = Vec3::new(0.1, 0.5, 0.7);
        let v2 = Vec3::new(1.0, 2.0, 1.0);

        let expected = Vec3::new(0.1, 1.0, 0.7);

        assert_eq!( v1*v2, expected );
    }

    #[test]
    fn test_mul_float(){
        let v1 = Vec3::new(0.1, 0.5, 0.7);
        let f1 = 0.5;

        let expected = Vec3::new(0.05, 0.25, 0.35);

        assert_eq!( v1*f1, expected );
    }

    #[test]
    fn test_mul_vec_assign(){
        let mut v1 = Vec3::new(0.1, 0.5, 0.7);
        let v2 = Vec3::new(1.0, 2.0, 1.0);

        let expected = Vec3::new(0.1, 1.0, 0.7);

        v1*=v2;
        assert_eq!( v1, expected );
    }

    #[test]
    fn test_mul_float_assign(){
        let mut v1 = Vec3::new(0.1, 0.5, 0.7);
        let f1 = 0.5;

        let expected = Vec3::new(0.05, 0.25, 0.35);
        
        v1*=f1;
        assert_eq!( v1, expected );
    }

    #[test]
    fn test_div_vec(){
        let v1 = Vec3::new(0.1, 0.5, 0.7);
        let v2 = Vec3::new(0.5, 2.0, 1.0);

        let expected = Vec3::new(0.2, 0.25, 0.7);

        assert_eq!( v1/v2, expected );
    }

    #[test]
    fn test_div_float(){
        let v1 = Vec3::new(0.1, 0.5, 0.7);
        let f1 = 0.5;

        let expected = Vec3::new(0.2, 1.0, 1.4);

        assert_eq!( v1/f1, expected );
    }

    #[test]
    fn test_div_vec_assign(){
        let mut v1 = Vec3::new(0.1, 0.5, 0.7);
        let v2 = Vec3::new(1.0, 2.0, 1.0);

        let expected = Vec3::new(0.1, 0.25, 0.7);

        v1/=v2;
        assert_eq!( v1, expected );
    }

    #[test]
    fn test_div_float_assign(){
        let mut v1 = Vec3::new(0.1, 0.5, 0.7);
        let f1 = 0.5;

        let expected = Vec3::new(0.2, 1., 1.4);
        
        v1/=f1;
        assert_eq!( v1, expected );
    }
    
    #[test]
    fn test_length_squared(){
        let v = Vec3::new(2.0, 0.0, 2.0);
        let len = v.length_squared();
        assert_eq!(len, 8.0);
    }
    
    #[test]
    fn test_length(){
        let v = Vec3::new(3.0, 4.0, 0.0);
        let len = v.length();
        assert_eq!(len, 5.0)
    }

    #[test]
    fn test_dot_perpendicular(){
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(Vec3::dot(v1, v2), 0.0);
    }
    
    #[test]
    fn test_dot_parallel(){
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(Vec3::dot(v1, v2), 1.0);
    }

    #[test]
    fn test_cross_perpendicular(){
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);

        let expected = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(Vec3::cross(v1, v2), expected);
    }
    
    #[test]
    fn test_cross_parallel(){
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(1.0, 0.0, 0.0);

        let expected = Vec3::new(0.0, 0.0, 0.0);

        assert_eq!(Vec3::cross(v1, v2), expected);
    }

    #[test]
    fn test_cross_111(){
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);

        let expected = Vec3::new(-1.0, 0.0, 1.0);

        assert_eq!(Vec3::cross(v1, v2), expected);
    }

    #[test]
    fn test_unit_shorten(){
        let v = Vec3::new(2.0, 0.0, 0.0);
        let expected = Vec3::new(1.0, 0.0, 0.0);
        
        assert_eq!(Vec3::as_unit(&v), expected);
    }

    #[test]
    fn test_unit_lengthen(){
        let v = Vec3::new(0.5, 0.0, 0.0);
        let expected = Vec3::new(1.0, 0.0, 0.0);
        
        assert_eq!(Vec3::as_unit(&v), expected);
    }

    #[test]
    fn test_unit_111(){
        let v = Vec3::new(1.0, 1.0, 1.0);
        let expected = Vec3::new(0.577350269,0.577350269,0.577350269);

        assert!(Vec3::as_unit(&v) <= expected * 1.001); // within very small under-estimate
        assert!(Vec3::as_unit(&v) >= expected * 0.999); // within very small over-estimate
    }
}

