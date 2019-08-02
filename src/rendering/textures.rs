use sdl2::render::{Texture, TextureCreator};
use crate::Direction;
use sdl2::video:: WindowContext;
use sdl2::pixels::PixelFormatEnum;

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
                if x == 0 || x == h - 1 || y == h - 1 || y == 0 || x == 1 || y == 1 || x == h - 2 || y == h - 2 ||
                    ((y == first_eye.0 || y == second_eye.0 || y == first_eye.0+1 || y == second_eye.0+1)
                        && (x == first_eye.1 || x == second_eye.1 || x == first_eye.1+1 || x == second_eye.1+1)) {
                    buffer[offset + 0] = 0;
                    buffer[offset + 1] = 0;
                    buffer[offset + 2] = 200;
                } else {
                    buffer[offset + 0] = 0;
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
pub fn body(creator: &TextureCreator<WindowContext>, direction: Direction, width: u32, height: u32) -> Texture {
    let mut texture = creator.create_texture_streaming(PixelFormatEnum::RGB24, width, height).unwrap();
    let mut head_texture = creator.create_texture_streaming(PixelFormatEnum::RGB24, width, height).unwrap();
    turn_head(&mut head_texture, Direction::NotMove);
    head_texture
}

pub fn point_texture(creator: &TextureCreator<WindowContext>,  width: u32, height: u32) -> Texture{
    let mut point_texture = creator.create_texture_streaming(PixelFormatEnum::RGB24, width, height).unwrap();
    point_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        let (w, h) = (width as usize, height as usize);
        let step = (255 / (w / 2) / (h / 2)) as u8;
        for y in 0..w {
            for x in 0..h {
                let offset = 3 * y + x * pitch;
                if x == 0 || x == h - 1 || y == h - 1 || y == 0 {
                    buffer[offset + 0] = 200;
                    buffer[offset + 1] = 200;
                    buffer[offset + 2] = 0;
                } else {
                    buffer[offset + 0] = 0;
                    buffer[offset + 1] = ((((h as i32) / 2) * 3 - ((x as i32) - (h as i32) / 2).abs() * 2) * (((h as i32) / 2) * 3 - ((y as i32) - (h as i32) / 2).abs() * 2)) as u8;
                    buffer[offset + 2] = 0;
                }
            }
        };
    }).unwrap();
    point_texture
}