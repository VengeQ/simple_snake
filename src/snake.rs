extern crate rpds;

use crate::moving::Direction;
use crate::moving::Moving;
use rpds::Vector;

#[derive(Debug)]
pub struct Snake {
    head_direction: Direction,
    next_direction: Direction,
    prev_direction: Direction,
    position: rpds::Vector<(i32, i32)>,
}

impl Snake {
    pub fn from_position(position: Vec<(i32, i32)>) -> Self {
        let resized_position: rpds::Vector<(i32, i32)> = position.iter().map(|x| (x.0 * crate::BASE_SIZE as i32, x.1 * crate::BASE_SIZE as i32)).collect();
        Snake {
            head_direction: Direction::NotMove,
            next_direction: Direction::Right,
            position: resized_position,
            prev_direction: Direction::NotMove,
        }
    }

    pub fn get_position(&self) ->&Vector<(i32, i32)>{
        &self.position
    }

    //Если голова змеи пересекает ее тело, то хана
    pub fn is_break(snake: &Snake) -> bool {
        let a = &snake.position;
        match a.iter().find(|current| *current == &a[0]) {
            Some(x) => true,
            None => false
        }
    }

    pub fn set_new_position_if_border(&mut self, max: i32) {
        let (x_position,y_position) = self.position[0];
        match self.next_direction {
            Direction::Top if y_position <= 0 => self.remove_tail_to_head((x_position, max)) ,
            Direction::Bot if y_position >= max =>  self.remove_tail_to_head((x_position,0)),
            Direction::Left if x_position <= 0 => self.remove_tail_to_head((max, y_position)),
            Direction::Right if x_position >= max =>  self.remove_tail_to_head((0, y_position)),
            _ => ()
        }
    }

    fn remove_tail_to_head(&mut self, new_head: (i32, i32)) {
        if self.position.len() == 1 {
            self.position = Vector::new().push_back(new_head);
        } else {
            let without_last = self.position.drop_last().unwrap();
            let mut new_position = Vector::new().push_back(new_head);
            without_last.iter().for_each(|value| {
                new_position.push_back_mut(*value);
            });
            self.position = new_position;
        }
    }
}

//пока плавно не получится сделать, сначала попробую отрисовывать попроще
impl Moving for Snake {
    fn move_in_direction(&mut self) {
        println!("{:?}", &self.position);
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

        let (x_position,y_position) = self.position[0];
        match self.head_direction {
            Direction::Bot => {
                let new_head = (x_position, y_position + crate::BASE_SIZE as i32);
                Snake::remove_tail_to_head(self, new_head);
            }
            Direction::Top => {
                let new_head = (x_position, y_position - crate::BASE_SIZE as i32);
                Snake::remove_tail_to_head(self, new_head);
            }
            Direction::Left => {
                let new_head = (x_position - crate::BASE_SIZE as i32, y_position);
                Snake::remove_tail_to_head(self, new_head);
            }
            Direction::Right => {
                let new_head = (x_position + crate::BASE_SIZE as i32, y_position);
                Snake::remove_tail_to_head(self, new_head);
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