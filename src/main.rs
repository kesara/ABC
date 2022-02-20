extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::log;
use sdl2::mixer::{InitFlag, Chunk, DEFAULT_CHANNELS, AUDIO_S16LSB};
use sdl2::rect::Rect;
use std::path::Path;
use std::process;

// assets directory
const ASSETS_DIR: &'static str = "assets";

// default font file
const DEFAULT_FONT: &str = "knewave.ttf";

// const
const YENULI: &str = "YENULI";
const YELINSA: &str = "YELINSA";
const YESARA: &str = "YESARA";

fn get_asset(file: String) -> String {
    format!("{}/{}", ASSETS_DIR, file)
}

struct Letter {
    letter: char,
    played: bool,
}

impl Letter {
    fn new(name: char) -> Letter {
        Letter {
            letter: name,
            played: false,
        }
    }

    fn letter(&self) -> String {
        self.letter.to_string()
    }

    fn is_special(&self) -> bool {
        YENULI.contains(self.letter) || YELINSA.contains(self.letter) ||
            YESARA.contains(self.letter)
    }

    fn get_sound_path(&self) -> String {
        let sound_file = format!("{}.ogg", &self.letter.to_lowercase());
        get_asset(sound_file)
    }
}

fn main() {
    // boundaries
    let height: i32 = 480;
    let width: i32 = 640;
    let letter_box_size: u32 = 100;

    // SDL context
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();


    // mixer
    let _ =
        sdl2::mixer::open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024);
    let _ = sdl2::mixer::init(InitFlag::OGG);
    let _ = sdl2::mixer::allocate_channels(2);

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
    let font_path = get_asset(DEFAULT_FONT.to_string());
    let font = ttf_context.load_font(Path::new(&font_path), 512).unwrap();

    // screen
    let screen = Rect::new(0, 0, width as u32, height as u32);
    let letter_box = Rect::new(
        (width / 2) - (letter_box_size / 2) as i32,
        (height / 2) - (letter_box_size / 2) as i32,
        letter_box_size,
        letter_box_size,
    );

    // colours
    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);

    // letter
    let mut l: Letter = Letter::new(' ');
    let mut buildup: String = "".to_string();

    log::log("Starting ABC");
    let mut main_loop = || {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    // quit
                    log::log("Exiting ABC");
                    process::exit(1);
                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    l = Letter::new('A');
                }
                Event::KeyDown { keycode: Some(Keycode::B), .. } => {
                    l = Letter::new('B');
                }
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    l = Letter::new('C');
                }
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    l = Letter::new('D');
                }
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    l = Letter::new('E');
                }
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    l = Letter::new('F');
                }
                Event::KeyDown { keycode: Some(Keycode::G), .. } => {
                    l = Letter::new('G');
                }
                Event::KeyDown { keycode: Some(Keycode::H), .. } => {
                    l = Letter::new('H');
                }
                Event::KeyDown { keycode: Some(Keycode::I), .. } => {
                    l = Letter::new('I');
                }
                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
                    l = Letter::new('J');
                }
                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    l = Letter::new('K');
                }
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    l = Letter::new('L');
                }
                Event::KeyDown { keycode: Some(Keycode::M), .. } => {
                    l = Letter::new('M');
                }
                Event::KeyDown { keycode: Some(Keycode::N), .. } => {
                    l = Letter::new('N');
                }
                Event::KeyDown { keycode: Some(Keycode::O), .. } => {
                    l = Letter::new('O');
                }
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    l = Letter::new('P');
                }
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    l = Letter::new('Q');
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    l = Letter::new('R');
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    l = Letter::new('S');
                }
                Event::KeyDown { keycode: Some(Keycode::T), .. } => {
                    l = Letter::new('T');
                }
                Event::KeyDown { keycode: Some(Keycode::U), .. } => {
                    l = Letter::new('U');
                }
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                    l = Letter::new('V');
                }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    l = Letter::new('W');
                }
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    l = Letter::new('X');
                }
                Event::KeyDown { keycode: Some(Keycode::Y), .. } => {
                    l = Letter::new('Y');
                }
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    l = Letter::new('Z');
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
        let _ = renderer.fill_rect(letter_box);
        if l.letter != ' ' {
            let text_render =
                font.render(&l.letter().to_string()).blended(white).unwrap();
            let texture_creator = renderer.texture_creator();
            let texture = texture_creator
                .create_texture_from_surface(&text_render)
                .unwrap();
            renderer.copy(&texture, None, Some(letter_box)).unwrap();
        }

        // present
        let _ = renderer.present();

        // play sound
        if l.letter != ' ' {
            if !l.played {
                let sound = Chunk::from_file(Path::new(&l.get_sound_path()))
                    .unwrap();
                let _ = sdl2::mixer::Channel::all().play(&sound, 0);
                l.played = true;
                timer.delay(500);

                if l.is_special() {
                    if buildup.is_empty() &&
                        (YENULI.starts_with(l.letter) ||
                             YELINSA.starts_with(l.letter) ||
                             YESARA.starts_with(l.letter))
                    {
                        buildup = l.letter.to_string();

                    } else {
                        buildup = format!("{}{}", buildup, l.letter);
                        if YENULI.eq(&buildup) ||
                            YELINSA.eq(&buildup) ||
                            YESARA.eq(&buildup)
                        {
                            log::log(&format!("Play {}", buildup));
                            let sound_file =
                                format!("{}.ogg", buildup.to_lowercase());
                            let path = get_asset(sound_file);
                            let sound = Chunk::from_file(Path::new(&path))
                                .unwrap();
                            let _ = sdl2::mixer::Channel::all().play(&sound, 0);
                            buildup = "".to_string();
                            timer.delay(1100);
                        }
                        if !(YENULI.starts_with(&buildup) ||
                                 YELINSA.starts_with(&buildup) ||
                                 YESARA.starts_with(&buildup))
                        {
                            buildup = "".to_string();
                        }
                    }
                }
            }
        }
    };

    loop {
        main_loop();
    }
}
