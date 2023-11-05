use std::ops::{Mul, AddAssign};

pub struct Vector2 {
   pub x: f32,
   pub y: f32,
}

impl Vector2 {
   pub fn zero() -> Vector2 {
      Vector2 {x:0.0, y:0.0}
   }

   pub fn dot(&self, r: &Vector2) -> f32{
      (self.x * r.x) + (self.y * r.y)
   }

   pub fn length(&self) -> f32{
      f32::sqrt((self.x * self.x) + (self.y * self.y))
   }

   pub fn length_sq(&self) -> f32{
      (self.x * self.x) + (self.y * self.y)
   }

   pub fn get_normalise(&self) -> Vector2{
      let l = self.length();
      if l == 0.0 {
         return Vector2{x:0.0, y:0.0};
      }

      Vector2 {x:self.x/l, y:self.y/l}
   }
}

impl Mul<f32> for &Vector2 {
   type Output = Vector2;

   fn mul(self, rhs: f32) -> Self::Output {
      Vector2{x:self.x * rhs, y:self.y * rhs}
   }
}

impl Mul<f64> for &Vector2 {
   type Output = Vector2;

   fn mul(self, rhs: f64) -> Self::Output {
      let x = self.x as f64 * rhs;
      let y = self.y as f64 * rhs;
      Vector2{x:x as f32, y:y as f32}
   }
}

impl AddAssign for Vector2 {
   fn add_assign(&mut self, other: Self) {
      *self = Self {
          x: self.x + other.x,
          y: self.y + other.y,
      };
  }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn zero() {
      let v :Vector2 = Vector2::zero();
      assert_eq!(v.x, 0.0);
      assert_eq!(v.y, 0.0);
   }

   #[test]
   fn dot() {
      let v2 :Vector2 = Vector2 {x:10.0, y:20.0};
      let v1 :Vector2 = Vector2 {x:10.0, y:20.0};
      assert_eq!(v1.dot(&v2), 500.0);
   }

   #[test]
   fn length() {
      let v :Vector2 = Vector2 {x:10.0, y:20.0};
      assert_eq!(v.length(), 22.36068);
   }

   #[test]
   fn length_sq() {
      let v :Vector2 = Vector2 {x:10.0, y:20.0};
      assert_eq!(v.length_sq(), 500.0);
   }

   #[test]
   fn get_normalise() {
      let v = Vector2 {x:10.0, y:10.0};
      let n = v.get_normalise();
      assert_eq!(n.x, 0.70710677);
      assert_eq!(n.y, 0.70710677);
   }

   #[test]
   fn scaler_mul() {
      let v = Vector2 {x:10.0, y:10.0};
      let r: Vector2 = &v * 2.0;
      assert_eq!(r.x, 20.0);
      assert_eq!(r.y, 20.0);
      //make sure original hasn't changed
      assert_eq!(v.x, 10.0);
      assert_eq!(v.y, 10.0);
   }

   #[test]
   fn assign() {
      let mut v = Vector2 {x:20.0, y:20.0};
      v += Vector2 {x:10.0, y:20.0};
      assert_eq!(v.x, 30.0);
      assert_eq!(v.y, 40.0);
   }
}