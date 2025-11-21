use piston_window::{Context, G2d, ellipse};
use piston_window::types::Color;
use crate::snake::Direction;
use crate::draw::{draw_circle, to_coord};
use rand::{rng, Rng};

const ENEMY_COLOR: Color = [0.8, 0.0, 0.0, 1.0]; // Red enemy
const ENEMY_EYE_COLOR: Color = [1.0, 1.0, 1.0, 1.0]; // White eyes

pub struct Enemy {
    x: i32,
    y: i32,
    direction: Direction,
    move_counter: i32,
    change_direction_counter: i32,
}

impl Enemy {
    pub fn new(x: i32, y: i32) -> Enemy {
        Enemy {
            x,
            y,
            direction: Direction::Right,
            move_counter: 0,
            change_direction_counter: 0,
        }
    }
    
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Draw enemy as a red circle with white eyes
        draw_circle(ENEMY_COLOR, self.x, self.y, con, g);
        
        // Draw eyes
        let gui_x = to_coord(self.x);
        let gui_y = to_coord(self.y);
        ellipse(
            ENEMY_EYE_COLOR,
            [gui_x + 6.0, gui_y + 6.0, 4.0, 4.0],
            con.transform,
            g,
        );
        ellipse(
            ENEMY_EYE_COLOR,
            [gui_x + 15.0, gui_y + 6.0, 4.0, 4.0],
            con.transform,
            g,
        );
    }
    
    pub fn update(&mut self, width: i32, height: i32) {
        self.change_direction_counter += 1;
        
        // Change direction randomly every 3-5 moves
        if self.change_direction_counter >= 3 {
            let mut rng = rng();
            if rng.random_range(0..100) < 30 { // 30% chance to change direction
                let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
                self.direction = directions[rng.random_range(0..directions.len())];
            }
            self.change_direction_counter = 0;
        }
        
        // Move enemy
        self.move_counter += 1;
        if self.move_counter >= 2 { // Enemy moves slower than snake
            self.move_counter = 0;
            
            match self.direction {
                Direction::Up => {
                    if self.y > 1 {
                        self.y -= 1;
                    } else {
                        self.direction = Direction::Down;
                    }
                }
                Direction::Down => {
                    if self.y < height - 2 {
                        self.y += 1;
                    } else {
                        self.direction = Direction::Up;
                    }
                }
                Direction::Left => {
                    if self.x > 1 {
                        self.x -= 1;
                    } else {
                        self.direction = Direction::Right;
                    }
                }
                Direction::Right => {
                    if self.x < width - 2 {
                        self.x += 1;
                    } else {
                        self.direction = Direction::Left;
                    }
                }
            }
        }
    }
    
    pub fn check_collision(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y == y
    }
}

