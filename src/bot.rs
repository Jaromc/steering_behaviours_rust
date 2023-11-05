use crate::Vector2;
use crate::Params;
use rand::{thread_rng, Rng};

pub struct Bot {
   facing: Vector2,
   position: Vector2,
   force: Vector2,
   velocity: Vector2,
   pub max_speed: f32,
   wander_angle: f64
}

impl Bot {
   pub fn new() -> Bot {
      Bot {
         facing:Vector2::zero(),
         position:Vector2::zero(),
         force:Vector2::zero(),
         velocity:Vector2::zero(),
         max_speed:0.0,
         wander_angle:0.0
       }
   }

   pub fn get_positon(&self) -> Vector2 {
      Vector2{x: self.position.x, y: self.position.y}
   }

   pub fn set_positon(&mut self, pos: &Vector2) {
      self.position.x = pos.x;
      self.position.y = pos.y;
   }

   fn seek(&mut self, target : Vector2) {
      let mut direction = Vector2::zero();
      direction.x = target.x - self.position.x;
      direction.y = target.y - self.position.y;

      if direction.length() <= 0.5
      {
         //arrived at target
         return;
      }

      direction = direction.get_normalise();
      self.force += &direction * self.max_speed;
   }

   fn wander(&mut self, params: &Params) {
      let mut target= &self.velocity.get_normalise() * params.get_wander_distance();
      target += Vector2 {x : self.position.x, y : self.position.y};

      let mut rng = rand::thread_rng(); 
      self.wander_angle += rng.gen_range(-1.0..=1.0) * params.get_wander_rate() as f64;

      let offset = Vector2{x: params.get_wander_radius() * self.wander_angle.cos() as f32,
                                    y: params.get_wander_radius() * self.wander_angle.sin() as f32};

      
      target += offset;
      self.seek(target);
   }

   pub fn update(&mut self, params: &Params, dt: f64) {
      self.wander(&params);

      self.velocity = &self.force * dt;
      if self.velocity.length_sq() > 0.0{
         self.facing = self.velocity.get_normalise();
      }

      self.position += Vector2{x:self.velocity.x, y:self.velocity.y};

      if self.position.x > params.get_window_x() {
         self.position.x = 0.0;
      }
      if self.position.x < 0.0 {
         self.position.x = params.get_window_x();
      }
      if self.position.y > params.get_window_y() {
         self.position.y = 0.0;
      }
      if self.position.y < 0.0 {
         self.position.y = params.get_window_y();
      }

      self.force = Vector2::zero();
   }

}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn new() {
      let bot :Bot = Bot::new();
      assert_eq!(bot.get_positon().x, 0.0);
      assert_eq!(bot.get_positon().y, 0.0);
   }

   #[test]
   fn set_get_positon() {
      let mut bot :Bot = Bot::new();
      bot.set_positon(&Vector2 { x:1.0, y: 1.0});
      assert_eq!(bot.get_positon().x, 1.0);
      assert_eq!(bot.get_positon().y, 1.0);
   }

}
