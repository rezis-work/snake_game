use piston_window::{ellipse, rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    )
}

pub fn draw_circle(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    ellipse(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    )
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [x,
        y,
        BLOCK_SIZE * (width as f64),
        BLOCK_SIZE * (height as f64)],
        con.transform,
        g,
    )
}

pub fn draw_apple(x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    
    // Apple body - red with slight gradient effect
    let apple_red: Color = [0.8, 0.1, 0.1, 1.0];
    let apple_highlight: Color = [0.9, 0.3, 0.2, 1.0];
    
    // Draw main apple body
    ellipse(
        apple_red,
        [gui_x, gui_y, BLOCK_SIZE * 0.9, BLOCK_SIZE * 0.9],
        con.transform,
        g,
    );
    
    // Draw highlight on apple
    ellipse(
        apple_highlight,
        [gui_x + BLOCK_SIZE * 0.2, gui_y + BLOCK_SIZE * 0.2, BLOCK_SIZE * 0.3, BLOCK_SIZE * 0.3],
        con.transform,
        g,
    );
    
    // Draw stem (brown rectangle)
    let stem_color: Color = [0.4, 0.2, 0.1, 1.0];
    rectangle(
        stem_color,
        [gui_x + BLOCK_SIZE * 0.45, gui_y - BLOCK_SIZE * 0.1, BLOCK_SIZE * 0.1, BLOCK_SIZE * 0.15],
        con.transform,
        g,
    );
    
    // Draw leaf (green ellipse)
    let leaf_color: Color = [0.2, 0.7, 0.2, 1.0];
    ellipse(
        leaf_color,
        [gui_x + BLOCK_SIZE * 0.6, gui_y - BLOCK_SIZE * 0.05, BLOCK_SIZE * 0.2, BLOCK_SIZE * 0.15],
        con.transform,
        g,
    );
}
