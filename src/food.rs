use piston_window::*;

use crate::draw::{draw_block};

#[derive(Copy, Clone)]
pub enum FoodType {
    NORMAL,
    PORTAL,
    POISON,
}

#[derive(Copy, Clone)]
pub struct Food {
    pub x: i32,
    pub y: i32,
    pub food_type: FoodType,
}

impl Food {
    pub fn new(x: i32, y: i32, food_type: FoodType) -> Food {
        Food {
            x,
            y,
            food_type,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let color = match self.food_type {
            FoodType::NORMAL => [0.00, 0.80, 0.00, 1.0],
            FoodType::PORTAL => [0.00, 0.00, 0.80, 1.0],
            FoodType::POISON => [0.80, 0.00, 0.00, 1.0],
        };
        draw_block(color, self.x, self.y, con, g);
    }
}