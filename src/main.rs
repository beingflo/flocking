extern crate nannou;

mod agent;
mod arena;

use nannou::prelude::*;

use arena::Arena;

const HEIGHT: u32 = 800;
const WIDTH: u32 = 800;

fn main() {
    nannou::app(model, event, view).run();
}

struct Model {
    arena: Arena,
}

fn model(app: &App) -> Model {
    let window = app.new_window().with_title("Algen").build().unwrap();

    let mut arena = Arena::new();

    for _ in 0..10 {
        arena.add_agent();
    }

    Model { arena: arena }
}

fn event(app: &App, mut model: Model, event: Event) -> Model {
    match event {
        Event::Update(update) => {
            model.arena.step(update.since_last.secs() as f32);
        },
        _ => (),
    }
    model
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    draw.background().color(WHITE);

    model.arena.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
    frame
}
