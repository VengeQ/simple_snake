use crate::moving::Moving;

pub mod snake;
pub mod square;

//TODO убрать pub
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
}