#[macro_use]
extern crate log;
extern crate log4rs;

extern crate libc;
extern crate sdl2_sys;
extern crate sdl2;

mod moving;
mod square;
mod helpers;
mod snake;

use moving::Moving;
use moving::Direction;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;
use sdl2::mouse::MouseButton;
use sdl2::render::{TextureCreator, Canvas};
use sdl2::rect::Rect;
use helpers::*;
use snake::Snake;
use square::Square;
use sdl2::EventPump;
use rand::prelude::ThreadRng;
use sdl2::video::Window;

const WIDTH: u32 = 600;
//количество полей
const FIELD: u32 = 22;
const HEIGHT: u32 = 800;
//размер "квадратика
const BASE_SIZE: u32 = 20;
//отступ по краям
const L_SIZE: u32 = (600 - BASE_SIZE * FIELD) / 2;
// расстояние между гридом и граничкой
const BORDER_HEIGHT: u32 = 650;

macro_rules! vec_deq {
    ($($x:expr),*) =>{
        {
            let mut result = std::collections::VecDeque::new();
            $(
                result.push_front($x);
            )*
            result
        }
    };
}

struct SnakeGame {
    snake: crate::snake::Snake,
    point: i32,
    is_over: bool,
}

pub fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).expect("File not found or can't be read");
    info!("Logger is ready");

    let rng = rand::thread_rng();
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");
    let window:Window = video_subsystem.window("rust-sdl2 demo: Snake", WIDTH, HEIGHT)
        .position_centered().vulkan().build().expect("Failed to create window");

    let mut canvas = window.into_canvas()
        .target_texture().present_vsync().build().expect("Failed to convert window into canvas");

    let grid_left = L_SIZE;
    let grid_right = L_SIZE;
    let grid_top = HEIGHT - (BASE_SIZE * FIELD + L_SIZE);
    let grid_bottom = L_SIZE;
    info!("Grid values:\n\tleft:{}\n\tright:{}\n\ttop:{}\n\tbottom:{}", grid_left, grid_right, grid_top, grid_bottom);

    let creator: TextureCreator<_> = canvas.texture_creator();
    let grid = create_texture_rect(&mut canvas, &creator, TextureColor::Black, BASE_SIZE * FIELD).expect("Failed to create a texture");
    let border = create_texture_rect(&mut canvas, &creator, TextureColor::White, BASE_SIZE * FIELD + L_SIZE).expect("Failed to create a texture");

    let mut point = Square::from_position(random_position_in_grid(rng));
    let mut test_snake = Snake::from_position(vec_deq!((2, 2), (2, 3), (2, 4)));
    info!("Test snake:{:?}", &test_snake);
    //let test_snake_textures =
    let mut cube = Square::new(Direction::Bot);
    let snake_point = create_texture_rect(&mut canvas, &creator, TextureColor::Green, BASE_SIZE).expect("Failed to create a texture");
    let point_texture = create_texture_rect(&mut canvas, &creator, TextureColor::Blue, BASE_SIZE).expect("Failed to create a texture");


    canvas.set_draw_color(Color::RGB(255, 0, 0));

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
    let mut counter_loop = 0;
    let mut quit = false;
    'running: loop {
        handle_events(&mut event_pump, &mut quit, &mut cube, &mut test_snake, rng, &mut canvas);
        if quit == true {
            break 'running;
        }

        counter_loop += 1;
        if counter_loop >= 120 {
            //  point.set_position(random_position_in_grid(rng));
            counter_loop = 0;
        }
        if counter_loop % 5 == 0 {
            Snake::set_new_position_if_border(&mut test_snake, (BASE_SIZE * (FIELD - 1)) as i32);
            test_snake.move_in_direction();
        }

        Square::set_new_position_if_border(&mut cube, (BASE_SIZE * (FIELD - 1)) as i32);
        //Новое положение кубика, скорость опеределена числом
        for _ in 0..7 {
            if cube.consume_another_cube(&point) {
                info!("point!");
                point.set_position(random_position_in_grid(rng));
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
        for i in test_snake.get_position() {
            canvas.copy(&snake_point, None, Rect::new(i.0 + grid_left as i32, i.1 + grid_top as i32, BASE_SIZE, BASE_SIZE)).unwrap();
        }

        canvas.present();
        //60 FPS
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


fn handle_events(event_pump: &mut EventPump, quit: &mut bool, cube: &mut square::Square, test_snake: &mut snake::Snake, rng: ThreadRng, canvas: &mut Canvas<Window>) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                *quit = true;
                break;
            }
            Event::KeyDown { keycode: Some(Keycode::Up), .. } | Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                cube.change_direction(Direction::Top);
                test_snake.change_direction(Direction::Top);
            }
            Event::KeyDown { keycode: Some(Keycode::Down), .. } | Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                cube.change_direction(Direction::Bot);
                test_snake.change_direction(Direction::Bot);
            }
            Event::KeyDown { keycode: Some(Keycode::Left), .. } | Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                cube.change_direction(Direction::Left);
                test_snake.change_direction(Direction::Left);
            }
            Event::KeyDown { keycode: Some(Keycode::Right), .. } | Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                cube.change_direction(Direction::Right);
                test_snake.change_direction(Direction::Right)
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
}
