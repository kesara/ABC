extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::log;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::process;
use std::path::Path;

fn main() {
    // boundaries
    let height: i32 = 480;
    let width: i32 = 640;

    // SDL context
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    // main window
    let window = match video
        .window("ABC", width as u32, height as u32)
        .position_centered()
        .opengl()
        .build() {
        Ok(window) => window,
        Err(err) => panic!("Failed to create window: {}", err),
    };

    // window renderer
    let mut renderer = match window.into_canvas().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("Failed to create renderer {}", err),
    };

    // font configuration
    let font_path = Path::new("assets/knewave.ttf");
    let font = ttf_context.load_font(font_path, 128).unwrap();

    // screen
    let screen = Rect::new(0, 0, width as u32, height as u32);

    // colours
    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);

    // letter
    let mut letter = " ";

    let mut main_loop = || {
        log::log("Starting ABC");
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    // quit
                    log::log("Exiting ABC");
                    process::exit(1);
                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    letter = "A";
                }
                _ => {
                    // pass
                }
            };
        }
        // draw main window
        let _ = renderer.set_draw_color(black);
        let _ = renderer.clear();
        let _ = renderer.fill_rect(screen);
        let text_render = font.render(letter).blended(white).unwrap();
        let texture_creator = renderer.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&text_render)
            .unwrap();
        renderer.copy(&texture, None, Some(screen)).unwrap();

        // present
        let _ = renderer.present();
    };

    loop {
        main_loop();
    }
}
