use crate::moving::Direction;
use crate::moving::Moving;

pub struct Snake {
    direction: Direction,
    next_direction: Direction,
    pub position:SnakePosition,
    prev_direction: Direction,
}

pub struct SnakePosition{
    pub head:(i32,i32),
    pub tail:Vec<(i32,i32)>,
}

impl Snake{
    pub fn from_position(head:(i32,i32), tail:Vec<(i32,i32)>)-> Self{
        let resized_tail :Vec<(i32,i32)> = tail.iter().map(|x|(x.0*crate::BASE_SIZE as i32, x.1*crate::BASE_SIZE as i32)).collect();
        Snake{
            direction: Direction::NotMove,
            next_direction: Direction::NotMove,
            position: SnakePosition {head,tail:resized_tail},
            prev_direction: Direction::NotMove
        }
    }
}