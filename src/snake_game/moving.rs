///Direction of moving
#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Direction {
    Bot,
    Top,
    Left,
    Right,
    NotMove,
}

pub trait Moving {
    ///Move for current direction
    fn move_in_direction(&mut self);
    ///Set new position on grid
    fn set_position(&mut self, position: (i32, i32));
    fn change_direction(&mut self, new_direction: Direction);
    fn pause(&mut self);
    fn unpause(&mut self);
}