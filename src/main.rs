extern crate nannou;

mod agent;
mod arena;

use nannou::prelude::*;
use nannou::event::SimpleWindowEvent;

use arena::Arena;

fn main() {
    nannou::app(model, event, view).run();
}

struct Model {
    arena: Arena,
}

fn model(app: &App) -> Model {
    let window = app.new_window().with_title("Algen").build().unwrap();

    let (width, height) = app.main_window().inner_size_points();

    let mut arena = Arena::new(width, height);

    for _ in 0..100 {
        arena.add_agent();
    }

    Model { arena: arena }
}

fn event(app: &App, mut model: Model, event: Event) -> Model {
    match event {
        Event::Update(update) => {
            model.arena.step(update.since_last.secs() as f32);
        },
        Event::WindowEvent { simple: Some(SimpleWindowEvent::Resized(size)), .. } => {
            model.arena.update_size(size);
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
