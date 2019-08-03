use crate::moving::Direction;
use crate::moving::Moving;

use std::collections::vec_deque::VecDeque;


#[derive(Debug)]
pub struct Snake {
    curr_direction: Direction,
    next_direction: Direction,
    prev_direction: Direction,
    position: VecDeque<(i32, i32)>,
}

impl Snake {
    pub fn from_position(position: VecDeque<(i32, i32)>) -> Self {
        let resized_position: VecDeque<(i32, i32)> = position.iter().map(|x| (x.0 * crate::BASE_SIZE as i32, x.1 * crate::BASE_SIZE as i32)).collect();
        Snake {
            curr_direction: Direction::NotMove,
            next_direction: Direction::Right,
            position: position,
            prev_direction: Direction::NotMove,
        }
    }

    pub fn get_position(&self) -> &VecDeque<(i32, i32)> {
        &self.position
    }

    //Если голова змеи пересекает ее тело, то хана
    pub fn is_break(snake: &Snake) -> bool {
        let head = &snake.position.get(0).unwrap();
        match snake.get_position().iter().skip(1).find(|value| value == head) {
            Some(t) => true,
            None => false
        }
    }

    pub fn is_border(&mut self, max: i32) -> bool {
        let (x_position, y_position) = self.position[0];
        match self.next_direction {
            Direction::Top if y_position == 0 => true,
            Direction::Bot if y_position == max => true,
            Direction::Left if x_position == 0 => true,
            Direction::Right if x_position == max => true,
            _ => false
        }
    }

    pub fn set_new_position_if_border(&mut self, max: i32) {
        let (x_position, y_position) = self.position[0];

        match self.next_direction {
            Direction::Top => self.shift_tail_to_head((x_position, max - 1)),
            Direction::Bot => self.shift_tail_to_head((x_position, 0)),
            Direction::Left => self.shift_tail_to_head((max - 1, y_position)),
            Direction::Right => self.shift_tail_to_head((0, y_position)),
            _ => ()
        }
    }

    fn shift_tail_to_head(&mut self, new_head: (i32, i32)) {
        debug!("new head {:?}", &new_head);
        self.position.pop_back().unwrap();
        self.position.push_front(new_head);
    }


    ///Do something when consume another square. May be this should be in Trait Moving
    pub fn consume_another_cube(&self, another: &crate::Square) -> bool {
        self.position[0] == another.get_position()
    }

    pub fn direction(&self) -> Direction{
        self.curr_direction
    }

    pub fn grow_up(&mut self) {
        self.position.push_back(*(self.position.get(self.position.len() - 1).unwrap()));
    }

    pub fn is_pause(&mut self) -> bool{
        self.curr_direction == Direction::NotMove
    }
}

//пока плавно не получится сделать, сначала попробую отрисовывать попроще
impl Moving for Snake {
    fn move_in_direction(&mut self) {
        if let Direction::NotMove = self.next_direction { self.curr_direction = self.next_direction }
        match self.next_direction {
            Direction::Top | Direction::Bot => self.curr_direction = self.next_direction,
            _ => ()
        }

        match self.next_direction {
            Direction::Left | Direction::Right => self.curr_direction = self.next_direction,
            _ => ()
        }
        let is_border_neg = !self.is_border((crate::FIELD - 1) as i32);
        if is_border_neg {
            let (x_position, y_position) = self.position[0];
            match self.curr_direction {
                Direction::Bot => {
                    Snake::shift_tail_to_head(self, (x_position, y_position + 1));
                }
                Direction::Top => {
                    Snake::shift_tail_to_head(self, (x_position, y_position - 1))
                }
                Direction::Left => {
                    Snake::shift_tail_to_head(self, (x_position - 1, y_position))
                }
                Direction::Right => {
                    Snake::shift_tail_to_head(self, (x_position + 1, y_position))
                }
                Direction::NotMove => ()
            }
        } else {
            debug!("new_border");
            self.set_new_position_if_border(crate::FIELD as i32);
        }
    }

    fn set_position(&mut self, position: (i32, i32)) {
        unimplemented!()
    }

    fn change_direction(&mut self, new_direction: Direction) {
        match new_direction {
            Direction::Right => if self.curr_direction != Direction::Left && self.curr_direction != Direction::NotMove { self.next_direction = new_direction },
            Direction::Left => if self.curr_direction != Direction::Right && self.curr_direction != Direction::NotMove { self.next_direction = new_direction }
            Direction::Bot => if self.curr_direction != Direction::Top && self.curr_direction != Direction::NotMove { self.next_direction = new_direction }
            Direction::Top => if self.curr_direction != Direction::Bot && self.curr_direction != Direction::NotMove { self.next_direction = new_direction }
            Direction::NotMove => { self.next_direction = new_direction }  //??
        }
    }

    fn pause(&mut self) {
        if self.curr_direction == Direction::NotMove {
            self.next_direction = self.prev_direction
        } else {
            self.prev_direction = self.curr_direction;
            self.next_direction = Direction::NotMove;
        }
    }

    fn unpause(&mut self) {
        self.curr_direction = self.next_direction
    }


}

#[cfg(test)]
mod tests {
    use super::Snake;
    use super::Direction;
    use std::collections::VecDeque;

    #[test]
    fn impl_snake_test() {
        let deque: VecDeque<(i32, i32)> = (vec![(0, 0), (0, 1)].iter().map(|x| *x)).collect();
        let test_snake = Snake::from_position(deque.clone());

        assert_eq!(test_snake.position, deque);
    }

    #[test]
    fn impl_moving_for_snake_test() {}
}