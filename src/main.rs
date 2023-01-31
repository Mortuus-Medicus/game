// To compile for Android run this command while having Docker Destop running: docker run --rm -v ${pwd}:/root/src -w /root/src notfl3/cargo-apk cargo quad-apk build --release

use macroquad::prelude::*;

struct Player {
    x: f32,
    y: f32,
    walking_speed: f32,
    direction: i8, // 0 - N, 1 - E, 2 - S, 3 - W
    animation: f32,
    moving: bool,
    attack_speed: f32,
    attacking: bool,
}

#[macroquad::main("Game")]
async fn main() {
    macroquad::file::set_pc_assets_folder("assets");
    let texture: Texture2D = load_texture("Overworld.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    //let tileset_width = 32.0;
    //let tileset_height = 32.0;

    let world_size = [ 14, 10 ];
    let tile_size = [ 40.0, 20.0 ];
    let origin = [ 5.0, 1.0 ];

    let player_texture: Texture2D = load_texture("character.png").await.unwrap();
    let my_tiles: Texture2D = load_texture("tiles.png").await.unwrap();
    player_texture.set_filter(FilterMode::Nearest);
    my_tiles.set_filter(FilterMode::Nearest);

    let mut player = Player {
        x: 100.0,
        y: 200.0,
        walking_speed: 2.0,
        direction: 2,
        animation: 0.0,
        moving: false,
        attack_speed: 2.0,
        attacking: false,
    };

    loop {
        // clear_background(DARKGRAY);
        clear_background(BEIGE);
        //let player_vision_width = ((screen_width() / tileset_width) + 1.0) as i32;
        //let player_vision_height = ((screen_height() / tileset_height) + 1.0) as i32;

        
/*
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
*/

        for y in 0..world_size[0] {
            for x in 0..world_size[1] {
                draw_texture_ex(
                    my_tiles,
                    (origin[0] * tile_size[0]) + (y as f32 - x as f32) * (tile_size[0] / 2.0),
                    (origin[1] * tile_size[1]) + (x as f32 + y as f32) * (tile_size[1] / 2.0),
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(40.0, 20.0)),
                        source: Some(Rect::new(40.0, 0.0, 40.0, 20.0)),
                        ..Default::default()
                    },
                );
            }
        }

        


        draw_text(&format!("FPS: {}", &get_fps().to_string()), 10.0, 20.0, 25.0, MAGENTA);
        draw_text(&format!("Cell: {} {}", &(mouse_position().0 / tile_size[0]).floor().to_string(), &(mouse_position().1 / tile_size[1]).floor().to_string()), 10.0, 40.0, 25.0, MAGENTA);
        
        #[cfg(target_os = "android")]
        draw_text("Android - Attribute", 10.0, 80.0, 25.0, MAGENTA);
        
        #[cfg(target_os = "windows")]
        draw_text("Windows - Attribute", 10.0, 80.0, 25.0, MAGENTA);
        
        #[cfg(target_os = "linux")]
        draw_text("Linux - Attribute", 10.0, 80.0, 25.0, MAGENTA);

        if cfg!(target_os = "android") {
            draw_text("Android - Macro", 10.0, 60.0, 25.0, MAGENTA);
        } else if cfg!(target_os = "windows") {
            draw_text("Windows - Macro", 10.0, 60.0, 25.0, MAGENTA);
        } else if cfg!(target_os = "linux") {
			draw_text("Linux - Macro", 10.0, 60.0, 25.0, MAGENTA);
        }

    
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

        if is_key_down(KeyCode::A) {
            player.x -= player.walking_speed;
            player.moving = true;
            if player.attacking == false {
                player.direction = 3;
            }
        }
        if is_key_down(KeyCode::D) {
            player.x += player.walking_speed;
            player.moving = true;
            if player.attacking == false {
                player.direction = 1;
            }
        }
        if is_key_down(KeyCode::W) {
            player.y -= player.walking_speed;
            player.moving = true;
            if player.attacking == false {
                player.direction = 0;
            }
        }
        if is_key_down(KeyCode::S) {
            player.y += player.walking_speed;
            player.moving = true;
            if player.attacking == false {
                player.direction = 2;
            }
        }

        if player.attacking == false {
            if is_key_down(KeyCode::Left) {
                player.animation = 0.0;
                player.attacking = true;
                player.direction = 3;
            }
            if is_key_down(KeyCode::Right) {
                player.animation = 0.0;
                player.attacking = true;
                player.direction = 1;
            }
            if is_key_down(KeyCode::Up) {
                player.animation = 0.0;
                player.attacking = true;
                player.direction = 0;
            }
            if is_key_down(KeyCode::Down) {
                player.animation = 0.0;
                player.attacking = true;
                player.direction = 2;
            }
        }

        if player.attacking == true {
            if (player.animation + player.attack_speed / 10.0) >= 4.0 {
                player.animation = 0.0;
                player.attacking = false;
            } else {
                player.animation += player.attack_speed / 10.0;
            }
        } else if player.moving == true {
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
                if player.attacking == true {
                    draw_texture_ex(
                        player_texture,
                        player.x,
                        player.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(64.0, 64.0)),
                            source: Some(Rect::new(
                                32.0 * player.animation.floor(),
                                5.0 * 32.0,
                                32.0,
                                32.0,
                            )),
                            ..Default::default()
                        },
                    );
                } else {
                    draw_texture_ex(
                        player_texture,
                        player.x,
                        player.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(32.0, 64.0)),
                            source: Some(Rect::new(
                                16.0 * player.animation.floor(),
                                2.0 * 32.0,
                                16.0,
                                32.0,
                            )),
                            ..Default::default()
                        },
                    );
                }
            }
            1 => {
                if player.attacking == true {
                    draw_texture_ex(
                        player_texture,
                        player.x,
                        player.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(64.0, 64.0)),
                            source: Some(Rect::new(
                                32.0 * player.animation.floor(),
                                6.0 * 32.0,
                                32.0,
                                32.0,
                            )),
                            ..Default::default()
                        },
                    );
                } else {
                    draw_texture_ex(
                        player_texture,
                        player.x,
                        player.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(32.0, 64.0)),
                            source: Some(Rect::new(
                                16.0 * player.animation.floor(),
                                1.0 * 32.0,
                                16.0,
                                32.0,
                            )),
                            ..Default::default()
                        },
                    );
                }
            }
            2 => {
                if player.attacking == true {
                    draw_texture_ex(
                        player_texture,
                        player.x,
                        player.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(64.0, 64.0)),
                            source: Some(Rect::new(
                                32.0 * player.animation.floor(),
                                4.0 * 32.0,
                                32.0,
                                32.0,
                            )),
                            ..Default::default()
                        },
                    );
                } else {
                    draw_texture_ex(
                        player_texture,
                        player.x,
                        player.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(32.0, 64.0)),
                            source: Some(Rect::new(
                                16.0 * player.animation.floor(),
                                0.0 * 32.0,
                                16.0,
                                32.0,
                            )),
                            ..Default::default()
                        },
                    );
                }
            }
            3 => {
                if player.attacking == true {
                    draw_texture_ex(
                        player_texture,
                        player.x,
                        player.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(64.0, 64.0)),
                            source: Some(Rect::new(
                                32.0 * player.animation.floor(),
                                7.0 * 32.0,
                                32.0,
                                32.0,
                            )),
                            ..Default::default()
                        },
                    );
                } else {
                    draw_texture_ex(
                        player_texture,
                        player.x,
                        player.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(32.0, 64.0)),
                            source: Some(Rect::new(
                                16.0 * player.animation.floor(),
                                3.0 * 32.0,
                                16.0,
                                32.0,
                            )),
                            ..Default::default()
                        },
                    );
                }
            }

            _ => {}
        }

        next_frame().await
    }
}
