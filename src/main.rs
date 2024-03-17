use macroquad::prelude::*;

// creates the actual window and and gives it a name
#[macroquad::main("CSPFinal")]

async fn main() {
    loop {
        let mut speed = 10.0; // probably should have a speed variable to move

        clear_background(BLACK);
        let player = draw_rectangle(xscreen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0, GREEN);

        if is_key_down(KeyCode::W) {
            // move up 
            player.y += speed;
        }
        if is_key_down(KeyCode::S) {
            // move down 
            player.y -= speed;
        }
        if is_key_down(KeyCode::A) {
            // move left 
            player.x -= speed;
        }
        if is_key_down(KeyCode::D) {
            // move right
            player.x += speed;
        }

        next_frame().await; // I think this draws
    }
}
