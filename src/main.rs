use macroquad::prelude::*;

struct Player {
    x: f32,
    y: f32,
    walking_speed: f32,
    direction: i8, // 0 - N, 1 - E, 2 - S, 3 - W
    animation: f32,
    moving: bool,
}

#[macroquad::main("Game")]
async fn main() {
    let texture: Texture2D = load_texture("assets/Overworld.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let tileset_width = 32.0;
    let tileset_height = 32.0;

    let player_texture: Texture2D = load_texture("assets/character.png").await.unwrap();
    player_texture.set_filter(FilterMode::Nearest);


    let mut player = Player {
        x: 100.0, 
        y: 200.0,
        walking_speed: 2.0,
        direction: 2,
        animation: 0.0,
        moving: false,
    };
    

    loop {
        clear_background(DARKGRAY);
        let player_vision_width = ((screen_width() / tileset_width) + 1.0) as i32;
        let player_vision_height = ((screen_height() / tileset_height) + 1.0) as i32;

        for i in 0..player_vision_width {
            for j in 0..player_vision_height {
                draw_texture_ex(
                    texture,
                    (i as f32) * tileset_width,
                    (j as f32) * tileset_height,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(32.0, 32.0)),
                        source: Some(Rect::new(0.0, 0.0, 16.0, 16.0)),
                        ..Default::default()
                    },
                );
            }
        }


        draw_text("FPS: ", 10.0, 20.0, 25.0, MAGENTA);
        draw_text(&get_fps().to_string(), 65.0, 20.0, 25.0, MAGENTA);


        let temp_mouse_position = mouse_position();

        if is_mouse_button_down(MouseButton::Left) == true {
            draw_circle(temp_mouse_position.0, temp_mouse_position.1, 5.0, RED);
        } else {
            draw_circle(temp_mouse_position.0, temp_mouse_position.1, 5.0, WHITE);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        player.moving = false;

        if is_key_down(KeyCode::A){
            player.x -= player.walking_speed;
            player.direction = 3;
            player.moving = true;
        }
        if is_key_down(KeyCode::D){
            player.x += player.walking_speed;
            player.direction = 1;
            player.moving = true;
        }
        if is_key_down(KeyCode::W){
            player.y -= player.walking_speed;
            player.direction = 0;
            player.moving = true;
        }
        if is_key_down(KeyCode::S){
            player.y += player.walking_speed;
            player.direction = 2;
            player.moving = true;
        }

        if player.moving == true {
            if (player.animation + player.walking_speed / 25.0) >= 4.0 {
                player.animation = 0.0;
            } else {
                player.animation += player.walking_speed / 25.0;
            }

            player.moving = false;
        } else {
            player.animation = 0.0;
        }

        match player.direction {
            0 => {
                draw_texture_ex(
                    player_texture,
                    player.x,
                    player.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(32.0, 64.0)),
                        source: Some(Rect::new(16.0 * player.animation.floor(), 2.0*32.0, 16.0, 32.0)),
                        ..Default::default()
                    },
                );
            },
            1 => {
                draw_texture_ex(
                    player_texture,
                    player.x,
                    player.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(32.0, 64.0)),
                        source: Some(Rect::new(16.0 * player.animation.floor(), 1.0*32.0, 16.0, 32.0)),
                        ..Default::default()
                    },
                );
            },
            2 => {
                draw_texture_ex(
                    player_texture,
                    player.x,
                    player.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(32.0, 64.0)),
                        source: Some(Rect::new(16.0 * player.animation.floor(), 0.0*32.0, 16.0, 32.0)),
                        ..Default::default()
                    },
                );
            },
            3 => {
                draw_texture_ex(
                    player_texture,
                    player.x,
                    player.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(32.0, 64.0)),
                        source: Some(Rect::new(16.0 * player.animation.floor(), 3.0*32.0, 16.0, 32.0)),
                        ..Default::default()
                    },
                );
            },
            
            _ => {
                
            },
        }

        

        next_frame().await
    }
}
