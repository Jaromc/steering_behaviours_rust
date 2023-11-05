use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::fs::read_to_string;

pub struct Params {
   wander_distance : f32,
   wander_rate : f32,
   wander_radius : f32,
   window_x: f32,
   window_y: f32
}

impl Params {
   pub fn new() -> Params {
      Params {
         wander_distance: 0.0,
         wander_rate: 0.0,
         wander_radius: 0.0,
         window_x: 0.0,
         window_y: 0.0
       }
   }

   pub fn default() -> Params {
      Params {
         wander_distance: 200.0,
         wander_rate: 0.2,
         wander_radius: 100.0,
         window_x: 800.0,
         window_y: 600.0
       }
   }

   pub fn get_wander_distance(&self) -> f32 {
      self.wander_distance
   }

   pub fn get_wander_rate(&self) -> f32 {
      self.wander_rate
   }

   pub fn get_wander_radius(&self) -> f32 {
      self.wander_radius
   }

   pub fn get_window_x(&self) -> f32 {
      self.window_x
   }

   pub fn get_window_y(&self) -> f32 {
      self.window_y
   }

   fn get_number(s: &str) -> Option<f32>
   {
      s.split(':')
      .skip(1)    // skip "user"
      .next()?    // the ? returns None if necessary
      .parse()    // result is Result<f32,...>
      .ok()       // transforms a Result into an Option
   }

   pub fn load_from_string(contents : &str) -> Params {
      let mut params : Params = Params::new();

      let mut lines = contents.lines();
      params.wander_distance = Params::get_number(lines.next().unwrap()).unwrap();
      params.wander_rate = Params::get_number(lines.next().unwrap()).unwrap();
      params.wander_radius = Params::get_number(lines.next().unwrap()).unwrap();
      params.window_x = Params::get_number(lines.next().unwrap()).unwrap();
      params.window_y = Params::get_number(lines.next().unwrap()).unwrap();

      return params;
   }

   pub fn load_from_file(file_path : &str) -> Params {
      println!("{}", file_path);
      let res = read_to_string(file_path);

      let res_file = match res {
         Ok(file) => file,
         Err(error) => panic!("Problem opening the file: {:?}", error),
     };
     
      return Params::load_from_string(&res_file[..]);
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn parse() {
      let st: &str = "wander_distance:200.0
      wander_rate:0.2
      wander_radius:100.0
      window_x:800.0
      window_y:600.0
      ";

      let params = Params::load_from_string(st);

      assert_eq!(params.wander_distance, 200.0);
      assert_eq!(params.wander_rate, 0.2);
      assert_eq!(params.wander_radius, 100.0);
   }

   #[test]
   fn load_from_file() {
      let params = Params::load_from_file("src\\params.txt");

      assert_eq!(params.wander_distance, 200.0);
      assert_eq!(params.wander_rate, 0.2);
      assert_eq!(params.wander_radius, 100.0);
   }
}