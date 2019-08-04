

use crate::moving::Direction;
use crate::moving::Moving;

///simple square
#[derive(Debug)]
pub struct Square {
    direction: Direction,
    next_direction: Direction,
    position: (i32, i32),
    prev_direction: Direction,
}

impl Square {
    ///Create new square with custom direction in top left corner
    #[allow(dead_code)]
    pub fn new(direction: Direction) -> Self {
        Square {
            direction,
            next_direction: direction,
            position: (0, 0),
            prev_direction: Direction::NotMove,
        }
    }
    #[allow(dead_code)]
    pub fn set_new_position_if_border(&mut self, max: i32) {
        match self.direction {
            Direction::Top if self.position.1 <= 0 => self.position = (self.position.0, max),
            Direction::Bot if self.position.1 >= max => self.position = (self.position.0, 0),
            Direction::Left if self.position.0 <= 0 => self.position = (max, self.position.1),
            Direction::Right if self.position.0 >= max => self.position = (0, self.position.1),
            _ => ()
        }
    }
    ///Create not moving square with random position on grid
    pub fn from_position(position: (i32, i32)) -> Self {
        Square {
            direction: Direction::NotMove,
            next_direction: Direction::NotMove,
            position,
            prev_direction: Direction::NotMove,
        }
    }

    ///return postion of square as tuple(horizontal, vertical)
    pub fn get_position(&self) -> ((i32, i32)) {
        self.position
    }


    ///Do something when consume another square. May be this should be in Trait Moving
    #[allow(dead_code)]
    pub fn consume_another_cube(&self, another: &Square) -> bool {
        self.position == another.position
    }
}

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
            Direction::NotMove => if self.position.0 % crate::BASE_SIZE as i32 == 0 && self.position.1 % crate::BASE_SIZE as i32 == 0 { self.direction = Direction::NotMove }
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
    fn unpause(&mut self){unimplemented!()}
}

