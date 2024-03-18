use macroquad::prelude::*;

// creates the actual window and and gives it a name
#[macroquad::main("CSPFinal")]
async fn main() {
    
    struct Player {
        pos: Vec2,
        speed: Vec2
    }

    let mut player = Player {
        pos: vec2(screen_width() / 2., screen_height() / 2.),
        speed: vec2(5., 5.)
    };

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::W) {
            player.pos.y -= player.speed.y;
        }
        if is_key_down(KeyCode::S) {
            player.pos.y += player.speed.y;
        }
        if is_key_down(KeyCode::A) {
            player.pos.x -= player.speed.x;
        }
        if is_key_down(KeyCode::D) {
            player.pos.x += player.speed.x;
        }

        draw_rectangle(player.pos.x, player.pos.y, 10., 10., GREEN);

        next_frame().await; // I think this draws
    }
}
