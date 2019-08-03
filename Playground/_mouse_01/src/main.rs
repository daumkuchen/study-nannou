use nannou::prelude::*;

fn main() {

    nannou::app(model)
        .update(update)
        .run();

}

struct Model {
    time: f32,
    mouse: Mouse,
    circle: Circle,
    speed: Speed
}

struct Mouse {
    x: f32,
    y: f32,
}

struct Circle {
    x: f32,
    y: f32,
}

struct Speed {
    x: f32,
    y: f32,
}

const ease: f32 = 0.1;

// const gravity: f32 = 5.0;
const mass: f32 = 4.0;
const stiffness: f32 = 1.0;
const damping: f32 = 0.8;

fn model(app: &App) -> Model {

    app.new_window()
        .with_title("title")
        .with_dimensions(512, 512)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let time = 1.0;
    let mouse = Mouse {
        x: 0.0,
        y: 0.0
    };
    let circle = Circle {
        x: 0.0,
        y: 0.0
    };
    let speed = Speed {
        x: 0.0,
        y: 0.0
    };

    Model {
        time,
        mouse,
        circle,
        speed
    }

}

fn updateEase(model: &mut Model){
    model.circle.x += (model.mouse.x - model.circle.x) * ease;
    model.circle.y += (model.mouse.y - model.circle.y) * ease;
}

fn updateSpring(model: &mut Model){
    let force_x = (model.mouse.x - model.circle.x) * stiffness;
    let force_y = (model.mouse.y - model.circle.y) * stiffness;
    let ax = force_x / mass;
    let ay = force_y / mass;
    model.speed.x = damping * (model.speed.x + ax);
    model.speed.y = damping * (model.speed.y + ay);
    model.circle.x += model.speed.x;
    model.circle.y += model.speed.y;
}

fn update(app: &App, model: &mut Model, update: Update) {

    model.time += 1.0;
    // println!("update::time: {}", model.time);

    // updateEase(model);
    updateSpring(model);
    
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {

    match event {
        MouseMoved(pos) => {
            model.mouse.x = pos.x;
            model.mouse.y = pos.y;
            // println!("x: {}, y: {}", pos.x, pos.y);
        }
        _other => ()
    }

}

fn view(app: &App, model: &Model, frame: &Frame) {

    let draw = app.draw();
    draw.background().color(BLACK);

    draw.ellipse()
        .xy(pt2(model.circle.x, model.circle.y))
        .radius(50.0)
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();

}