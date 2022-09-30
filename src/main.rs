mod parser;

extern crate sdl2;

use std::{env, fs};
use std::ops::Index;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use json::JsonValue;
use sdl2::rect::Rect;


fn get_program_file_content() -> String
{
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("You have to supply a file name!");

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    return contents;
}

pub fn main() {



    println!("{}", parser::convert_vpll_to_json(get_program_file_content()))


    /*let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("VPLL", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        canvas.set_draw_color(Color::RED);
        canvas.draw_rect(Rect::new(100, 100, 100, 100)).expect("TODO: panic message");
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }*/
}