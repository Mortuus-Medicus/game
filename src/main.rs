use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, RED);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, MAGENTA);
        draw_text("Wee Woo!", 20.0, 50.0, 30.0, PINK);

        next_frame().await
    }
}
