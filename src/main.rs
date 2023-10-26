mod stack;

mod chip8;
pub mod errors;
use chip8::Chip8;
use sdl2::rect::Rect;

// use std::{thread, time};
use std::time::Duration;

// use rusttype::Rect;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const CHIP8_WIDTH: u32 = 64;
const CHIP8_HEIGHT: u32 = 32;
const SCALE_FACTOR: u32 = 20;
const PADDING: u32 = SCALE_FACTOR/8;
const WIDTH: u32 = CHIP8_WIDTH * SCALE_FACTOR;
const HEIGHT: u32  = CHIP8_HEIGHT * SCALE_FACTOR;

fn main() {
    println!("Octorust  Copyright (C) 2023  Facundo A.
    This program comes with ABSOLUTELY NO WARRANTY.
    This is free software, and you are welcome to redistribute it
    under certain conditions.");
    let mut chip8 = Chip8::new();

    let sdl_context = sdl2::init().expect("Init SDL2 error");
    let video_subsystem = sdl_context.video().expect("Init video subsystem error");

    let window = video_subsystem.window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("Window init error");

    let mut canvas = window.into_canvas().build()
            .expect("Canvas build error");

    // canvas.set_draw_color(Color::RGB(255, 0, 0));
    // canvas.clear();
    // canvas.present();

    let mut event_pump = sdl_context.event_pump().expect("Event pump error");

    // let mut i = 0;
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

        ::std::thread::sleep(Duration::new(0, 1_000_000u32 / 60) * 60);
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
            

            // draw pixel on canvas
            let rect = Rect::new(PADDING as i32 + (x as i32 * SCALE_FACTOR as i32), PADDING as i32 + (y as i32 * SCALE_FACTOR as i32), SCALE_FACTOR - PADDING, SCALE_FACTOR - PADDING);
            canvas.fill_rect(rect).expect("Error rendering pixel");
        }
    }
    canvas.present();
}
