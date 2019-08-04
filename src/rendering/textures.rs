use sdl2::render::{Texture, TextureCreator};
use crate::Direction;
use sdl2::video::WindowContext;
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

fn turn_head(texture: &mut Texture, direction: Direction) {
    let (w, h) = (texture.query().width as usize, texture.query().height as usize);
    let (first_eye, second_eye) = match direction {
        Direction::Top => ((w / 3, h / 3), (2 * w / 3, h / 3)),
        Direction::Bot => ((w / 3, 2 * h / 3), (2 * w / 3, 2 * h / 3)),
        Direction::Left => ((w / 3, h / 3), (w / 3, 2 * h / 3)),
        Direction::Right => ((2 * w / 3, h / 3), (2 * w / 3, 2 * h / 3)),
        Direction::NotMove => ((0, 0), (0, 0)),
    };
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..w {
            for x in 0..h {
                let offset = 3 * y + x * (pitch);
                if x == 0 || x == h - 1 || y == h - 1 || y == 0 ||
                    ((y == first_eye.0 || y == second_eye.0 || y == first_eye.0 + 1 || y == second_eye.0 + 1)
                        && (x == first_eye.1 || x == second_eye.1 || x == first_eye.1 + 1 || x == second_eye.1 + 1)) {
                    buffer[offset] = 0;
                    buffer[offset + 1] = 0;
                    buffer[offset + 2] = 200;
                } else {
                    buffer[offset] = 0;
                    buffer[offset + 1] = 200;
                    buffer[offset + 2] = 0;
                }
            };
        }
    }).unwrap();
}

pub fn head_with_eyes(creator: &TextureCreator<WindowContext>, direction: Direction, width: u32, height: u32) -> Texture {
    let mut head_texture = creator.create_texture_streaming(PixelFormatEnum::RGB24, width, height).unwrap();
    turn_head(&mut head_texture, direction);
    head_texture
}

//Тело - это голова без глаз))
pub fn body(creator: &TextureCreator<WindowContext>, width: u32, height: u32) -> Texture {
    let mut head_texture = creator.create_texture_streaming(PixelFormatEnum::RGB24, width, height).unwrap();
    turn_head(&mut head_texture, Direction::NotMove);
    head_texture
}

pub fn point_texture(creator: &TextureCreator<WindowContext>, width: u32, height: u32) -> Texture {
    let mut point_texture = creator.create_texture_streaming(PixelFormatEnum::RGB24, width, height).unwrap();
    point_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        let (w, h) = (width as usize, height as usize);
        for y in 0..w {
            for x in 0..h {
                let offset = 3 * y + x * pitch;

                buffer[offset] = 0;
                buffer[offset + 1] = ((((h as i32) / 2) * 3 - ((x as i32) - (h as i32) / 2).abs() * 2) * (((h as i32) / 2) * 3 - ((y as i32) - (h as i32) / 2).abs() * 2)) as u8;
                buffer[offset + 2] = 0;
            }
        };
    }).unwrap();
    point_texture
}

fn get_rect_from_text(text: &str, x: i32, y: i32) -> Option<Rect> {
    info!("text {} have length: {}", text, text.len());
    Some(Rect::new(x, y, text.len() as u32 * 14, 32))
}

fn create_texture_from_text<'a>(texture_creator: &'a TextureCreator<WindowContext>,
                                font: &sdl2::ttf::Font,
                                text: &str,
                                r: u8, g: u8, b: u8, ) -> Option<Texture<'a>> {
    if let Ok(surface) = font.render(text).blended(Color::RGB(r, g, b)) {
        texture_creator.create_texture_from_surface(&surface).ok()
    } else {
        None
    }
}

pub fn display_game_information<'a>(snake_game: &crate::SnakeGame, canvas: &mut Canvas<Window>,
                                    texture_creator: &'a TextureCreator<WindowContext>,
                                    font: &sdl2::ttf::Font,
                                    start_x_point: i32, start_y_point: i32) {
    let points_text = format!("Score:{}", snake_game.points);
    let current_speed_text = format!("Speed:{}", snake_game.speed);
    let await_new_game_text = "Click LKM for new game";
    let game_over_text = format!("Game over! Your points: {}", snake_game.points);
    let start_new_game_text = "Press space for start!";

    let points_texture = create_texture_from_text(texture_creator, font, &points_text, 100, 100, 100).expect("Can't create points_texture");
    let current_speed_texture = create_texture_from_text(texture_creator, font, &current_speed_text, 100, 100, 100).expect("Can't create points_texture");
    let await_new_game_texture = create_texture_from_text(texture_creator, font, &await_new_game_text, 100, 100, 100).expect("Can't create points_texture");
    let game_over_texture = create_texture_from_text(texture_creator, font, &game_over_text, 100, 100, 100).expect("Can't create points_texture");
    let start_new_game_texture = create_texture_from_text(texture_creator, font, &start_new_game_text, 100, 100, 100).expect("Can't create points_texture");

    if snake_game.is_over && !snake_game.is_started {
        canvas.copy(&game_over_texture, None, get_rect_from_text(&game_over_text, start_x_point, start_y_point)).expect("Can't render texture");
        canvas.copy(&await_new_game_texture, None, get_rect_from_text(&await_new_game_text, start_x_point, start_y_point + font.height())).expect("Can't render texture");
    } else if !snake_game.is_over && !snake_game.is_started {
        canvas.copy(&start_new_game_texture, None, get_rect_from_text(&start_new_game_text, start_x_point, start_y_point)).expect("Can't render texture");
    } else {
        canvas.copy(&points_texture, None, get_rect_from_text(&points_text, start_x_point, start_y_point)).expect("Can't render texture");
        canvas.copy(&current_speed_texture, None, get_rect_from_text(&current_speed_text, start_x_point, start_y_point + font.height())).expect("Can't render texture");
    }
}

pub fn render_snake<'a>(snake_game: &crate::SnakeGame, canvas: &mut Canvas<Window>, base_size: u32,
                        start_x_point: i32, start_y_point: i32, snake_textures:&'a SnakeTextures<'a>) {



    let base_size_i32 = base_size as i32;
    if snake_game.is_started {
        canvas.copy(&snake_textures.point_texture, None, Rect::new(snake_game.point_position.get_position().0 * base_size_i32 + start_x_point, snake_game.point_position.get_position().1 * base_size_i32 + start_y_point, base_size, base_size)).unwrap();
    }
    for snake_body in snake_game.snake.get_position().iter().skip(1) {
        canvas.copy(&snake_textures.body_texture, None, Rect::new(snake_body.0 * base_size_i32 + start_x_point, snake_body.1 * base_size_i32 + start_y_point, base_size, base_size)).unwrap();
    }
    let snake_head = snake_game.snake.get_position().iter().take(1).next().unwrap();
    match snake_game.snake.direction() {
        Direction::Left => {
            canvas.copy(&snake_textures.left_head, None, Rect::new(snake_head.0 * base_size_i32 + start_x_point, snake_head.1 * base_size_i32 + start_y_point, base_size, base_size)).unwrap();
            //last_state=Some(&snake_textures.left_head)
        }
        Direction::Top => {
            canvas.copy(&snake_textures.top_head, None, Rect::new(snake_head.0 * base_size_i32 + start_x_point, snake_head.1 * base_size_i32 + start_y_point, base_size, base_size)).unwrap();
            //last_state=Some(&snake_textures.top_head)
        }
        Direction::Bot => {
            canvas.copy(&snake_textures.bot_head, None, Rect::new(snake_head.0 * base_size_i32 + start_x_point, snake_head.1 * base_size_i32 + start_y_point, base_size, base_size)).unwrap();
           // last_state=Some(&snake_textures.bot_head)
        }
        Direction::Right => {
            canvas.copy(&snake_textures.right_head, None, Rect::new(snake_head.0 * base_size_i32 + start_x_point, snake_head.1 * base_size_i32 + start_y_point, base_size, base_size)).unwrap();
           // last_state=Some(&snake_textures.right_head)
        }
        Direction::NotMove =>{
            let head_wait = match &snake_game.snake.prev_direction(){
                Direction::Right => &snake_textures.right_head,
                Direction::Left => &snake_textures.left_head,
                Direction::Top => &snake_textures.top_head,
                Direction::Bot => &snake_textures.bot_head,
                _ => &snake_textures.right_head
            };
            canvas.copy(head_wait, None, Rect::new(snake_head.0 * base_size_i32 + start_x_point, snake_head.1 * base_size_i32 + start_y_point, base_size, base_size)).unwrap();
        }
    }
}

pub struct SnakeTextures<'a>{
    body_texture:  Texture<'a>,
    left_head:  Texture<'a>,
    right_head:  Texture<'a>,
    bot_head:  Texture<'a>,
    top_head:  Texture<'a>,
    point_texture:  Texture<'a>,
}

impl<'a> SnakeTextures<'a> {
    pub fn from_base_size(base_size:u32, creator: &'a TextureCreator<WindowContext>) -> Self{
        SnakeTextures {
            body_texture: body(&creator, base_size, base_size),
            left_head: head_with_eyes(&creator, Direction::Left, base_size, base_size),
            right_head:  head_with_eyes(&creator, Direction::Right, base_size, base_size),
            bot_head: head_with_eyes(&creator, Direction::Bot, base_size, base_size),
            top_head: head_with_eyes(&creator, Direction::Top, base_size, base_size),
            point_texture:point_texture(&creator, base_size, base_size),

        }
    }


}