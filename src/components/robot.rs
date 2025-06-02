use bevy::prelude::*;
use crate::structs::controls::Direction;

#[derive(Component)]
pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub start_x: i32,
    pub start_y: i32,
    pub start_direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self {
            x,
            y,
            direction,
            start_x: x,
            start_y: y,
            start_direction: direction,
        }
    }

    pub fn reset_to_start(&mut self) {
        self.x = self.start_x;
        self.y = self.start_y;
        self.direction = self.start_direction;
    }

    pub fn move_forward(&mut self) {
        let (dx, dy) = self.direction.get_offset();
        self.x += dx;
        self.y += dy;
    }

    pub fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }
}