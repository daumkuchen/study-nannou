use nannou::prelude::*;

fn main() {
    nannou::sketch(view);
}

fn view(app: &App, frame: &Frame) {

    let win = app.window_rect();
    let draw = app.draw();

    draw.background().color(BLACK);

    let tris = (1..win.w() as usize)
        .flat_map(|i|{
            let v1 = pt2(-100., -100.);
            let v2 = pt2( 100., -100.);
            let v3 = pt2( 100.,  100.);
            let v4 = pt2(-100.,  100.);
            geom::Quad([v1, v2, v3, v4]).triangles_iter()
        })
        .map(|tri| {
            tri.map_vertices(|v|{
                let r = 1.0;
                let g = 1.0;
                let b = 1.0;
                let rgba = srgba(r, g, b, 1.0);
                geom::vertex::Srgba(v, rgba)
            })
        });

    draw.mesh().tris(tris);

    draw.to_frame(app, &frame).unwrap();

}