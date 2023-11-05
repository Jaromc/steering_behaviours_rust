extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
use rand::Rng;

use vector2::Vector2;
mod vector2;

use matrix::Matrix;
mod matrix;

use bot::Bot;
mod bot;

use params::Params;
mod params;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    let params = Params::load_from_file("src\\params.txt");

    //create bots
    let mut rng = rand::thread_rng();
    let mut bot_list = Vec::new();
    let mut colour_list: Vec<Color> = Vec::new();
    for i in 0..20
    {
        let mut bot = Bot::new();
        let xpos : i32 = rng.gen_range(0..params.get_window_x() as i32);
        let ypos : i32 = rng.gen_range(0..params.get_window_y() as i32);
        let pos = &Vector2 { x: xpos as f32, y: ypos as f32 };
        bot.set_positon(pos);
        bot.max_speed = 50.0;
        bot_list.push(bot);

        colour_list.push(Color::RGB(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)));
    }

    let mut now: Instant = Instant::now();
    
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        
        let dt = now.elapsed();
        now = Instant::now();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        //move and render bots
        let mut colour_idx = 0;
        for bot in &mut  bot_list {
            bot.update(&params, dt.as_secs_f64());

            canvas.set_draw_color(colour_list[colour_idx]);

            colour_idx += 1;

            let bot_pos = bot.get_positon();
            canvas.fill_rect(Rect::new(bot_pos.x as i32, bot_pos.y as i32, 20, 20));
        }

        canvas.present();
    }

    Ok(())
}
