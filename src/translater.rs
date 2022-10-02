//translates the VPLL code to the sdl2 rust wrapper equivalent.

use std::thread;
use std::time::Duration;
use json::JsonValue;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

pub fn start_sdl2(translated_json: JsonValue)
{
    println!("{}", translated_json);
    let (window_width, window_height) = (translated_json["window"]["width"].as_str().unwrap().parse::<u32>().unwrap(), translated_json["window"]["height"].as_str().unwrap().parse::<u32>().unwrap());
    let (r, g, b) = (translated_json["background"]["r"].as_str().unwrap().parse::<u8>().unwrap(), translated_json["background"]["g"].as_str().unwrap().parse::<u8>().unwrap(), translated_json["background"]["b"].as_str().unwrap().parse::<u8>().unwrap());

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("VPLL", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        //background
        canvas.set_draw_color(Color::RGB(r, g, b));
        canvas.clear();


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

        for element in translated_json["elements"].members()
        {
            canvas.set_draw_color(Color::RGB(element["r"].as_str().unwrap().parse::<u8>().unwrap(), element["g"].as_str().unwrap().parse::<u8>().unwrap(), element["b"].as_str().unwrap().parse::<u8>().unwrap()));

            match element["type"].as_str().unwrap() {
                "rectangle" => {
                    canvas.draw_rect(Rect::new(
                        element["x"].as_str().unwrap().parse::<i32>().unwrap(),
                        element["y"].as_str().unwrap().parse::<i32>().unwrap(),
                        element["width"].as_str().unwrap().parse::<u32>().unwrap(),
                        element["height"].as_str().unwrap().parse::<u32>().unwrap()))
                        .expect("Could not make rectangle.");
                }
                "fill_rectangle" => {
                    canvas.fill_rect(Rect::new(
                        element["x"].as_str().unwrap().parse::<i32>().unwrap(),
                        element["y"].as_str().unwrap().parse::<i32>().unwrap(),
                        element["width"].as_str().unwrap().parse::<u32>().unwrap(),
                        element["height"].as_str().unwrap().parse::<u32>().unwrap()))
                        .expect("Could not make rectangle.");
                }
                "line"  => {
                    canvas.draw_line(
                        Point::new(
                            element["x1"].as_str().unwrap().parse::<i32>().unwrap(),
                            element["y1"].as_str().unwrap().parse::<i32>().unwrap()
                        ),
                        Point::new(
                            element["x2"].as_str().unwrap().parse::<i32>().unwrap(),
                            element["y2"].as_str().unwrap().parse::<i32>().unwrap()
                        ))
                        .expect("Could not draw a line.");
                }
                "sleep"  => {
                    thread::sleep(Duration::new(
                        element["seconds"].as_str().unwrap().parse::<u64>().unwrap(),
                        element["nanoseconds"].as_str().unwrap().parse::<u32>().unwrap()
                    ));
                }
                "present" => {
                    canvas.present();
                }
                "clear"  => {
                    canvas.set_draw_color(Color::RGB(r, g, b));
                    canvas.clear();
                    canvas.present();
                }
                _ => {}
            }
        }


        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}