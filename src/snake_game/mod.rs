use crate::moving::Moving;
use rand::rngs::ThreadRng;

pub mod snake;
pub mod square;

use square::Square;
use snake::Snake;
use crate::helpers::random_position_in_grid_exclusive;

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

//TODO убрать pub
#[derive(Debug)]
pub struct SnakeGame {
    pub snake: snake::Snake,
    pub point_position: square::Square,
    pub points: i32,
    pub is_started: bool,
    pub is_over: bool,
    pub speed: u8,
    pub  speed_controller: u8,
}


impl SnakeGame {
    pub fn start(&mut self) {
        info!("Start game");
        self.snake.change_direction(crate::Direction::Bot);
        self.is_over = false;
        self.is_started = true;
    }

    pub fn add_points(&mut self, value: i32) {
        self.points += value;
    }

    pub fn get_points(&self) -> i32 {
        self.points
    }

    pub fn game_over(&mut self) {
        self.is_over = true;
        self.is_started = false;
    }

    pub fn speed_up(&mut self) {
        self.speed_controller += 1;
        if self.speed_controller == 10 && self.speed != 10 {
            self.speed += 1;
            self.speed_controller = 0;
        }
    }

    pub fn new_game(& mut self,field:u32, rng:ThreadRng){
            self.snake = Snake::from_position(vec_deq!((0, 0)));
            self.point_position = Square::from_position(random_position_in_grid_exclusive(rng, &vec_deq!((0, 0)), field));
            self.points = 0;
            self.is_started= false;
            self.is_over = false;
            self.speed = 4;
            self.speed_controller = 0;

            info!("new game with: {:?}", self)
    }

    pub fn with_field(field:u32, rng:ThreadRng) -> Self{
       SnakeGame {
            snake: Snake::from_position(vec_deq!((0, 0))),
            point_position: Square::from_position(random_position_in_grid_exclusive(rng, &vec_deq!((0, 0)), field)),
            points: 0,
            is_started: false,
            is_over: false,
            speed: 4,
            speed_controller: 0,
        }
    }
}