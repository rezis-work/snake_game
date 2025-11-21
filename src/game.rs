use piston_window::*;
use piston_window::types::Color;

use rand::{rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_rectangle, draw_apple, draw_block};
use crate::menu::GameMode;
use crate::enemy::Enemy;

const BORDER_COLOR: Color = [0.2, 0.3, 0.4, 1.0];
const GAMEOVER_COLOR: Color = [0.9, 0.1, 0.0, 0.5];

const RESTART_TIME: f64 = 3.0; // Give player time to see game over screen
const FOOD_PER_LEVEL: i32 = 5; // Number of foods needed to level up

pub struct Game {
    snake: Snake,

    food_exist: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
    
    score: i32,
    level: i32,
    foods_eaten: i32,
    
    game_mode: GameMode,
    final_score: i32,
    final_level: i32,
    
    // Timer mode
    game_time: f64,
    time_limit: f64,
    
    // High score tracking
    high_score: i32,
    
    // Enemy (appears from level 3)
    enemy: Option<Enemy>,
    enemy_move_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32, game_mode: GameMode) -> Game {
        let time_limit = game_mode.get_time_limit();
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exist: false,
            food_x: 0,
            food_y: 0,
            width,
            height,
            game_over: false,
            score: 0,
            level: 1,
            foods_eaten: 0,
            game_mode,
            final_score: 0,
            final_level: 1,
            game_time: 0.0,
            time_limit,
            high_score: 0,
            enemy: None,
            enemy_move_time: 0.0,
        }
    }
    
    pub fn restart_game(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exist = false;
        self.food_x = 0;
        self.food_y = 0;
        self.game_over = false;
        self.score = 0;
        self.level = 1;
        self.foods_eaten = 0;
        self.final_score = 0;
        self.final_level = 1;
        self.enemy = None;
        self.enemy_move_time = 0.0;
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
            _ => None,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exist {
            draw_apple(self.food_x, self.food_y, con, g);
        }
        
        // Draw enemy if it exists (level 3+)
        if let Some(ref enemy) = self.enemy {
            enemy.draw(con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);

        // Display score visually using colored blocks (each block = 10 points)
        let score_color: Color = [1.0, 1.0, 0.0, 1.0];
        let score_blocks = (self.score / 10).min(10); // Show up to 10 blocks
        for i in 0..score_blocks {
            draw_block(score_color, self.width + 1 + i, 0, con, g);
        }

        // Display level visually using colored blocks
        let level_color: Color = [0.0, 0.5, 1.0, 1.0];
        let level_blocks = self.level.min(10); // Show up to 10 blocks
        for i in 0..level_blocks {
            draw_block(level_color, self.width + 1 + i, 1, con, g);
        }
        
        // Display current game mode indicator
        let mode_color: Color = [0.5, 0.0, 1.0, 1.0]; // Purple
        let mode_indicator = match self.game_mode {
            crate::menu::GameMode::Easy => 1,
            crate::menu::GameMode::Medium => 2,
            crate::menu::GameMode::Hard => 3,
            crate::menu::GameMode::Timer => 4,
            crate::menu::GameMode::Survival => 5,
        };
        for i in 0..mode_indicator.min(10) {
            draw_block(mode_color, self.width + 1 + i, 2, con, g);
        }
        
        // Display timer for timer mode
        if self.game_mode.is_timed_mode() {
            let remaining_time = (self.time_limit - self.game_time).max(0.0) as i32;
            let timer_blocks = (remaining_time / 5).min(10);
            let timer_color: Color = if remaining_time < 10 {
                [1.0, 0.3, 0.0, 1.0] // Red when low
            } else {
                [0.0, 1.0, 1.0, 1.0] // Cyan
            };
            for i in 0..timer_blocks {
                draw_block(timer_color, self.width + 1 + i, 3, con, g);
            }
        }

        if self.game_over {
            self.draw_game_over(con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            // Game over screen is handled in draw, no auto-restart
            return;
        }
        
        // Update game time for timer mode
        if self.game_mode.is_timed_mode() {
            self.game_time += delta_time;
            if self.game_time >= self.time_limit {
                self.game_over = true;
                self.final_score = self.score;
                self.final_level = self.level;
                if self.score > self.high_score {
                    self.high_score = self.score;
                }
                return;
            }
        }

        if !self.food_exist {
            self.add_food();
        }
        
        // Spawn enemy at level 3
        if self.level >= 3 && self.enemy.is_none() {
            self.spawn_enemy();
        }
        
        // Update enemy movement
        if let Some(ref mut enemy) = self.enemy {
            self.enemy_move_time += delta_time;
            if self.enemy_move_time > 0.3 { // Enemy moves every 0.3 seconds
                enemy.update(self.width, self.height);
                self.enemy_move_time = 0.0;
            }
        }

        // Calculate moving period based on level and game mode
        let base_speed = self.game_mode.get_base_speed();
        let speed_multiplier = self.game_mode.get_speed_multiplier();
        let moving_period = base_speed / (1.0 + (self.level as f64) * speed_multiplier);
        if self.waiting_time > moving_period {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exist && self.food_x == head_x && self.food_y == head_y {
            self.food_exist = false;
            self.snake.restore_tail();
            self.score += 10;
            self.foods_eaten += 1;
            
            // Level up every FOOD_PER_LEVEL foods eaten
            // Level 1: 0-4 foods, Level 2: 5-9 foods, Level 3: 10-14 foods, etc.
            let new_level = (self.foods_eaten / FOOD_PER_LEVEL) + 1;
            if new_level > self.level {
                self.level = new_level;
            }
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y): (i32, i32) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }
        
        // Check collision with enemy
        if let Some(ref enemy) = self.enemy {
            if enemy.check_collision(next_x, next_y) {
                return false;
            }
        }
        
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }
    
    fn spawn_enemy(&mut self) {
        use rand::{rng, Rng};
        let mut rng = rng();
        // Spawn enemy away from snake and food
        let (snake_x, snake_y) = self.snake.head_position();
        let mut enemy_x = rng.random_range(1..self.width - 1);
        let mut enemy_y = rng.random_range(1..self.height - 1);
        
        // Make sure enemy doesn't spawn on snake or food
        let mut attempts = 0;
        while (enemy_x == snake_x && enemy_y == snake_y) || 
              (enemy_x == self.food_x && enemy_y == self.food_y) ||
              self.snake.overlap_tail(enemy_x, enemy_y) {
            enemy_x = rng.random_range(1..self.width - 1);
            enemy_y = rng.random_range(1..self.height - 1);
            attempts += 1;
            if attempts > 50 {
                // Fallback: spawn at a safe corner
                enemy_x = self.width - 3;
                enemy_y = self.height - 3;
                break;
            }
        }
        
        self.enemy = Some(Enemy::new(enemy_x, enemy_y));
    }

    fn add_food(&mut self) {
        let mut rng = rng();
        let mut new_x = rng.random_range(1..self.width - 1);
        let mut new_y = rng.random_range(1..self.height - 1);
        let mut attempts = 0;
        while self.snake.overlap_tail(new_x, new_y) || 
              (self.enemy.is_some() && self.enemy.as_ref().unwrap().check_collision(new_x, new_y)) {
            new_x = rng.random_range(1..self.width - 1);
            new_y = rng.random_range(1..self.height - 1);
            attempts += 1;
            if attempts > 100 {
                break; // Prevent infinite loop
            }
        }
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exist = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
            self.final_score = self.score;
            self.final_level = self.level;
            // Update high score for survival mode
            if matches!(self.game_mode, crate::menu::GameMode::Survival) && self.score > self.high_score {
                self.high_score = self.score;
            }
            self.waiting_time = 0.0; // Reset timer for game over screen
        }
        self.waiting_time = 0.0;
    }

    pub fn should_return_to_menu(&self) -> bool {
        self.game_over && self.waiting_time > RESTART_TIME
    }
    
    pub fn get_score(&self) -> i32 {
        self.score
    }
    
    pub fn get_level(&self) -> i32 {
        self.level
    }
    
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
    
    fn draw_game_over(&self, con: &Context, g: &mut G2d) {
        // Draw semi-transparent overlay
        draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        
        let center_y = self.height / 2;
        
        // Draw simple "GAME OVER" indicator - just a big red X pattern
        let go_color: Color = [1.0, 0.0, 0.0, 1.0];
        let go_size = 5;
        let go_x = self.width / 2 - go_size / 2;
        let go_y = center_y - 3;
        
        // Draw X pattern
        for i in 0..go_size {
            draw_block(go_color, go_x + i, go_y + i, con, g);
            draw_block(go_color, go_x + go_size - 1 - i, go_y + i, con, g);
        }
        
        // Draw final score with label indicator
        let score_y = center_y + 2;
        let score_label_color: Color = [1.0, 1.0, 0.0, 1.0];
        // Draw "S" indicator for Score
        for i in 0..3 {
            draw_block(score_label_color, 2, score_y + i, con, g);
        }
        draw_block(score_label_color, 3, score_y, con, g);
        draw_block(score_label_color, 3, score_y + 2, con, g);
        
        // Draw score blocks
        let score_color: Color = [1.0, 1.0, 0.0, 1.0];
        let score_blocks_count = (self.final_score / 10).min(12);
        for i in 0..score_blocks_count {
            draw_block(score_color, 5 + i, score_y + 1, con, g);
        }
        
        // Draw final level with label indicator
        let level_y = center_y + 5;
        let level_label_color: Color = [0.0, 0.5, 1.0, 1.0];
        // Draw "L" indicator for Level
        for i in 0..4 {
            draw_block(level_label_color, 2, level_y + i, con, g);
        }
        draw_block(level_label_color, 3, level_y + 3, con, g);
        draw_block(level_label_color, 4, level_y + 3, con, g);
        
        // Draw level blocks
        let level_color: Color = [0.0, 0.5, 1.0, 1.0];
        let level_blocks_count = self.final_level.min(12);
        for i in 0..level_blocks_count {
            draw_block(level_color, 5 + i, level_y + 1, con, g);
        }
        
        // Show high score for survival mode - simple indicator
        if matches!(self.game_mode, crate::menu::GameMode::Survival) && self.high_score > 0 {
            let hs_y = level_y + 4;
            let hs_color: Color = [1.0, 0.8, 0.0, 1.0];
            // Draw "H" indicator for High Score
            draw_block(hs_color, 2, hs_y, con, g);
            draw_block(hs_color, 4, hs_y, con, g);
            draw_block(hs_color, 2, hs_y + 1, con, g);
            draw_block(hs_color, 4, hs_y + 1, con, g);
            draw_block(hs_color, 2, hs_y + 2, con, g);
            draw_block(hs_color, 3, hs_y + 2, con, g);
            draw_block(hs_color, 4, hs_y + 2, con, g);
            draw_block(hs_color, 2, hs_y + 3, con, g);
            draw_block(hs_color, 4, hs_y + 3, con, g);
            draw_block(hs_color, 2, hs_y + 4, con, g);
            draw_block(hs_color, 4, hs_y + 4, con, g);
            
            let hs_blocks = (self.high_score / 10).min(12);
            for i in 0..hs_blocks {
                draw_block(hs_color, 5 + i, hs_y + 2, con, g);
            }
        }
    }
}