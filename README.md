# Snake Game in Rust

A modern Snake game implementation written in Rust using the [Piston](https://github.com/PistonDevelopers/piston) game engine.

## Description

Control the snake to eat apples and grow longer. Avoid hitting the walls or your own tail! Choose from multiple game modes and difficulty levels.

## Prerequisites

You need to have Rust and Cargo installed on your system. You can install them from [rustup.rs](https://rustup.rs/).

## How to Run

1. Navigate to the project directory:
   ```bash
   cd snake
   ```

2. Run the game using Cargo:
   ```bash
   cargo run
   ```

## Controls

- **Arrow Keys** (Up, Down, Left, Right): Move the snake
- **Enter**: Select menu option / Start game
- **Esc**: Exit the game

## Game Modes

### Difficulty Modes
- **Easy**: Slower speed, gradual acceleration
- **Medium**: Balanced speed and acceleration
- **Hard**: Fast speed, rapid acceleration

### Special Modes
- **Timer**: 60-second challenge - Score as many points as possible!
- **Survival**: Endless mode with high score tracking

## Features

- **Beautiful Graphics**: Snake with eyes, realistic apples with stems
- **Multiple Game Modes**: 5 different modes to choose from
- **Progressive Difficulty**: Speed increases with each level
- **Score & Level System**: Earn points and advance levels
- **Visual Indicators**: 
  - Yellow blocks = Score (each block = 10 points)
  - Blue blocks = Current level
  - Purple blocks = Active game mode
  - Cyan blocks = Timer countdown (Timer mode only)
- **High Score Tracking**: Best score saved in Survival mode
- **Game Over Screen**: Shows final score, level, and mode
- **Polished UI**: Menu system with clear mode selection

