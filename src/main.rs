mod cpu;
mod font;
mod game_loader;
mod event_handler;
mod graphics;

use std::{env, thread};
use std::time::{Duration, Instant};

use cpu::CPU;
use graphics::Graphics;
use event_handler::EventHandler;
use game_loader::game_loader;

fn main() {
    let filepath = load_filepath();
    let game_data = game_loader(&filepath);
    let sdl_context = sdl2::init().unwrap();

    let mut cpu = CPU::new();
    let mut screen = Graphics::new(&sdl_context, (960, 480));
    let mut event_handler = EventHandler::new(&sdl_context);

    let window_title = String::from("Chip8 Emulator");
    screen.set_title(&window_title);
    cpu.load(&game_data);

    //fps counter
    let mut second = 0;
    let mut fps = 0;
    let one_second = Duration::from_secs(1);
    let mut second_counter = Instant::now();

    let mut start = Instant::now();
    let tps = Duration::from_micros(1000000 / 700);
    
    'gameloop: loop {
        if second_counter.elapsed() >= one_second {
            second += 1;
            println!("Second {}, fps: {}", second, fps);
            fps = 0;
            second_counter = Instant::now();
        }

        let keypad = event_handler.get_keypad();

        if keypad.terminate {
            break 'gameloop;
        }

        cpu.tick(keypad.keys);

        if cpu.new_screen() {
            screen.render(cpu.get_vram());
        }

        fps += 1;
        
        if tps > start.elapsed() {
            thread::sleep(tps - start.elapsed());
        }

        start = Instant::now();
    }
}

fn load_filepath() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("Must specify filepath");
    }

    String::from(&args[1])
}
