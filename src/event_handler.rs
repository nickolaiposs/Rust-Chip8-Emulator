use sdl2::{Sdl, EventPump};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct EventHandler {
    event_pump: EventPump
}

impl EventHandler {
    pub fn new(sdl_context: &Sdl) -> Self {
        Self {
            event_pump: sdl_context.event_pump().unwrap()
        }
    }

    pub fn get_keypad(&mut self) -> Keypad {
        let mut keys = [false; 16];
        let mut terminate = false;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => { terminate = true },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => { keys[0x1] = true},
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => { keys[0x2] = true},
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => { keys[0x3] = true},
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => { keys[0xC] = true},
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => { keys[0x4] = true},
                Event::KeyDown { keycode: Some(Keycode::W), .. } => { keys[0x5] = true},
                Event::KeyDown { keycode: Some(Keycode::E), .. } => { keys[0x6] = true},
                Event::KeyDown { keycode: Some(Keycode::R), .. } => { keys[0xD] = true},
                Event::KeyDown { keycode: Some(Keycode::A), .. } => { keys[0x7] = true},
                Event::KeyDown { keycode: Some(Keycode::S), .. } => { keys[0x8] = true},
                Event::KeyDown { keycode: Some(Keycode::D), .. } => { keys[0x9] = true},
                Event::KeyDown { keycode: Some(Keycode::F), .. } => { keys[0xE] = true},
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => { keys[0xA] = true},
                Event::KeyDown { keycode: Some(Keycode::X), .. } => { keys[0x0] = true},
                Event::KeyDown { keycode: Some(Keycode::C), .. } => { keys[0xB] = true},
                Event::KeyDown { keycode: Some(Keycode::V), .. } => { keys[0xF] = true},
                _ => ()
            }
        }

        Keypad {
            keys,
            terminate
        }
    }
}

pub struct Keypad {
    pub keys: [bool; 16],
    pub terminate: bool
}