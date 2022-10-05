//translates the VPLL code to the sdl2 rust wrapper equivalent.

use std::thread;
use std::time::Duration;
use json::JsonValue;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::glob;
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


    let mut global_value = 0;
    let mut first_time:bool = true;
    'running: loop {
        //background
        canvas.set_draw_color(Color::RGB(r, g, b));
        canvas.clear();




        // The rest of the game loop goes here...

        for element in translated_json["elements"].members()
        {
            canvas.set_draw_color(Color::RGB(element["r"].as_str().unwrap().parse::<u8>().unwrap(), element["g"].as_str().unwrap().parse::<u8>().unwrap(), element["b"].as_str().unwrap().parse::<u8>().unwrap()));

            match element["type"].as_str().unwrap() {
                "rectangle" => {
                    canvas.draw_rect(Rect::new(
                        to_i32(element["x"].to_string(), global_value as i32),
                        to_i32(element["y"].to_string(), global_value as i32),
                        to_i32(element["width"].to_string(), global_value as i32) as u32,
                        to_i32(element["height"].to_string(), global_value as i32) as u32))
                        .expect("Could not make rectangle.");
                }
                "fill_rectangle" => {
                    canvas.fill_rect(Rect::new(
                        to_i32(element["x"].to_string(), global_value as i32),
                        to_i32(element["y"].to_string(), global_value as i32),
                        to_i32(element["width"].to_string(), global_value as i32) as u32,
                        to_i32(element["height"].to_string(), global_value as i32) as u32))
                        .expect("Could not make rectangle.");
                }
                "line"  => {
                    canvas.draw_line(
                        Point::new(
                            to_i32(element["x1"].to_string(), global_value as i32),
                            to_i32(element["y1"].to_string(), global_value as i32)
                        ),
                        Point::new(
                            to_i32(element["x2"].to_string(), global_value as i32),
                            to_i32(element["y2"].to_string(), global_value as i32)
                        ))
                        .expect("Could not draw a line.");
                }
                "var" => {
                    if first_time
                    {
                        global_value = to_i32(element["value"].to_string(), global_value as i32);
                        first_time = false;
                    }
                    else
                    {
                        global_value += to_i32(element["increment"].to_string(), global_value as i32);
                    }
                }
                "circle"  => {
                    let radius = if element["radius"].as_str().unwrap().parse::<f32>().is_err() {
                        global_value as f32
                    }
                    else {
                        element["radius"].as_str().unwrap().parse::<f32>().unwrap()
                    };
                    let x_offset = to_f32(element["x"].to_string(), global_value as f32);
                    let y_offset = to_f32(element["y"].to_string(), global_value as f32);

                    for i in 0..6300 {
                        let i = (i as f32) * 0.001;

                        let x = (i).cos()*radius+x_offset;
                        let y = (i).sin()*radius+y_offset;

                        canvas.draw_point(Point::new(x as i32, y as i32)).expect("Could not draw point for the cirlce");
                    }
                }
                "sleep"  => {
                    thread::sleep(Duration::new(
                        element["seconds"].as_str().unwrap().parse::<u64>().unwrap(),
                        element["nanoseconds"].as_str().unwrap().parse::<u32>().unwrap()
                    ));
                    canvas.set_draw_color(Color::RGB(r, g, b));
                    canvas.clear();
                }
                "present" => {
                    canvas.present();
                }
                "clear"  => {
                    canvas.set_draw_color(Color::RGB(r, g, b));
                    canvas.clear();
                }
                _ => {}
            }


            //Check for polling events to be a bit more responsive
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
        }


        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn to_i32(value: String, global_value: i32) -> i32
{
     let return_value =  if value.parse::<i32>().is_err() {
        global_value
    }
    else {
        value.parse::<i32>().unwrap()
    };

    return return_value;
}

fn to_f32(value: String, global_value: f32) -> f32
{
    let return_value =  if value.parse::<f32>().is_err() {
        global_value
    }
    else {
        value.parse::<f32>().unwrap()
    };

    return return_value;
}

fn to_u32(value: String) -> u32
{
    return value.parse::<u32>().unwrap();
}