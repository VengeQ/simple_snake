use rand::prelude::ThreadRng;
use sdl2::pixels::Color;
use sdl2::render::{Texture, Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use rand::Rng;
use std::collections::VecDeque;


///Some Color for use
#[derive(Clone, Copy)]
pub enum TextureColor {
    Green,
    Blue,
    Red,
    Black,
    White,
}

///Generate random Color
pub fn rand_color(mut rng: ThreadRng) -> sdl2::pixels::Color {
    let color = Color::RGB(rng.gen(), rng.gen(), rng.gen());
    info!("Generate random color: {:?}", color);
    color
}

///Change Color for mutable texture
pub fn new_color(t: &mut Texture, c: &mut Canvas<Window>, rng: ThreadRng) {
    c.with_texture_canvas(t, |texture| {
        texture.set_draw_color(rand_color(rng));
        texture.clear();
    }).unwrap()
}

///Set random position on grid in view of FIELD and BASE_SIZE
pub fn random_position_in_grid(mut rng: ThreadRng) -> (i32, i32) {
    let x = rng.gen_range(0, crate::FIELD - 1) * crate::BASE_SIZE;
    let y = rng.gen_range(0, crate::FIELD - 1) * crate::BASE_SIZE;
    info!("pos:{:?}", (x as i32, y as i32));
    (x as i32, y as i32)
}

///Create rectangle texture with color and size
pub fn create_texture_rect<'a>(canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, color: TextureColor, size: u32) -> Option<Texture<'a>> {
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

///Set random position on grid in view of FIELD and BASE_SIZE exclude already using fields
// при длинной змейке могут быть проблемы с такой генерацией
pub fn random_position_in_grid_exclusive(mut rng: ThreadRng, ex: &VecDeque<(i32, i32)>, field: u32) -> (i32, i32) {
    let x = rng.gen_range(0, field);
    let y = rng.gen_range(0, field);
    match ex.iter().find(|current| *current == &(x as i32, y as i32)) {
        Some(x) => random_position_in_grid_exclusive(rng, ex, field),
        None => {
            info!("pos:{:?}", (x as i32, y as i32));
            (x as i32, y as i32)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use sdl2::pixels::Color;

    #[test]
    fn random_position_in_grid_exclusive_test() {
        let base_size = 10_u32;
        let field = 2_u32;
        let rng = rand::thread_rng();
        let mut deque = VecDeque::new();
        deque.push_front((0, 1));
        deque.push_front((0, 0));
        deque.push_front((1, 0));
        assert_eq!(crate::random_position_in_grid_exclusive(rng, &deque, field), (1, 1))
    }

    #[test]
    fn rand_color_test_smoke() {
        let rng = rand::thread_rng();
        let mut color_rgb;
        for _ in 0..100 {
            color_rgb = crate::rand_color(rng).rgb();
        }
    }
}