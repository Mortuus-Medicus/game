use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        let texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/CrystalSagaProfileImage.png"),
            None,
        );

        draw_texture(texture, 50.0, 50.0, WHITE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, BLUE);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, MAGENTA);
        draw_text(&get_fps().to_string(), 20.0, 50.0, 30.0, PINK);

        let temp_mouse_position = mouse_position();

        if is_mouse_button_down(MouseButton::Left) == true {
            draw_circle(temp_mouse_position.0, temp_mouse_position.1, 5.0, RED);
        } else {
            draw_circle(temp_mouse_position.0, temp_mouse_position.1, 5.0, WHITE);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await
    }
}
