use macroquad::hash;
use macroquad::ui::root_ui;

use macroquad::{prelude::*, ui::widgets};
use rhai::Engine;

#[derive(Clone)]
enum ShapeKind {
    Rect,
    Circle,
}

#[allow(dead_code)]
#[derive(Clone)]
struct Shape {
    x: i64,
    y: i64,
    shape: ShapeKind,
}

impl Shape {
    fn rect(x: i64, y: i64) -> Self {
        Shape {
            x,
            y,
            shape: ShapeKind::Rect,
        }
    }

    fn circle(x: i64, y: i64) -> Self {
        Shape {
            x,
            y,
            shape: ShapeKind::Circle,
        }
    }
    fn draw(&self) {
        match self.shape {
            ShapeKind::Rect => draw_rectangle(self.x as f32, self.y as f32, 100., 100., WHITE),
            ShapeKind::Circle => draw_circle(self.x as f32, self.y as f32, 50., WHITE),
        }
    }
}

#[macroquad::main("Embeddable Language")]
async fn main() {
    let mut text = String::new();
    let mut boxes = Vec::<Shape>::new();

    let mut engine = Engine::new();
    let mut error = String::new();

    engine
        .register_fn("log", |label: &str| {
            debug!("{}", label);
        })
        .register_type::<Shape>()
        .register_fn("rect", Shape::rect)
        .register_fn("circle", Shape::circle);

    let script = "log(\"hello from Rhai\");";
    engine.run(script).unwrap();

    loop {
        draw_checkerboard();

        boxes.iter().for_each(|x| x.draw());

        widgets::Window::new(hash!(), vec2(400., 200.), vec2(330., 300.))
            .label("Code")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.editbox(hash!(), vec2(320., 200.), &mut text);
                if ui.button(vec2(0., 202.), "Run") {
                    match engine.eval::<Shape>(&text.to_string()) {
                        Ok(boxx) => {
                            boxes.push(boxx);
                            error = String::new();
                        }
                        Err(err) => {
                            error = err.to_string();
                        }
                    }
                };
                ui.label(vec2(0., 220.), &error);
            });

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
