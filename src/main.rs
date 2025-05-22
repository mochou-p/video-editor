// video-editor/src/main.rs

use macroquad::prelude::*;

#[macroquad::main("idk")]
async fn main() {
    loop {
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}

