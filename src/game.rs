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
    snake1: Snake,  // Player 1 (Arrow keys)
    snake2: Snake,  // Player 2 (WASD keys)

    food_exist: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
    
    score1: i32,  // Player 1 score
    score2: i32,  // Player 2 score
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
    
    // Enemies (appears from level 3, more as levels increase)
    enemies: Vec<Enemy>,
    enemy_move_time: f64,
    
    // Score multiplier power-up (for player 1 in Hard mode)
    score_multiplier: i32,
    multiplier_timer: f64,
    multiplier_duration: f64,
}

impl Game {
    pub fn new(width: i32, height: i32, game_mode: GameMode) -> Game {
        let time_limit = game_mode.get_time_limit();
        let center_x = width / 2;
        let center_y = height / 2;
        // Create snakes facing opposite directions with more spacing
        // Player 1: left side, facing right (away from center)
        let snake1 = Snake::new(center_x - 8, center_y);
        // Player 2: right side, facing left (away from center)
        let snake2 = Snake::new_left(center_x + 8, center_y);
        Game {
            snake1,
            snake2,
            waiting_time: 0.0,
            food_exist: false,
            food_x: 0,
            food_y: 0,
            width,
            height,
            game_over: false,
            score1: 0,
            score2: 0,
            level: 1,
            foods_eaten: 0,
            game_mode,
            final_score: 0,
            final_level: 1,
            game_time: 0.0,
            time_limit,
            high_score: 0,
            enemies: Vec::new(),
            enemy_move_time: 0.0,
            score_multiplier: 1,
            multiplier_timer: 0.0,
            multiplier_duration: 0.0,
        }
    }
    
    pub fn restart_game(&mut self) {
        let center_x = self.width / 2;
        let center_y = self.height / 2;
        self.snake1 = Snake::new(center_x - 8, center_y);
        self.snake2 = Snake::new_left(center_x + 8, center_y);
        self.waiting_time = 0.0;
        self.food_exist = false;
        self.food_x = 0;
        self.food_y = 0;
        self.game_over = false;
        self.score1 = 0;
        self.score2 = 0;
        self.level = 1;
        self.foods_eaten = 0;
        self.final_score = 0;
        self.final_level = 1;
        self.enemies.clear();
        self.enemy_move_time = 0.0;
        self.score_multiplier = 1;
        self.multiplier_timer = 0.0;
        self.multiplier_duration = 0.0;
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        // Player 1 controls (Arrow keys)
        let dir1 = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };
        
        if let Some(dir) = dir1 {
            if dir != self.snake1.head_direction().opposite() {
                self.update_snake1(Some(dir));
            }
            return;
        }

        // Player 2 controls (WASD keys)
        let dir2 = match key {
            Key::W => Some(Direction::Up),
            Key::S => Some(Direction::Down),
            Key::A => Some(Direction::Left),
            Key::D => Some(Direction::Right),
            _ => None,
        };
        
        if let Some(dir) = dir2 {
            if dir != self.snake2.head_direction().opposite() {
                self.update_snake2(Some(dir));
            }
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Draw both snakes with different colors
        self.snake1.draw(con, g, true);  // Player 1 - Green
        self.snake2.draw(con, g, false); // Player 2 - Blue

        if self.food_exist {
            draw_apple(self.food_x, self.food_y, con, g);
        }
        
        // Draw enemies (level 3+)
        for enemy in &self.enemies {
            enemy.draw(con, g);
        }
        
        // Draw multiplier indicator if active (only in Hard mode)
        if matches!(self.game_mode, crate::menu::GameMode::Hard) && self.score_multiplier > 1 {
            let multiplier_color: Color = [1.0, 0.0, 1.0, 1.0]; // Magenta
            let multiplier_y = 4;
            // Draw "X2" or "X3" etc indicator
            for i in 0..self.score_multiplier.min(5) {
                draw_block(multiplier_color, self.width + 1 + i, multiplier_y, con, g);
            }
            // Show remaining time as blocks
            let time_left = ((self.multiplier_duration - self.multiplier_timer) / 3.0) as i32;
            let time_blocks = time_left.min(10);
            for i in 0..time_blocks {
                draw_block([0.5, 0.0, 0.5, 1.0], self.width + 1 + i, multiplier_y + 1, con, g);
            }
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);

        // Display Player 1 score (Yellow blocks)
        let score1_color: Color = [1.0, 1.0, 0.0, 1.0];
        let score1_blocks = (self.score1 / 10).min(10);
        for i in 0..score1_blocks {
            draw_block(score1_color, self.width + 1 + i, 0, con, g);
        }
        
        // Display Player 2 score (Orange blocks)
        let score2_color: Color = [1.0, 0.5, 0.0, 1.0];
        let score2_blocks = (self.score2 / 10).min(10);
        for i in 0..score2_blocks {
            draw_block(score2_color, self.width + 1 + i, 1, con, g);
        }

        // Display level visually using colored blocks
        let level_color: Color = [0.0, 0.5, 1.0, 1.0];
        let level_blocks = self.level.min(10); // Show up to 10 blocks
        for i in 0..level_blocks {
            draw_block(level_color, self.width + 1 + i, 2, con, g);
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
            draw_block(mode_color, self.width + 1 + i, 3, con, g);
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
                self.final_score = self.score1.max(self.score2);
                self.final_level = self.level;
                let max_score = self.score1.max(self.score2);
                if max_score > self.high_score {
                    self.high_score = max_score;
                }
                return;
            }
        }

        if !self.food_exist {
            self.add_food();
        }
        
        // Update score multiplier timer (only in Hard mode)
        if matches!(self.game_mode, crate::menu::GameMode::Hard) && self.score_multiplier > 1 {
            self.multiplier_timer += delta_time;
            if self.multiplier_timer >= self.multiplier_duration {
                self.score_multiplier = 1;
                self.multiplier_timer = 0.0;
            }
        } else if !matches!(self.game_mode, crate::menu::GameMode::Hard) {
            // Reset multiplier if not in Hard mode
            self.score_multiplier = 1;
            self.multiplier_timer = 0.0;
        }
        
        // Spawn enemies based on level - starting from level 1
        // Level 1: 1 enemy, Level 3: 2 enemies, Level 5: 3 enemies, Level 7+: 4 enemies
        let target_enemy_count = match self.level {
            1..=2 => 1,
            3..=4 => 2,
            5..=6 => 3,
            _ => 4,
        };
        
        while self.enemies.len() < target_enemy_count {
            self.spawn_enemy();
        }
        
        // Update enemy movement
        self.enemy_move_time += delta_time;
        if self.enemy_move_time > 0.3 { // Enemies move every 0.3 seconds
            for enemy in &mut self.enemies {
                enemy.update(self.width, self.height);
            }
            self.enemy_move_time = 0.0;
        }

        // Calculate moving period based on level and game mode
        let base_speed = self.game_mode.get_base_speed();
        let speed_multiplier = self.game_mode.get_speed_multiplier();
        let moving_period = base_speed / (1.0 + (self.level as f64) * speed_multiplier);
        if self.waiting_time > moving_period {
            self.update_snake1(None);
            self.update_snake2(None);
            self.waiting_time = 0.0; // Reset timer after both snakes move
        }
    }

    fn check_eating(&mut self) {
        let (head1_x, head1_y) = self.snake1.head_position();
        let (head2_x, head2_y) = self.snake2.head_position();
        
        // Check if player 1 eats food
        if self.food_exist && self.food_x == head1_x && self.food_y == head1_y {
            self.food_exist = false;
            self.snake1.restore_tail();
            
            let base_score = 10;
            let final_score = if matches!(self.game_mode, crate::menu::GameMode::Hard) {
                base_score * self.score_multiplier
            } else {
                base_score
            };
            self.score1 += final_score;
            
            // Random chance to spawn multiplier power-up (10% chance, only in Hard mode)
            if matches!(self.game_mode, crate::menu::GameMode::Hard) {
                let mut rng = rng();
                if rng.random_range(0..100) < 10 && self.score_multiplier == 1 {
                    self.score_multiplier = 2;
                    self.multiplier_duration = 15.0;
                    self.multiplier_timer = 0.0;
                }
            }
            
            self.foods_eaten += 1;
            self.update_level();
            return;
        }
        
        // Check if player 2 eats food
        if self.food_exist && self.food_x == head2_x && self.food_y == head2_y {
            self.food_exist = false;
            self.snake2.restore_tail();
            self.score2 += 10; // Player 2 always gets base score
            self.foods_eaten += 1;
            self.update_level();
        }
    }
    
    fn update_level(&mut self) {
        let new_level = (self.foods_eaten / FOOD_PER_LEVEL) + 1;
        if new_level > self.level {
            self.level = new_level;
        }
    }

    fn check_if_snake_alive(&self, snake: &Snake, other_snake: &Snake, dir: Option<Direction>) -> bool {
        let (next_x, next_y): (i32, i32) = snake.next_head(dir);

        // Check collision with own tail
        if snake.overlap_tail(next_x, next_y) {
            return false;
        }
        
        // Check collision with other snake
        if other_snake.overlap_tail(next_x, next_y) {
            return false;
        }
        
        // Check collision with enemies
        for enemy in &self.enemies {
            if enemy.check_collision(next_x, next_y) {
                return false;
            }
        }
        
        // Check wall collision
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }
    
    fn spawn_enemy(&mut self) {
        use rand::{rng, Rng};
        let mut rng = rng();
        // Spawn enemy away from both snakes and food
        let (snake1_x, snake1_y) = self.snake1.head_position();
        let (snake2_x, snake2_y) = self.snake2.head_position();
        let mut enemy_x = rng.random_range(1..self.width - 1);
        let mut enemy_y = rng.random_range(1..self.height - 1);
        
        // Make sure enemy doesn't spawn on snakes, food, or other enemies
        let mut attempts = 0;
        loop {
            let mut valid_position = true;
            
            // Check snake1 collision
            if (enemy_x == snake1_x && enemy_y == snake1_y) || 
               self.snake1.overlap_tail(enemy_x, enemy_y) {
                valid_position = false;
            }
            
            // Check snake2 collision
            if (enemy_x == snake2_x && enemy_y == snake2_y) || 
               self.snake2.overlap_tail(enemy_x, enemy_y) {
                valid_position = false;
            }
            
            // Check food collision
            if enemy_x == self.food_x && enemy_y == self.food_y {
                valid_position = false;
            }
            
            // Check other enemies
            for existing_enemy in &self.enemies {
                if existing_enemy.check_collision(enemy_x, enemy_y) {
                    valid_position = false;
                    break;
                }
            }
            
            if valid_position {
                break;
            }
            
            enemy_x = rng.random_range(1..self.width - 1);
            enemy_y = rng.random_range(1..self.height - 1);
            attempts += 1;
            if attempts > 100 {
                // Fallback: spawn at a safe corner
                enemy_x = self.width / 2;
                enemy_y = self.height / 2;
                break;
            }
        }
        
        self.enemies.push(Enemy::new(enemy_x, enemy_y));
    }

    fn add_food(&mut self) {
        let mut rng = rng();
        let mut new_x = rng.random_range(1..self.width - 1);
        let mut new_y = rng.random_range(1..self.height - 1);
        let mut attempts = 0;
        loop {
            let mut valid_position = true;
            
            // Check snake1 collision
            if self.snake1.overlap_tail(new_x, new_y) {
                valid_position = false;
            }
            
            // Check snake2 collision
            if self.snake2.overlap_tail(new_x, new_y) {
                valid_position = false;
            }
            
            // Check enemy collisions
            for enemy in &self.enemies {
                if enemy.check_collision(new_x, new_y) {
                    valid_position = false;
                    break;
                }
            }
            
            if valid_position {
                break;
            }
            
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

    fn update_snake1(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(&self.snake1, &self.snake2, dir) {
            self.snake1.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
            self.final_score = self.score1.max(self.score2);
            self.final_level = self.level;
            if matches!(self.game_mode, crate::menu::GameMode::Survival) {
                let max_score = self.score1.max(self.score2);
                if max_score > self.high_score {
                    self.high_score = max_score;
                }
            }
        }
    }
    
    fn update_snake2(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(&self.snake2, &self.snake1, dir) {
            self.snake2.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
            self.final_score = self.score1.max(self.score2);
            self.final_level = self.level;
            if matches!(self.game_mode, crate::menu::GameMode::Survival) {
                let max_score = self.score1.max(self.score2);
                if max_score > self.high_score {
                    self.high_score = max_score;
                }
            }
        }
    }

    pub fn should_return_to_menu(&self) -> bool {
        self.game_over && self.waiting_time > RESTART_TIME
    }
    
    pub fn get_score(&self) -> i32 {
        self.score1.max(self.score2)
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
        
        // Draw Player 1 final score
        let score1_y = center_y + 2;
        let score1_color: Color = [1.0, 1.0, 0.0, 1.0]; // Yellow
        let score1_blocks = (self.score1 / 10).min(12);
        for i in 0..score1_blocks {
            draw_block(score1_color, 3 + i, score1_y, con, g);
        }
        
        // Draw Player 2 final score
        let score2_y = center_y + 4;
        let score2_color: Color = [1.0, 0.5, 0.0, 1.0]; // Orange
        let score2_blocks = (self.score2 / 10).min(12);
        for i in 0..score2_blocks {
            draw_block(score2_color, 3 + i, score2_y, con, g);
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