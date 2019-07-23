use crate::moving::Direction;
use crate::moving::Moving;

#[derive(Debug)]
pub struct Snake {
    head_direction: Direction,
    next_direction: Direction,
    pub position: SnakePosition,
    prev_direction: Direction,
}
#[derive(Debug)]
pub struct SnakePosition {
    pub length: i32,
    pub head: (i32, i32),
    pub tail: Vec<(i32, i32)>,
}

impl Snake {
    pub fn from_position(head: (i32, i32), tail: Vec<(i32, i32)>) -> Self {
        let resized_tail: Vec<(i32, i32)> = tail.iter().map(|x| (x.0 * crate::BASE_SIZE as i32, x.1 * crate::BASE_SIZE as i32)).collect();
        let resized_head = (head.0 * crate::BASE_SIZE as i32, head.1 * crate::BASE_SIZE as i32);
        Snake {
            head_direction: Direction::NotMove,
            next_direction: Direction::Right,
            position: SnakePosition { length: 1+tail.len() as i32, head:resized_head, tail: resized_tail },
            prev_direction: Direction::NotMove,
        }
    }

    //Если голова змеи пересекает ее тело, то хана
    pub fn is_break(snake: &Snake) -> bool {
        let a = &snake.position;
        match a.tail.iter().find(|current| *current == &a.head) {
            Some(x) => true,
            None => false
        }
    }

    fn remove_tail_to_head(&mut self, new_head: (i32, i32)) {
        let current_position = &self.position;
        let mut tail = vec!(current_position.head);

        for index in 0..current_position.length - 2 {
            tail.push(current_position.tail[index as usize]);
        }
        self.position = SnakePosition {
            length: current_position.length,
            head: new_head,
            tail: tail,
        }
    }
}

//пока плавно не получится сделать, сначала попробую отрисовывать попроще
impl Moving for Snake {
    fn move_in_direction(&mut self) {
        let step = crate::BASE_SIZE;

            if let Direction::NotMove = self.next_direction { self.head_direction = self.next_direction }
            match self.next_direction {
                Direction::Top | Direction::Bot => self.head_direction = self.next_direction,
                _ => ()
            }

            match self.next_direction {
                Direction::Left | Direction::Right => self.head_direction = self.next_direction,
                _ => ()
            }

        if self.position.length == 1 {
            match self.head_direction {
                Direction::Bot => self.position.head = (self.position.head.0, self.position.head.1 + (crate::BASE_SIZE / 1) as i32),
                Direction::Top => self.position.head = (self.position.head.0, self.position.head.1 - (crate::BASE_SIZE / 1) as i32),
                Direction::Left => self.position.head = (self.position.head.0 - (crate::BASE_SIZE / step) as i32, self.position.head.1),
                Direction::Right => self.position.head = (self.position.head.0 + (crate::BASE_SIZE / step) as i32, self.position.head.1),
                Direction::NotMove => self.head_direction = Direction::NotMove
            }
        } else {
            match self.head_direction {
                Direction::Bot => {
                    let new_head = (self.position.head.0, self.position.head.1 + crate::BASE_SIZE as i32);
                    Snake::remove_tail_to_head( self, new_head);
                }
                Direction::Top => {
                    let new_head = (self.position.head.0, self.position.head.1 - crate::BASE_SIZE as i32);
                    Snake::remove_tail_to_head(self, new_head);
                }
                Direction::Left => {
                    let new_head = (self.position.head.0 - crate::BASE_SIZE as i32, self.position.head.1);
                    Snake::remove_tail_to_head(self, new_head);
                }
                Direction::Right => {
                    let new_head = (self.position.head.0 + crate::BASE_SIZE as i32, self.position.head.1);
                    Snake::remove_tail_to_head(self, new_head);
                }
                Direction::NotMove => ()
            }
        }


    }

    fn set_position(&mut self, position: (i32, i32)) {
        unimplemented!()
    }

    fn change_direction(&mut self, new_direction: Direction) {
        unimplemented!()
    }

    fn pause(&mut self) {
        unimplemented!()
    }
}
/*
impl Moving for Square {
    fn move_in_direction(&mut self) {
        let step = crate::BASE_SIZE;
        if self.position.1 % crate::BASE_SIZE as i32 == 0 && self.position.0 % crate::BASE_SIZE as i32 == 0 {
            if let Direction::NotMove = self.next_direction { self.direction = self.next_direction }
        }
        if self.position.0 % crate::BASE_SIZE as i32 == 0 {
            match self.next_direction {
                Direction::Top | Direction::Bot => self.direction = self.next_direction,
                _ => ()
            }
        }
        if self.position.1 % crate::BASE_SIZE as i32 == 0 {
            match self.next_direction {
                Direction::Left | Direction::Right => self.direction = self.next_direction,
                _ => ()
            }
        }
        //движение, step - прорисовывает каждые step пикселей
        // Так можно регулировать скорость, возможно лучше в `running loop - я пока не понял еще.
        match self.direction {
            Direction::Bot => {
                self.position = (self.position.0, self.position.1 + (crate::BASE_SIZE / step) as i32)
            }
            Direction::Top => {
                self.position = (self.position.0, self.position.1 - (crate::BASE_SIZE / step) as i32)
            }
            Direction::Left => {
                self.position = (self.position.0 - (crate::BASE_SIZE / step) as i32, self.position.1)
            }
            Direction::Right => {
                self.position = (self.position.0 + (crate::BASE_SIZE / step) as i32, self.position.1)
            }
            Direction::NotMove => if self.position.0 % crate::BASE_SIZE as i32 == 0 && self.position.0 % crate::FIELD as i32 == 0 { self.direction = Direction::NotMove }
        };
    }

    //Выставить текущую позицию
    fn set_position(&mut self, new_position: (i32, i32)) {
        self.position = new_position;
    }
    //Установить следующее направление движения
    fn change_direction(&mut self, new_direction: Direction) {
        match new_direction {
            Direction::Right => if self.direction != Direction::Left && self.direction != Direction::NotMove { self.next_direction = new_direction },
            Direction::Left => if self.direction != Direction::Right && self.direction != Direction::NotMove { self.next_direction = new_direction }
            Direction::Bot => if self.direction != Direction::Top && self.direction != Direction::NotMove { self.next_direction = new_direction }
            Direction::Top => if self.direction != Direction::Bot && self.direction != Direction::NotMove { self.next_direction = new_direction }
            Direction::NotMove => { self.next_direction = new_direction }  //??
        }
    }

    fn pause(&mut self) {
        if self.direction == Direction::NotMove {
            self.next_direction = self.prev_direction
        } else {
            self.prev_direction = self.direction;
            self.next_direction = Direction::NotMove;
        }
    }
}

*/