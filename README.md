# Snake Game in Rust

A classic Snake game implementation written in Rust using the [Piston](https://github.com/PistonDevelopers/piston) game engine.

## Description

Control the snake to eat food and grow longer. Avoid hitting the walls or your own tail!

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

- **Arrow Keys** (Up, Down, Left, Right): Move the snake.
- **Esc**: Exit the game.

## Features

- Classic snake gameplay.
- Score increases as you eat food (snake grows).
- Game Over state when hitting walls or self.
- Automatic restart after Game Over.

