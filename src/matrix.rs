use crate::Vector2;

pub struct Matrix {
   pub m: [[f32; 3]; 3],
}

impl Matrix {
   // constructors
   pub fn zero() -> Matrix {
      Matrix {
           m: [[0.0; 3]; 3],
       }
   }

   pub fn identity() -> Matrix {
      Matrix {
           m: [[1.0, 0.0, 0.0], 
               [0.0, 1.0, 0.0], 
               [0.0, 0.0, 1.0]],
       }
   }

   pub fn set(pos: &crate::Vector2, radian: f64) -> Matrix {
      Matrix {
           m: [[radian.cos() as f32, radian.sin() as f32, 0.0], 
               [-radian.sin() as f32, radian.cos() as f32, 0.0], 
               [pos.x, pos.y, 1.0]],
       }
   }
}

mod tests {
   use super::*;

   #[test]
   fn zero() {
      let m = Matrix::zero();
      assert_eq!(m.m[0][0], 0.0);
      assert_eq!(m.m[0][1], 0.0);
      assert_eq!(m.m[0][2], 0.0);
      assert_eq!(m.m[1][0], 0.0);
      assert_eq!(m.m[1][1], 0.0);
      assert_eq!(m.m[1][2], 0.0);
      assert_eq!(m.m[2][0], 0.0);
      assert_eq!(m.m[2][1], 0.0);
      assert_eq!(m.m[2][2], 0.0);
   }

   #[test]
   fn identity() {
      let m = Matrix::identity();
      assert_eq!(m.m[0][0], 1.0);
      assert_eq!(m.m[0][1], 0.0);
      assert_eq!(m.m[0][2], 0.0);
      assert_eq!(m.m[1][0], 0.0);
      assert_eq!(m.m[1][1], 1.0);
      assert_eq!(m.m[1][2], 0.0);
      assert_eq!(m.m[2][0], 0.0);
      assert_eq!(m.m[2][1], 0.0);
      assert_eq!(m.m[2][2], 1.0);
   }

   #[test]
   fn set() {
      let position: crate::Vector2 = crate::Vector2{x:2.0, y:2.0};
      let radian : f64 = 1.5708;
      let m = Matrix::set(&position, radian);
      assert_eq!(m.m[0][0], radian.cos() as f32);
      assert_eq!(m.m[0][1], radian.sin() as f32);
      assert_eq!(m.m[0][2], 0.0);
      assert_eq!(m.m[1][0], -radian.sin() as f32);
      assert_eq!(m.m[1][1], radian.cos() as f32);
      assert_eq!(m.m[1][2], 0.0);
      assert_eq!(m.m[2][0], position.x);
      assert_eq!(m.m[2][1], position.y);
      assert_eq!(m.m[2][2], 1.0);
   }
}