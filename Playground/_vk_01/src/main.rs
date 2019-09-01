use nannou::prelude::*;
use std::cell::RefCell;
use std::sync::Arc;

fn main() {

    nannou::app(model)
        .update(update)
        .run();

}

struct Model {
    render_pass: Arc<dyn vk::RenderPassAbstract + Send + Sync>,
    pipeline: Arc<dyn vk::GraphicsPipelineAbstract + Send + Sync>,
    vertex_buffer: Arc<vk::CpuAccessibleBuffer<[Vertex]>>,
    view_fbo: RefCell<ViewFbo>,
}

#[derive(Debug, Default, Clone)]
struct Vertex {
    position: [f32; 2],
    // color: [f32; 2],
}

vk::impl_vertex!(Vertex, position);

fn model(app: &App) -> Model {

    app.new_window()
        .with_title("title")
        .with_dimensions(512, 512)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let device = app.main_window().swapchain().device().clone();

    let vertex_buffer = {
        let positions = [
            [ 0.0, -0.5],
            [-0.5,  0.5],
            [ 0.5,  0.5]
        ];
        let color = [
            [1.0, 1.0],
            [1.0, 1.0],
            [1.0, 1.0]
        ];
        let vertices = positions.iter().map(|&position| Vertex { position });
        vk::CpuAccessibleBuffer::from_iter(device.clone(), vk::BufferUsage::all(), vertices)
            .unwrap()
    };

    let vertex_shader = vs::Shader::load(device.clone()).unwrap();
    let fragment_shader = fs::Shader::load(device.clone()).unwrap();

    let render_pass = Arc::new(
        vk::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: nannou::frame::COLOR_FORMAT,
                    samples: app.main_window().msaa_samples(),
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap(),
    );

    let pipeline = Arc::new(
        vk::GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vertex_shader.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fragment_shader.main_entry_point(), ())
            .render_pass(vk::Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );

    let view_fbo = RefCell::new(ViewFbo::default());

    Model {
        render_pass,
        pipeline,
        vertex_buffer,
        view_fbo,
    }

}

fn view(app: &App, model: &Model, frame: &Frame) {

    let win = app.window_rect();
    let draw = app.draw();

    let [w, h] = frame.swapchain_image().dimensions();
    let viewport = vk::ViewportBuilder::new().build([w as _, h as _]);
    let dynamic_state = vk::DynamicState::default().viewports(vec![viewport]);

    model
        .view_fbo
        .borrow_mut()
        .update(frame, model.render_pass.clone(), |builder, image| {
            builder.add(image)
        })
        .unwrap();

    let clear_values = vec![[0.0, 0.0, 0.0, 1.0].into()];

    frame
        .add_commands()
        .begin_render_pass(model.view_fbo.borrow().expect_inner(), false, clear_values)
        .unwrap()
        .draw(
            model.pipeline.clone(),
            &dynamic_state,
            vec![model.vertex_buffer.clone()],
            (),
            (),
        )
        .unwrap()
        .end_render_pass()
        .expect("failed to add `end_render_pass` command");

}

mod vs {
    nannou::vk::shaders::shader! {
    ty: "vertex",
    src: "
        #version 450

        layout(location = 0) in vec2 position;
        layout(location = 0) out vec3 color;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            color = vec3(position, 1.0);
        }
        "
    }
}

mod fs {
    nannou::vk::shaders::shader! {
    ty: "fragment",
    src: "
        #version 450

        layout(location = 0) in vec3 color;
        layout(location = 0) out vec4 f_color;

        void main() {
            vec3 albedo = vec3(1.0);
            f_color = vec4(color, 1.0);
        }
        "
    }
}

fn update(app: &App, model: &mut Model, update: Update){
}

fn event(app: &App, model: &mut Model, event: WindowEvent){
}