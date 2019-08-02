#[macro_use]
extern crate log;
extern crate log4rs;

extern crate libc;
extern crate sdl2_sys;
extern crate sdl2;

mod moving;
mod helpers;
mod snake_game;
mod rendering;

use moving::Moving;
use moving::Direction;
use snake_game::*;


use std::i32;
use sdl2::pixels::{Color, PixelFormatEnum};
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
use rendering::textures::*;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 800;
//количество полей
const FIELD: u32 = 20;
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

pub fn main() {

    //логирую только в девелопе
    match log4rs::init_file("config/log4rs.yaml", Default::default()) {
        Ok(_) => (),
        Err(_) => ()
    }
    info!("Logger is ready");

    let rng = rand::thread_rng();
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");
    let window: Window = video_subsystem.window("rust-sdl2 demo: Snake", WIDTH, HEIGHT)
        .position_centered().vulkan().build().expect("Failed to create window");
    let mut canvas = window.into_canvas()
        .target_texture().present_vsync().build().expect("Failed to convert window into canvas");

    let grid_left = L_SIZE;
    let grid_right = L_SIZE;
    let grid_top = HEIGHT - (BASE_SIZE * FIELD + L_SIZE);
    let grid_bottom = L_SIZE;
    info!("Grid values:\n\tleft:{}\n\tright:{}\n\ttop:{}\n\tbottom:{}", grid_left, grid_right, grid_top, grid_bottom);
    let creator: TextureCreator<_> = canvas.texture_creator();
    let point_texture = point_texture(&creator,BASE_SIZE,BASE_SIZE);

    let body_texture = body(&creator, Direction::NotMove, BASE_SIZE, BASE_SIZE);
    let left_head = head_with_eyes(&creator, Direction::Left, BASE_SIZE, BASE_SIZE);
    let right_head = head_with_eyes(&creator, Direction::Right, BASE_SIZE, BASE_SIZE);
    let bot_head = head_with_eyes(&creator, Direction::Bot, BASE_SIZE, BASE_SIZE);
    let top_head = head_with_eyes(&creator, Direction::Top, BASE_SIZE, BASE_SIZE);

    let grid = create_texture_rect(&mut canvas, &creator, TextureColor::Black, BASE_SIZE * FIELD).expect("Failed to create a texture");
    let border = create_texture_rect(&mut canvas, &creator, TextureColor::White, BASE_SIZE * FIELD + L_SIZE).expect("Failed to create a texture");

    let mut snake_game = SnakeGame {
        snake: Snake::from_position(vec_deq!((0, 0),(0, 1),(0, 2))),
        point_position: Square::from_position(random_position_in_grid_exclusive(rng, &vec_deq!((0, 0),(0, 1),(0, 2)), FIELD)),
        points: 0,
        is_started: false,
        is_over: false,
        speed: 1,
        speed_controller: 0,
    };
    info!("init point position: {:?}", snake_game.point_position.get_position());

    canvas.set_draw_color(Color::RGB(255, 0, 0));

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
    let mut counter_loop: u16 = 0;
    let mut quit = false;
    let mut last_state = &bot_head;

    'running: loop {
        quit = Snake::is_break(&snake_game.snake);
        handle_events(&mut event_pump, &mut quit, &mut snake_game, rng, &mut canvas);

        if quit {
            snake_game.game_over();
            info!("Game over.\n Your points:{}", snake_game.get_points());
            std::thread::sleep(Duration::from_secs(1));
            break;
        }

        if !snake_game.snake.is_pause() {
            counter_loop += 1;
        }


        if counter_loop >= 480 {
            snake_game.point_position.set_position(random_position_in_grid_exclusive(rng, snake_game.snake.get_position(), FIELD));
            counter_loop = 0;
        }

        if counter_loop % (11_u16 - (snake_game.speed as u16)) == 0 && snake_game.is_started {
            if snake_game.snake.consume_another_cube(&snake_game.point_position) {
                snake_game.point_position.set_position(random_position_in_grid_exclusive(rng, snake_game.snake.get_position(), FIELD));
                snake_game.add_points(snake_game.speed as i32);
                info!("Current points:{}", snake_game.get_points());
                snake_game.snake.grow_up();
                snake_game.speed_up();
                counter_loop = 0;
            }

            snake_game.snake.move_in_direction();
            debug!("current snake position: {:?}", snake_game.snake.get_position());
        }

        // We draw it.
        canvas.clear();
        canvas.copy(&border, None, Rect::new((L_SIZE / 2) as i32, (HEIGHT - BORDER_HEIGHT - L_SIZE / 2) as i32, L_SIZE + BASE_SIZE * FIELD, BORDER_HEIGHT)).unwrap();
        canvas.copy(&grid, None, Rect::new(L_SIZE as i32, (HEIGHT - (L_SIZE + BASE_SIZE * FIELD)) as i32, BASE_SIZE * FIELD, BASE_SIZE * FIELD)).unwrap();
        canvas.copy(&point_texture, None, Rect::new(snake_game.point_position.get_position().0 * (BASE_SIZE as i32) + grid_left as i32, snake_game.point_position.get_position().1 * (BASE_SIZE as i32) + grid_top as i32, BASE_SIZE, BASE_SIZE)).unwrap();
        for i in snake_game.snake.get_position().iter().skip(1) {
            canvas.copy(&body_texture, None, Rect::new(i.0 * (BASE_SIZE as i32) + grid_left as i32, i.1 * (BASE_SIZE as i32) + grid_top as i32, BASE_SIZE, BASE_SIZE)).unwrap();
        }

        let a = snake_game.snake.get_position().iter().take(1).next().unwrap();
        match snake_game.snake.direction() {
            Direction::Left => canvas.copy(&left_head, None, Rect::new(a.0 * (BASE_SIZE as i32) + grid_left as i32, a.1 * (BASE_SIZE as i32) + grid_top as i32, BASE_SIZE, BASE_SIZE)).unwrap(),
            Direction::Top => canvas.copy(&top_head, None, Rect::new(a.0 * (BASE_SIZE as i32) + grid_left as i32, a.1 * (BASE_SIZE as i32) + grid_top as i32, BASE_SIZE, BASE_SIZE)).unwrap(),
            Direction::Bot => canvas.copy(&bot_head, None, Rect::new(a.0 * (BASE_SIZE as i32) + grid_left as i32, a.1 * (BASE_SIZE as i32) + grid_top as i32, BASE_SIZE, BASE_SIZE)).unwrap(),
            Direction::Right => canvas.copy(&right_head, None, Rect::new(a.0 * (BASE_SIZE as i32) + grid_left as i32, a.1 * (BASE_SIZE as i32) + grid_top as i32, BASE_SIZE, BASE_SIZE)).unwrap(),
            Direction::NotMove => canvas.copy(last_state, None, Rect::new(a.0 * (BASE_SIZE as i32) + grid_left as i32, a.1 * (BASE_SIZE as i32) + grid_top as i32, BASE_SIZE, BASE_SIZE)).unwrap(),
        }

        canvas.present();
        //60 FPS
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


fn handle_events(event_pump: &mut EventPump, quit: &mut bool, snake_game: &mut SnakeGame, rng: ThreadRng, canvas: &mut Canvas<Window>) {
    for event in event_pump.poll_iter() {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Space), .. } =>
                {
                    if !snake_game.is_started {
                        snake_game.start();
                    }
                }
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                *quit = true;
                break;
            }
            Event::KeyDown { keycode: Some(Keycode::Up), .. } | Event::KeyDown { keycode: Some(Keycode::W), .. } =>
                if snake_game.is_started && !snake_game.is_over { snake_game.snake.change_direction(Direction::Top) },

            Event::KeyDown { keycode: Some(Keycode::Down), .. } | Event::KeyDown { keycode: Some(Keycode::S), .. } =>
                if snake_game.is_started && !snake_game.is_over { snake_game.snake.change_direction(Direction::Bot) },

            Event::KeyDown { keycode: Some(Keycode::Left), .. } | Event::KeyDown { keycode: Some(Keycode::A), .. } =>
                if snake_game.is_started && !snake_game.is_over { snake_game.snake.change_direction(Direction::Left) },

            Event::KeyDown { keycode: Some(Keycode::Right), .. } | Event::KeyDown { keycode: Some(Keycode::D), .. } =>
                if snake_game.is_started && !snake_game.is_over { snake_game.snake.change_direction(Direction::Right) },

            Event::KeyDown { keycode: Some(Keycode::P), .. } =>
                if snake_game.is_started && !snake_game.is_over { snake_game.snake.pause() },

            Event::MouseButtonDown { mouse_btn: MouseButton::Left, clicks: 1, .. } =>
                canvas.set_draw_color(rand_color(rng)),

            _ => {}
        }
    }
}
