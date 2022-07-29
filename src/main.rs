use macroquad::prelude::*;

#[macroquad::main("MyProject")] // TODO: name your project
async fn main() {
    loop {
        //  clear_background(RED);
        //
        //  draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //  draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //  draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //  draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);
        //
        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });

        set_default_camera();

        draw_checkerboard();

        next_frame().await
    }
}

pub fn draw_checkerboard() {
    for i in 0..=(screen_width() / 20.) as u32 {
        for j in 0..=(screen_height() / 20.) as u32 {
            draw_rectangle(
                i as f32 * 20. - 10.,
                j as f32 * 20. - 10.,
                20.,
                20.,
                match (i + j) % 2 {
                    0 => Color::from_rgba(43, 46, 51, 255),
                    _ => Color::from_rgba(59, 62, 67, 255),
                },
            )
        }
    }
}
