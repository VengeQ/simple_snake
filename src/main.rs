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
use rendering::textures::*;


const WIDTH: u32 = 600;
const HEIGHT: u32 = 800;
//количество полей
const FIELD: u32 = 25;
//размер "квадратика
const BASE_SIZE: u32 = 15;
//отступ по краям
const L_SIZE: u32 = (WIDTH - BASE_SIZE * FIELD) / 2;
// расстояние между гридом и граничкой
const BORDER_HEIGHT: u32 = 650;
const FPS: u16 = 120;


pub fn main() {

    //логирую только в девелопе
    if log4rs::init_file("config/log4rs.yaml", Default::default()).is_ok() {};
    info!("Logger is ready");
    let ttf_context = sdl2::ttf::init().expect("Could not load ttf context");
    let font = ttf_context.load_font("assets/amazone.ttf", 32).expect("Couldn't load the font");

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
    let mut snake_game = SnakeGame::with_field(FIELD, rng);
    info!("init point position: {:?}", snake_game.point_position.get_position());
    let creator: TextureCreator<_> = canvas.texture_creator();

    let snake_textures = SnakeTextures::from_base_size(BASE_SIZE, &creator);
    {
        let grid = create_texture_rect(&mut canvas, &creator, TextureColor::Grey, BASE_SIZE * FIELD).expect("Failed to create a texture");
        let border = create_texture_rect(&mut canvas, &creator, TextureColor::White, BASE_SIZE * FIELD + L_SIZE).expect("Failed to create a texture");


        canvas.set_draw_color(Color::RGB(255, 0, 0));

        let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
        let mut counter_loop: u16 = 0;
        let mut quit = false;

        let (start_x_point, start_y_point) = ((L_SIZE / 2) as i32, (HEIGHT - BORDER_HEIGHT - L_SIZE / 2) as i32);
        loop {
            if Snake::is_break(&snake_game.snake) {
                snake_game.game_over();
            }
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


            if counter_loop % FPS == 0 {
                debug!("{:?}", &snake_game);
            }
            if counter_loop >= 60 * 8 {
                info!("counter: {}", counter_loop);
                snake_game.point_position.set_position(random_position_in_grid_exclusive(rng, snake_game.snake.get_position(), FIELD));
                counter_loop = 0;
            }

            if counter_loop % (11_u16 - u16::from(snake_game.speed)) * FPS / 60 == 0 && snake_game.is_started {
                if snake_game.snake.consume_another_cube(&snake_game.point_position) {
                    snake_game.point_position.set_position(random_position_in_grid_exclusive(rng, snake_game.snake.get_position(), FIELD));
                    snake_game.add_points(1); //умножается на скорость
                    info!("Current points:{}", snake_game.get_points());
                    snake_game.snake.grow_up();
                    snake_game.speed_up();
                    info!("Current speed:{}", snake_game.speed);
                    counter_loop = 0;
                }
                snake_game.snake.move_in_direction();
                debug!("current snake position: {:?}", snake_game.snake.get_position());
            }

            // We draw it.
            canvas.clear();
            canvas.copy(&border, None, Rect::new((L_SIZE / 2) as i32, (HEIGHT - BORDER_HEIGHT - L_SIZE / 2) as i32, L_SIZE + BASE_SIZE * FIELD, BORDER_HEIGHT)).unwrap();
            canvas.copy(&grid, None, Rect::new(L_SIZE as i32, (HEIGHT - (L_SIZE + BASE_SIZE * FIELD)) as i32, BASE_SIZE * FIELD, BASE_SIZE * FIELD)).unwrap();

            render_snake(&snake_game, &mut canvas, BASE_SIZE, grid_left as i32, grid_top as i32, &snake_textures);

            display_game_information(&snake_game, &mut canvas, &creator, &font, start_x_point, start_y_point as i32);

            canvas.present();
            //60 FPS
            sleep(Duration::new(0, 1_000_000_000u32 / u32::from(FPS)));
        }
    }
}

fn handle_events(event_pump: &mut EventPump, quit: &mut bool, snake_game: &mut SnakeGame, rng: ThreadRng, canvas: &mut Canvas<Window>) {
    for event in event_pump.poll_iter() {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Space), .. } =>
                {
                    if !snake_game.is_started && !snake_game.is_over {
                        info!("Start new game!");
                        snake_game.start();
                        if snake_game.snake.is_pause() {
                            snake_game.snake.change_direction(Direction::Right);
                            snake_game.snake.unpause();
                        }
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
                {
                    canvas.set_draw_color(rand_color(rng));
                    if snake_game.is_over {
                        snake_game.new_game(FIELD, rng);
                        snake_game.snake.unpause();
                    }
                }

            _ => {}
        }
    }
}
