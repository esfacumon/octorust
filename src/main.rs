mod chip8;

use chip8::chip8::Chip8;

use sdl2::rect::Rect;

use std::time::Duration;


extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


use chip8::constants::{
    WIDTH,
    HEIGHT,
    SCALE_FACTOR,
    PADDING,
};

const SCREEN_WIDTH: u32 = (WIDTH * SCALE_FACTOR as usize) as u32;
const SCREEN_HEIGHT: u32  = (HEIGHT * SCALE_FACTOR as usize) as u32;

fn main() {
    println!("Octorust  Copyright (C) 2023  Facundo A.
    This program comes with ABSOLUTELY NO WARRANTY.
    This is free software, and you are welcome to redistribute it
    under certain conditions.");
    let mut chip8 = Chip8::new();

    let sdl_context = sdl2::init().expect("Init SDL2 error");
    let video_subsystem = sdl_context.video().expect("Init video subsystem error");

    let window = video_subsystem.window("rust-sdl2 demo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("Window init error");

    let mut canvas = window.into_canvas().build()
            .expect("Canvas build error");


    let mut event_pump = sdl_context.event_pump().expect("Event pump error");

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    // break 'running
                },
                _ => {}
            }
        }

        chip8.cycle();

        render(&chip8, &mut canvas);

        ::std::thread::sleep(Duration::new(0, 10_000_000u32 / 60) * 60);
    }
}

/**
Updates screen with pixel_array values
 */
fn render(chip8: &Chip8, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.clear();
    for (j, row) in chip8.pixel_array.iter().enumerate() {
        for (i, &pixel) in row.iter().enumerate() {
            let x = i as u32;
            let y = j as u32;

            let color;
            if pixel {
                color = Color::RGB(255, 255, 255);
            } else {
                color = Color::RGB(0, 0, 0);
            };

            canvas.set_draw_color(color);

            let rect = Rect::new(
                PADDING as i32 + (x as i32 * SCALE_FACTOR as i32),
                PADDING as i32 + (y as i32 * SCALE_FACTOR as i32),
                (SCALE_FACTOR - PADDING) as u32,
                (SCALE_FACTOR - PADDING) as u32);
            canvas.fill_rect(rect).expect("Error rendering pixel");
        }
    }
    canvas.present();
}
