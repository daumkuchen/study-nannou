use nannou::prelude::*;

fn main() {

    nannou::app(model)
        .update(update)
        .run();

}

struct Model {
}

fn model(app: &App) -> Model {

    app.new_window()
        .with_title("title")
        .with_dimensions(512, 512)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    Model {}

}

fn update(app: &App, model: &mut Model, update: Update) {
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
}

fn view(app: &App, model: &Model, frame: &Frame) {

    let draw = app.draw();
    draw.background().color(RED);
    draw.to_frame(app, &frame).unwrap();

}