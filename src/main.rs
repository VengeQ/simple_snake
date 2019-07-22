#[macro_use]
extern crate log;
extern crate log4rs;

extern crate libc;
extern crate sdl2_sys;
extern crate gfx;
extern crate gfx_window_sdl;
extern crate sdl2;

use gfx::Device;

mod moving;
mod moving_entities;

use crate::moving::Moving;
use crate::moving::Direction;
use crate::moving_entities::Cube;


use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;
use sdl2::mouse::MouseButton;
use rand::rngs::ThreadRng;
use rand::Rng;
use sdl2::render::{TextureCreator, Texture, Canvas, TextureAccess};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};
use std::fs::DirEntry;


#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
    Red,
    Black,
    White,
}

const WIDTH: u32 = 600;
//количество полей
const FIELD: u32 = 20;
const HEIGHT: u32 = 800;
//размер "квадратика
const BASE_SIZE: u32 = 20;
//отступ по краям
const L_SIZE: u32 = (600 - BASE_SIZE * FIELD) / 2;
// расстояние между гридом и граничкой
const BORDER_HEIGHT: u32 = 650;

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, color: TextureColor, size: u32) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) =
    texture_creator.create_texture_target(None, size, size) {
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            match color {
                TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
                TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
                TextureColor::Red => texture.set_draw_color(Color::RGB(255, 0, 0)),
                TextureColor::Black => texture.set_draw_color(Color::RGB(0, 0, 0)),
                TextureColor::White => texture.set_draw_color(Color::RGB(255, 255, 255)),
            }
            texture.clear();
        }).expect("Failed to color a texture");
        Some(square_texture)
    } else {
        // An error occured so we return nothing and let the function caller handle the error.
        None
    }
}

pub fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).expect("File not found or can't be read");
    info!("Logger is ready");

    let rng = rand::thread_rng();
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");
    let window = video_subsystem.window("rust-sdl2 demo: Snake", WIDTH, HEIGHT)
        .position_centered()
        .vulkan()
        .build()
        .expect("Failed to create window");


    let grid_left = L_SIZE;
    let grid_right = L_SIZE;
    let grid_top = HEIGHT - (BASE_SIZE * FIELD + L_SIZE);
    let grid_bottom = L_SIZE;

    info!("Grid values:\n\tleft:{}\n\tright:{}\n\ttop:{}\n\tbottom:{}", grid_left, grid_right, grid_top, grid_bottom);


    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Failed to convert window into canvas");

    let creator: TextureCreator<_> = canvas.texture_creator();
    let grid = create_texture_rect(&mut canvas, &creator, TextureColor::Black, BASE_SIZE * FIELD).expect("Failed to create a texture");
    let border = create_texture_rect(&mut canvas, &creator, TextureColor::White, BASE_SIZE * FIELD + L_SIZE).expect("Failed to create a texture");

    let mut point = Cube::from_position(random_position_in_grid(rng));
    let mut cube = Cube::new(Direction::Bot);
    let mut snake_point = create_texture_rect(&mut canvas, &creator, TextureColor::Green, BASE_SIZE).expect("Failed to create a texture");
    let mut point_texture = create_texture_rect(&mut canvas, &creator, TextureColor::Blue, BASE_SIZE).expect("Failed to create a texture");
    canvas.set_draw_color(Color::RGB(255, 0, 0));

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
    let mut counter_loop = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    {
                        break 'running;
                    }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } | Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    { cube.change_direction(Direction::Top) }
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } | Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    { cube.change_direction(Direction::Bot) }
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } | Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    { cube.change_direction(Direction::Left) }
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } | Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    { cube.change_direction(Direction::Right) }
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    cube.pause();
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, clicks: 1, .. } =>
                    {
                        canvas.set_draw_color(rand_color(rng));
                    }
                _ => {}
            }
        }


        counter_loop += 1;
        if counter_loop >= 120 {
            point.set_position(random_position_in_grid(rng));
            counter_loop = 0;
        }

        Cube::set_new_position_if_border(&mut cube, (FIELD * (BASE_SIZE - 1)) as i32);
        //Новое положение кубика, скорость опеределена числом
        for i in 0..7 {
            if cube.consume_another_cube(&point) {
                info!("point!");
                counter_loop = 360;
            };
            debug!("cube_pos:{:?}", cube.get_position());
            cube.move_in_direction();
        }

        // We draw it.
        canvas.clear();
        canvas.copy(&border, None, Rect::new((L_SIZE / 2) as i32, (HEIGHT - BORDER_HEIGHT - L_SIZE / 2) as i32, L_SIZE + BASE_SIZE * FIELD, BORDER_HEIGHT)).unwrap();
        canvas.copy(&grid, None, Rect::new(L_SIZE as i32, (HEIGHT - (L_SIZE + BASE_SIZE * FIELD)) as i32, BASE_SIZE * FIELD, BASE_SIZE * FIELD)).unwrap();
        canvas.copy(&snake_point, None, Rect::new(cube.get_position().0 + grid_left as i32, cube.get_position().1 + grid_top as i32, BASE_SIZE, BASE_SIZE)).unwrap();
        canvas.copy(&point_texture, None, Rect::new(point.get_position().0 + grid_left as i32, point.get_position().1 + grid_top as i32, BASE_SIZE, BASE_SIZE)).unwrap();
        canvas.present();
        //60 FPS
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


fn rand_color(mut rng: ThreadRng) -> sdl2::pixels::Color {
    Color::RGB(rng.gen(), rng.gen(), rng.gen())
}

fn new_color(t: &mut Texture, c: &mut Canvas<Window>, rng: ThreadRng) {
    c.with_texture_canvas(t, |texture| {
        texture.set_draw_color(rand_color(rng));
        texture.clear();
    }).unwrap()
}

struct Snake {
    length: i16,
    direction: Direction,

}
//fn handle_events

fn random_position_in_grid(mut rng: ThreadRng) -> (i32, i32) {
    let x = rng.gen_range(0, (FIELD - 1)) * BASE_SIZE;
    let y = rng.gen_range(0, (FIELD - 1)) * BASE_SIZE;
    info!("pos:{:?}", (x as i32, y as i32));
    (x as i32, y as i32)
}