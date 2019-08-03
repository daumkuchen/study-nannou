use nannou::prelude::*;

fn main() {
    nannou::sketch(view);
}

fn view(app: &App, frame: &Frame) {

    let win = app.window_rect();
    let t = app.time;
    // let t = app.mouse.x;
    let draw = app.draw();

    draw.background().color(BLACK);

    // let hz = map_range(app.mouse.x, win.left(), win.right(), 0.0, 100.0);
    let hz = 2.0;

    // let amp = app.mouse.y;
    let amp = 200.0;

    let tris = (1..win.w() as usize)
        .flat_map(|i| {

            let l_fract = (i - 1) as f32 / win.w();
            let r_fract = i as f32 / win.w();

            let l_x = map_range(l_fract, 0.0, 1.0, win.left(), win.right());
            let r_x = map_range(r_fract, 0.0, 1.0, win.left(), win.right());
            let l_y = (t * hz + l_fract * hz * TAU).sin() * amp;
            let r_y = (t * hz + r_fract * hz * TAU).sin() * amp;

            let a = pt2(l_x, l_y);
            let b = pt2(r_x, r_y);
            let c = pt2(r_x, 0.0);
            let d = pt2(l_x, 0.0);
            geom::Quad([a, b, c, d]).triangles_iter()

        })
        .map(|tri| {
            tri.map_vertices(|v| {

                // let y_fract = map_range(v.y.abs(), 0.0, win.top(), 0.0, 1.0);
                // let color = srgba(y_fract, 1.0 - y_fract, 1.0 - y_fract, 1.0);
                // geom::vertex::Srgba(v, color)

                let fract = map_range(v.y.abs(), 0.0, win.top(), 0.0, 1.0);
                let r = fract + 0.05;
                let g = fract + 0.05;
                let b = fract + 0.05;
                let rgba = srgba(r, g, b, 1.0);
                geom::vertex::Srgba(v, rgba)

            })
        });

    draw.mesh().tris(tris);

    draw.to_frame(app, &frame).unwrap();

}