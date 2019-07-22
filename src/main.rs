#[macro_use]
extern crate log;
extern crate log4rs;

extern crate libc;
extern crate sdl2_sys;
extern crate sdl2;

mod moving;
mod moving_entities;
mod helpers;
mod snake;

use crate::moving::Moving;
use crate::moving::Direction;
use crate::moving_entities::Cube;

use sdl2::pixels::{Color};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;
use sdl2::mouse::MouseButton;
use sdl2::render::{TextureCreator};
use sdl2::rect::Rect;
use helpers::*;
use crate::snake::Snake;

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

macro_rules! texture {
        ($r:expr, $g:expr, $b:expr) => (
            create_texture_rect(&mut canvas, &texture_creator, TextureColor::Green, BASE_SIZE as u32).unwrap()
        )
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
    let test_snake =Snake::from_position((2,2),vec![(2,3),(2,4)]);
    //let test_snake_textures =
    let mut cube = Cube::new(Direction::Bot);
    let snake_point = create_texture_rect(&mut canvas, &creator, TextureColor::Green, BASE_SIZE).expect("Failed to create a texture");
    let point_texture = create_texture_rect(&mut canvas, &creator, TextureColor::Blue, BASE_SIZE).expect("Failed to create a texture");


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
        for _ in 0..7 {
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

        for i in test_snake.position.tail.iter(){
            canvas.copy(&snake_point, None, Rect::new(i.0+ grid_left as i32, i.1 + grid_top as i32, BASE_SIZE, BASE_SIZE)).unwrap();
        }

        canvas.present();
        //60 FPS
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


//fn handle_events
