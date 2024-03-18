use macroquad::prelude::*;

// creates the actual window and and gives it a name
#[macroquad::main("CSPFinal")]

async fn main() {

    let mut player = draw_rectangle(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0, GREEN);
    let mut speed = 10.0; // probably should have a speed variable to move

    loop {
        clear_background(BLACK);

        let mut player_pos = vec2(screen_width() / 2.0, screen_height() / 2.0);

        if is_key_down(KeyCode::W) {
            // move up 
            player_pos.y += speed;
        }
        if is_key_down(KeyCode::S) {
            // move down 
            player_pos.y -= speed;
        }
        if is_key_down(KeyCode::A) {
            // move left 
            player_pos.x -= speed;
        }
        if is_key_down(KeyCode::D) {
            // move right
            player_pos.x += speed;
        }



        next_frame().await; // I think this draws
    }
}
