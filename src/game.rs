use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::snake::{Snake, Direction};
use crate::draw::{draw_rectangle};
use crate::food::{Food, FoodType};

const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food: Food,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
    portal: Food,
    portal2: Food,
    poison: Food,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            food_exists: false,
            food: Food::new(6, 4, FoodType::NORMAL),
            portal: Food::new(4, 4, FoodType::PORTAL),
            portal2: Food::new(40, 10, FoodType::PORTAL),
            poison: Food::new(28, 6, FoodType::POISON),
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()),
        };
        if let Some(dir) = dir {
            if dir == self.snake.head_direction().opposite() {
                return;
            }
        }
        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);
        self.portal.draw(con, g);
        self.portal2.draw(con, g);
        self.poison.draw(con, g);


        if self.food_exists {
            self.food.draw(con, g);
            // draw_block(FOOD_COLOR, self.food.x, self.food.y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g); // TOP BORDER
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g); // BOTTOM BORDER
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g); // LEFT BORDER
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g); // RIGHT BORDER

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }
        if !self.food_exists {
            self.add_food();
        }
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && self.food.x == head_x && self.food.y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&mut self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) || (next_x == self.poison.x && next_y == self.poison.y) {
            return false;
        }
        next_x > 0 && next_x < self.width - 1 && next_y > 0 && next_y < self.height - 1 
    }

    fn check_if_portal_enabled(&mut self) -> Food {
        let (head_x, head_y) = self.snake.head_position();
        if head_x == self.portal.x && head_y == self.portal.y {
            return self.portal2;
        }
        if head_x == self.portal2.x && head_y == self.portal2.y {
            return self.portal;
        }
        Food::new(-1, -1, FoodType::NORMAL)
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }
        self.food.x = new_x;
        self.food.y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true
        }
        let is_portal_enabled = self.check_if_portal_enabled();
        if is_portal_enabled.x != -1 && is_portal_enabled.y != -1 {
            self.snake.move_through_portal(&is_portal_enabled);
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.food_exists = true;
        self.game_over = false;
        self.waiting_time = 0.0;
        self.food = Food::new(6, 4, FoodType::NORMAL);
    }

}