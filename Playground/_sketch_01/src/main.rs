use nannou::prelude::*;

fn main() {

    nannou::app(model)
        .update(update)
        .run();

}

struct Model {
    time: f32
}

fn model(app: &App) -> Model {

    app.new_window()
        .with_title("title")
        .with_dimensions(512, 512)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let time = 1.0;

    Model {
        time
    }

}

fn update(app: &App, model: &mut Model, update: Update){

    model.time += 1.0;

}

fn event(app: &App, model: &mut Model, event: WindowEvent){}

fn view(app: &App, model: &Model, frame: &Frame){

    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);

    for i in 0..10 {

        let f_i = i as f32;
        
        let color = match i % 5 {
            0 => RED,
            1 => GREEN,
            2 => BLUE,
            3 => PURPLE,
            4 => ORANGE,
            _ => unreachable!(),
        };

        let x = f_i * 40.0 - (4.0 * 40.0);
        let y = 0.0;
        let xy = pt2(x, y);
        let radius = f_i * 5.0 + 5.0;

        draw.ellipse()
            .xy(xy)
            .radius(radius)
            .color(color);

    }

    draw.to_frame(app, &frame).unwrap();

}