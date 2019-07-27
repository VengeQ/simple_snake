extern crate rpds;

use crate::moving::Direction;
use crate::moving::Moving;
use rpds::Vector;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Snake {
    head_direction: Direction,
    next_direction: Direction,
    prev_direction: Direction,
    position: VecDeque<(i32,i32)>,
}

impl Snake {
    pub fn from_position(position: VecDeque<(i32, i32)>) -> Self {
        let resized_position: VecDeque<(i32, i32)> = position.iter().map(|x| (x.0 * crate::BASE_SIZE as i32, x.1 * crate::BASE_SIZE as i32)).collect();
        Snake {
            head_direction: Direction::NotMove,
            next_direction: Direction::Right,
            position: resized_position,
            prev_direction: Direction::NotMove,
        }
    }

    pub fn get_position(&self) ->&VecDeque<(i32, i32)>{
        &self.position
    }

    //Если голова змеи пересекает ее тело, то хана
    pub fn is_break(snake: &Snake) -> bool {
        let a = &snake.position;
        match a.iter().find(|current| *current == &a[0]) {
            Some(_) => true,
            None => false
        }
    }

    pub fn set_new_position_if_border(&mut self, max: i32) {
        let (x_position,y_position) = self.position[0];
        match self.next_direction {
            Direction::Top if y_position <= 0 => self.shift_tail_to_head((x_position, max)) ,
            Direction::Bot if y_position >= max =>  self.shift_tail_to_head((x_position, 0)),
            Direction::Left if x_position <= 0 => self.shift_tail_to_head((max, y_position)),
            Direction::Right if x_position >= max =>  self.shift_tail_to_head((0, y_position)),
            _ => ()
        }
    }

    fn shift_tail_to_head(&mut self, new_head: (i32, i32)) {
        self.position.pop_back().unwrap();
        self.position.push_front(new_head);

    }
}

//пока плавно не получится сделать, сначала попробую отрисовывать попроще
impl Moving for Snake {
    fn move_in_direction(&mut self) {
        if let Direction::NotMove = self.next_direction { self.head_direction = self.next_direction }
        match self.next_direction {
            Direction::Top | Direction::Bot => self.head_direction = self.next_direction,
            _ => ()
        }

        match self.next_direction {
            Direction::Left | Direction::Right => self.head_direction = self.next_direction,
            _ => ()
        }

        let (x_position,y_position) = self.position[0];
        match self.head_direction {
            Direction::Bot => {
                let new_head = (x_position, y_position + crate::BASE_SIZE as i32);
                Snake::shift_tail_to_head(self, new_head);
            }
            Direction::Top => {
                let new_head = (x_position, y_position - crate::BASE_SIZE as i32);
                Snake::shift_tail_to_head(self, new_head);
            }
            Direction::Left => {
                let new_head = (x_position - crate::BASE_SIZE as i32, y_position);
                Snake::shift_tail_to_head(self, new_head);
            }
            Direction::Right => {
                let new_head = (x_position + crate::BASE_SIZE as i32, y_position);
                Snake::shift_tail_to_head(self, new_head);
            }
            Direction::NotMove => ()
        }
    }

    fn set_position(&mut self, position: (i32, i32)) {
        unimplemented!()
    }

    fn change_direction(&mut self, new_direction: Direction) {
        match new_direction {
            Direction::Right => if self.head_direction != Direction::Left && self.head_direction != Direction::NotMove { self.next_direction = new_direction },
            Direction::Left => if self.head_direction != Direction::Right && self.head_direction != Direction::NotMove { self.next_direction = new_direction }
            Direction::Bot => if self.head_direction != Direction::Top && self.head_direction != Direction::NotMove { self.next_direction = new_direction }
            Direction::Top => if self.head_direction != Direction::Bot && self.head_direction != Direction::NotMove { self.next_direction = new_direction }
            Direction::NotMove => { self.next_direction = new_direction }  //??
        }
    }

    fn pause(&mut self) {
        unimplemented!()
    }
}