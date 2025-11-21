extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;
mod menu;
mod enemy;

use piston_window::*;
use piston_window::types::Color;
use game::Game;
use menu::{Menu, GameState};

use crate::draw::to_coord_u32;

const BLACK_COLOR: Color = [0.1, 0.1, 0.1, 1.0];

fn main() {
    let (width, height) = (20, 20);
    // Make window wider to accommodate score/level display
    let window_width = to_coord_u32(width + 12); // Extra space for score/level blocks
    let window_height = to_coord_u32(height);

    let mut window: PistonWindow = WindowSettings::new("Snake", [window_width, window_height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut menu = Menu::new();
    let mut game_state = GameState::Menu;
    let mut game: Option<Game> = None;

    while let Some(event) = window.next() {
        match game_state {
            GameState::Menu => {
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    match key {
                        Key::Up => menu.select_prev(),
                        Key::Down => menu.select_next(),
                        Key::Return => {
                            // Start game with selected mode
                            game = Some(Game::new(width, height, menu.get_selected_mode()));
                            game_state = GameState::Playing;
                        }
                        _ => {}
                    }
                }
                
                window.draw_2d(&event, |c, g, _device| {
                    clear(BLACK_COLOR, g);
                    menu.draw(width, height, &c, g);
                });
            }
            
            GameState::Playing => {
                if let Some(ref mut game) = game {
                    if let Some(Button::Keyboard(key)) = event.press_args() {
                        game.key_pressed(key);
                    }
                    
                    window.draw_2d(&event, |c, g, _device| {
                        clear(BLACK_COLOR, g);
                        game.draw(&c, g);
                    });

                    event.update(|arg| {
                        game.update(arg.dt);
                        if game.should_return_to_menu() {
                            game_state = GameState::GameOver;
                        }
                    });
                }
            }
            
            GameState::GameOver => {
                if let Some(ref mut current_game) = game {
                    if let Some(Button::Keyboard(_key)) = event.press_args() {
                        // Return to menu on any key press
                        game_state = GameState::Menu;
                        game = None;
                    } else {
                        window.draw_2d(&event, |c, g, _device| {
                            clear(BLACK_COLOR, g);
                            current_game.draw(&c, g);
                        });

                        event.update(|arg| {
                            current_game.update(arg.dt);
                        });
                    }
                }
            }
        }
    }
}
