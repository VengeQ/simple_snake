extern crate libc;
extern crate sdl2_sys;
extern crate gfx;
extern crate gfx_window_sdl;
extern crate sdl2;

use gfx::Device;


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


#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
    Red,
    Black,
    White,
}

const BASE_SIZE: u32 = 20;
const L_SIZE: u32 = (600 - BASE_SIZE * 20) / 2;
const BORDER_HEIGTH: u32 = 800 - BASE_SIZE * 20 + L_SIZE + 150;

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
    let rng = rand::thread_rng();
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");
    let window = video_subsystem.window("rust-sdl2 demo: Snake", 600, 800)
        .position_centered()
        .vulkan()
        .build()
        .expect("Failed to create window");


    let grid_left = L_SIZE;
    let grid_right = L_SIZE;
    let grid_top = 800 - (BASE_SIZE * 20 + L_SIZE);
    let grid_bottom = L_SIZE;

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Failed to convert window into canvas");

    let creator: TextureCreator<_> = canvas.texture_creator();
    let grid = create_texture_rect(&mut canvas, &creator, TextureColor::Black, BASE_SIZE * 20).expect("Failed to create a texture");
    let border = create_texture_rect(&mut canvas, &creator, TextureColor::White, BASE_SIZE * 20 + L_SIZE).expect("Failed to create a texture");
    let mut t = create_texture_rect(&mut canvas, &creator, TextureColor::Green, BASE_SIZE).expect("Failed to create a texture");

    let mut cube = Cube::new(Direction::Bot);
    let mut snake = create_texture_rect(&mut canvas, &creator, TextureColor::Green, BASE_SIZE).expect("Failed to create a texture");

    canvas.set_draw_color(Color::RGB(255, 0, 0));

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    let mut counter = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    {
                        break 'running;
                    }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    cube.set_direction(Direction::Top)
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    cube.set_direction(Direction::Bot)
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    cube.set_direction(Direction::Left)
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    cube.set_direction(Direction::Right)
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, clicks: 1, .. } =>
                    {
                        new_color(&mut t, &mut canvas, rng);
                        canvas.set_draw_color(rand_color(rng));
                    }
                _ => {}
            }
        }
        // We set fulfill our window with red.
        let cur_width = canvas.window().size().0 as i32;
        let cur_height = canvas.window().size().1 as i32;


        // We draw it.
        canvas.clear();

        counter += 1;

        if counter >= 5 {
            counter = 0;
            println!("{:?}",cube.position);
            cube.move_in_direction(cube.direction.clone());
            Cube::set_new_position_if_border(&mut cube, 20 * (BASE_SIZE - 1) as i32);
        }

        canvas.copy(&border, None, Rect::new((L_SIZE / 2) as i32, (800 - BORDER_HEIGTH - L_SIZE / 2) as i32, L_SIZE + BASE_SIZE * 20, BORDER_HEIGTH)).unwrap();

        canvas.copy(&grid, None, Rect::new(L_SIZE as i32, (800 - (L_SIZE + BASE_SIZE * 20)) as i32, BASE_SIZE * 20, BASE_SIZE * 20)).unwrap();
        canvas.copy(&snake, None, Rect::new(cube.position.0 + grid_left as i32, cube.position.1 + grid_top as i32, BASE_SIZE, BASE_SIZE)).unwrap();
        canvas.present();
        //60 FPS
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


fn rand_color(mut rng: ThreadRng) -> sdl2::pixels::Color {
    Color::RGB(rng.gen(), rng.gen(), rng.gen())
}

fn new_color_for_texture(te: &mut Canvas<Window>, rng: ThreadRng) {
    te.set_draw_color(rand_color(rng));
    te.clear();
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

trait Moving {
    fn move_in_direction(&mut self, direction: Direction);
    fn set_position(&mut self, position: (i32, i32));
    fn set_direction(&mut self, new_direction: Direction);
}


struct Cube {
    direction: Direction,
    position: (i32, i32),
}

impl Cube {
    fn new(direction: Direction) -> Self {
        Cube {
            direction,
            position: (0, 0),
        }
    }
    fn set_new_position_if_border(&mut self, max: i32) -> () {
        match &self.direction {
            &Direction::Top if self.position.1 < 0 => self.position = (self.position.0, max),
            &Direction::Bot if self.position.1 > max => self.position = (self.position.0, 0),
            &Direction::Left if self.position.0 < 0 => self.position = (max, self.position.1),
            &Direction::Bot if self.position.0> max => self.position = (0, self.position.1),
            _ => ()
        }
    }

}

impl Moving for Cube {
    fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Bot => self.position = (self.position.0, self.position.1 + BASE_SIZE as i32),
            Direction::Top => self.position = (self.position.0, self.position.1 - BASE_SIZE as i32),
            Direction::Left => self.position = (self.position.0 - BASE_SIZE as i32, self.position.1 ),
            Direction::Right => self.position = (self.position.0+ BASE_SIZE as i32, self.position.1),
            _ => {}
        };
    }
    fn set_position(&mut self, new_position: (i32, i32)) {
        self.position = new_position;
    }
    fn set_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }
}

#[derive(Clone)]
enum Direction {
    Bot,
    Top,
    Left,
    Right,
    NotMove
}

trait Hello{

}