use piston_window::types::Color;
use crate::draw::{draw_block, draw_rectangle};

#[derive(Copy, Clone, PartialEq)]
pub enum GameMode {
    Easy,
    Medium,
    Hard,
    Timer,      // Play for 60 seconds
    Survival,   // Endless mode with high score tracking
}

impl GameMode {
    pub fn get_base_speed(&self) -> f64 {
        match self {
            GameMode::Easy => 0.4,       // Slower start
            GameMode::Medium => 0.3,    // Medium start
            GameMode::Hard => 0.2,      // Faster start
            GameMode::Timer => 0.3,     // Medium speed for timer mode
            GameMode::Survival => 0.3,   // Medium speed for survival mode
        }
    }
    
    pub fn get_speed_multiplier(&self) -> f64 {
        match self {
            GameMode::Easy => 0.05,      // Slower acceleration
            GameMode::Medium => 0.08,   // Medium acceleration
            GameMode::Hard => 0.12,      // Faster acceleration
            GameMode::Timer => 0.08,     // Medium acceleration
            GameMode::Survival => 0.08,  // Medium acceleration
        }
    }
    
    pub fn get_name(&self) -> &str {
        match self {
            GameMode::Easy => "EASY",
            GameMode::Medium => "MEDIUM",
            GameMode::Hard => "HARD",
            GameMode::Timer => "TIMER",
            GameMode::Survival => "SURVIVAL",
        }
    }
    
    pub fn is_timed_mode(&self) -> bool {
        matches!(self, GameMode::Timer)
    }
    
    pub fn get_time_limit(&self) -> f64 {
        match self {
            GameMode::Timer => 60.0,  // 60 seconds
            _ => 0.0,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

pub struct Menu {
    pub selected_mode: usize,
    pub modes: Vec<GameMode>,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            selected_mode: 1, // Default to Medium
            modes: vec![
                GameMode::Easy, 
                GameMode::Medium, 
                GameMode::Hard,
                GameMode::Timer,
                GameMode::Survival
            ],
        }
    }
    
    pub fn select_next(&mut self) {
        self.selected_mode = (self.selected_mode + 1) % self.modes.len();
    }
    
    pub fn select_prev(&mut self) {
        if self.selected_mode == 0 {
            self.selected_mode = self.modes.len() - 1;
        } else {
            self.selected_mode -= 1;
        }
    }
    
    pub fn get_selected_mode(&self) -> GameMode {
        self.modes[self.selected_mode]
    }
    
    
    pub fn draw(&self, width: i32, height: i32, con: &piston_window::Context, g: &mut piston_window::G2d) {
        // Draw semi-transparent background
        let bg_color: Color = [0.0, 0.0, 0.0, 0.8];
        draw_rectangle(bg_color, 0, 0, width, height, con, g);
        
        // Draw title
        let title_color: Color = [1.0, 1.0, 1.0, 1.0];
        let title_y = height / 2 - 6;
        
        // Draw "SNAKE" title using blocks
        let title_blocks = vec![
            (3, title_y), (4, title_y), (5, title_y), (6, title_y), (7, title_y), // S
            (9, title_y), (10, title_y), (11, title_y), (12, title_y), (13, title_y), // N
            (15, title_y), (16, title_y), (17, title_y), // A
            (3, title_y + 2), (4, title_y + 2), (5, title_y + 2), (6, title_y + 2), (7, title_y + 2), // K
            (9, title_y + 2), (10, title_y + 2), (11, title_y + 2), (12, title_y + 2), (13, title_y + 2), // E
        ];
        
        for (x, y) in title_blocks {
            draw_block(title_color, x, y, con, g);
        }
        
        // Draw mode selection
        let mode_y_start = height / 2 + 2;
        for (i, mode) in self.modes.iter().enumerate() {
            let mode_y = mode_y_start + (i as i32 * 3);
            let is_selected = i == self.selected_mode;
            
            let mode_color = if is_selected {
                [0.0, 1.0, 0.0, 1.0] // Green for selected
            } else {
                [0.7, 0.7, 0.7, 1.0] // Gray for unselected
            };
            
            // Draw selection indicator
            if is_selected {
                draw_block([1.0, 1.0, 0.0, 1.0], 5, mode_y, con, g); // Yellow arrow
            }
            
            // Draw simple mode indicator - colored blocks instead of text
            let name_start_x = 7;
            let mode_indicator = match mode {
                GameMode::Easy => 1,
                GameMode::Medium => 2,
                GameMode::Hard => 3,
                GameMode::Timer => 4,
                GameMode::Survival => 5,
            };
            // Draw blocks to represent mode
            for i in 0..mode_indicator {
                draw_block(mode_color, name_start_x + i, mode_y, con, g);
            }
        }
        
        // Draw instructions
        let inst_color: Color = [0.5, 0.5, 0.5, 1.0];
        let inst_y = height - 3;
        // "UP/DOWN: Select, ENTER: Start"
        let inst_text = vec![
            (3, inst_y), (4, inst_y), (5, inst_y), // UP
            (7, inst_y), (8, inst_y), (9, inst_y), (10, inst_y), (11, inst_y), // DOWN
            (13, inst_y), (14, inst_y), (15, inst_y), (16, inst_y), (17, inst_y), // ENTER
        ];
        for (x, y) in inst_text {
            draw_block(inst_color, x, y, con, g);
        }
    }
}

