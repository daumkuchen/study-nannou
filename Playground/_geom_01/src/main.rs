use nannou::prelude::*;
use nannou::color::Rgb;
use nannou::color::Srgb;

fn main() {

    nannou::app(model)
        .update(update)
        .run();

}

struct Model {
    loop_num: i32,
    colors: Vec<Rgb>
}

fn model(_app: &App) -> Model {

    // _app.set_loop_mode(LoopMode::loop_once());
    _app.set_loop_mode(LoopMode::rate_fps(2.0));

    _app.new_window()
        .with_title("title")
        .with_dimensions(512, 512)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let loop_num = 32;

    // let hex: [u32; 11] = [0xFFFFFFFF, 0xFFFE0002, 0xFFBE0048, 0xFFC52D9C, 0xFF550068, 0xFF404042, 0xFF77787C, 0xFFFEF301, 0xFFDFB434, 0xFF009AE2, 0xFF00C4C3];
    // let hex_to_vec = hex.to_vec();
    // let colors: Vec<Rgb> = hex_to_vec.into_iter().map(|c| {  
    //     let blue: u8 = (c & 0xFF) as u8;    
    //     let green: u8 = (( c >> 8 ) & 0xFF) as u8;
    //     let red: u8 = (( c >> 16 ) & 0xFF) as u8;
    //     // let c = Srgb::new_u8(red, green, blue);
    //     let c = Srgb::new(red as f32, green as f32, blue as f32);
    //     c.into()
    // }).collect();

    let colors = vec! [
        rgb(222.0 / 255.0,  52.0 / 255.0,  90.0 / 255.0),
        rgb(222.0 / 255.0, 203.0 / 255.0, 183.0 / 255.0),
        rgb( 42.0 / 255.0, 134.0 / 255.0, 123.0 / 255.0),
        rgb( 89.0 / 255.0, 167.0 / 255.0, 117.0 / 255.0),
        rgb(119.0 / 255.0, 178.0 / 255.0, 140.0 / 255.0),
        rgb(0.0, 0.0, 0.0),
    ];
   
    Model {
        loop_num,
        colors
    }

}

fn update(_app: &App, _model: &mut Model, _update: Update){

    _model.loop_num = random_range(1, 4) * random_range(2, 8);

}

fn event(_app: &App, _model: &mut Model, _event: WindowEvent){
}

fn view(_app: &App, _model: &Model, _frame: &Frame){

    let _win = _app.window_rect();
    let draw = _app.draw();

    draw.background().color(rgba(0.0, 0.0, 0.0, 1.0));

    for _x in 0.._model.loop_num{
        for _y in 0.._model.loop_num{

            let radius = _win.w() / _model.loop_num as f32;
            let color = _model.colors[random_range(0, 6)];

            let n_x = _x as f32 * radius - _win.w() / 2.0 + radius / 2.0;
            let n_y = _y as f32 * radius - _win.h() / 2.0 + radius / 2.0;

            let shape_num = random_range(0, 3);
            if(shape_num == 0){

                draw.ellipse()
                    .xy(pt2(n_x, n_y))
                    .radius(radius / 2.0)
                    .color(color);

            }
            else if(shape_num == 1){

                draw.quad()
                    .xy(pt2(n_x, n_y))
                    .w(radius)
                    .h(radius)
                    .color(color);

            }
            else if(shape_num == 2){

                draw.tri()
                    .rotate(PI / random_range(1, 3) as f32)
                    .xy(pt2(n_x, n_y))
                    .points([-radius / 2.0, -radius / 2.0],
                        [ radius / 2.0, radius / 2.0],
                        [-radius / 2.0, radius / 2.0])
                    .color(color);

            }
        }
    }

    draw.to_frame(_app, &_frame).unwrap();

}