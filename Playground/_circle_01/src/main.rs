use nannou::prelude::*;

fn main() {

    nannou::app(model)
        .update(update)
        .run();

}

struct Model {
    time: f32
}

fn model(_app: &App) -> Model {

    _app.new_window()
        .with_title("title")
        .with_dimensions(512, 512)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let time = 0.0;

    Model {
        time
    }

}

fn update(_app: &App, _model: &mut Model, _update: Update){

    _model.time += 0.01;

}

fn event(_app: &App, _model: &mut Model, _event: WindowEvent){
}

fn view(_app: &App, _model: &Model, _frame: &Frame){

    let _win = _app.window_rect();
    let draw = _app.draw();

    draw.background().color(hsv(_model.time.sin(), 1.0, 1.0));

    for i in 0..32 {
        let f_i = i as f32;
        draw.ellipse()
            .xy(pt2(0.0, 0.0))
            .w_h(_win.w() / f_i, _win.w() / f_i)
            .hsv(_model.time.cos() * f_i, 1.0, 1.0);
    }
        
    draw.to_frame(_app, &_frame).unwrap();

}


// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 8-1: Simple Recursion
// use nannou::prelude::*;

// fn main() {
//     nannou::app(model).run();
// }

// struct Model;

// fn model(app: &App) -> Model {
//     app.set_loop_mode(LoopMode::loop_once());
//     let _window = app
//         .new_window()
//         .with_dimensions(640, 360)
//         .view(view)
//         .build()
//         .unwrap();
//     Model
// }

// fn view(app: &App, _model: &Model, frame: &Frame) {
//     // Begin drawing
//     let draw = app.draw();
//     draw.background().color(WHITE);

//     draw_circle(&draw, 0.0, 0.0, app.window_rect().w());

//     // Write the result of our drawing to the window's frame.
//     draw.to_frame(app, &frame).unwrap();
// }

// fn draw_circle(draw: &app::Draw, x: f32, y: f32, mut r: f32) {
//     draw.ellipse()
//         .x_y(x, y)
//         .radius(r)
//         .hsv(map_range(r, 2.0, 360.0, 0.0, 1.0), 0.75, 1.0);
//     // Exit condition, stop when radius is too small
//     if r > 2.0 {
//         r *= 0.75;
//         // Call the function insie the function! (recursion!)
//         draw_circle(&draw, x, y, r);
//     }
// }


// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 8-2: Simple Recursion
// use nannou::prelude::*;

// fn main() {
//     nannou::app(model).run();;
// }

// struct Model;

// fn model(app: &App) -> Model {
//     app.set_loop_mode(LoopMode::loop_once());
//     let _window = app
//         .new_window()
//         .with_dimensions(640, 360)
//         .view(view)
//         .build()
//         .unwrap();
//     Model
// }

// fn view(app: &App, _model: &Model, frame: &Frame) {
//     // Begin drawing
//     let draw = app.draw();
//     draw.background().color(WHITE);

//     draw_circle(&draw, 0.0, 0.0, 400.0);

//     // Write the result of our drawing to the window's frame.
//     draw.to_frame(app, &frame).unwrap();
// }

// // Recursive function
// fn draw_circle(draw: &app::Draw, x: f32, y: f32, r: f32) {
//     draw.ellipse()
//         .x_y(x, y)
//         .radius(r)
//         .hsv(map_range(r, 2.0, 360.0, 0.0, 1.0), 0.75, 1.0);

//     if r > 2.0 {
//         // Now we draw two more circles, one to the left
//         // and one to the right
//         draw_circle(&draw, x + r / 2.0, y, r / 2.0);
//         draw_circle(&draw, x - r / 2.0, y, r / 2.0);
//     }
// }


// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 8-3: Simple Recursion
// use nannou::prelude::*;

// fn main() {
//     nannou::app(model).run();
// }

// struct Model;

// fn model(app: &App) -> Model {
//     app.set_loop_mode(LoopMode::loop_once());
//     let _window = app
//         .new_window()
//         .with_dimensions(640, 360)
//         .view(view)
//         .build()
//         .unwrap();
//     Model
// }

// fn view(app: &App, _model: &Model, frame: &Frame) {
//     // Begin drawing
//     let draw = app.draw();
//     draw.background().color(WHITE);

//     draw_circle(&draw, 0.0, 0.0, 400.0);

//     // Write the result of our drawing to the window's frame.
//     draw.to_frame(app, &frame).unwrap();
// }

// // Recursive function
// fn draw_circle(draw: &app::Draw, x: f32, y: f32, r: f32) {
//     draw.ellipse()
//         .x_y(x, y)
//         .radius(r)
//         .hsv(map_range(r, 2.0, 360.0, 0.0, 1.0), 0.75, 1.0);

//     if r > 8.0 {
//         // Four circles! left right, up and down
//         draw_circle(&draw, x + r / 2.0, y, r / 2.0);
//         draw_circle(&draw, x - r / 2.0, y, r / 2.0);
//         draw_circle(&draw, x, y + r / 2.0, r / 2.0);
//         draw_circle(&draw, x, y - r / 2.0, r / 2.0);
//     }
// }
