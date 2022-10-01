//translates the VPLL code to the sdl2 rust wrapper equivalent.

use std::time::Duration;
use json::JsonValue;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

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

    canvas.set_draw_color(Color::RGB(r, g, b));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();


    'running: loop {
        canvas.set_draw_color(Color::RGB(r, g, b));
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
    }
}