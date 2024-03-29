use nannou::prelude::*;
use std::cell::RefCell;
use std::sync::Arc;

fn main(){

    nannou::app(model)
        .update(update)
        .run();

}

struct Model{
    render_pass: Arc<dyn vk::RenderPassAbstract + Send + Sync>,
    pipeline: Arc<dyn vk::GraphicsPipelineAbstract + Send + Sync>,
    vertex_buffer: Arc<vk::CpuAccessibleBuffer<[Vertex]>>,
    view_fbo: RefCell<ViewFbo>,
    desciptor_set: Arc<dyn vk::DescriptorSet + Send + Sync>,
}

#[derive(Debug, Default, Clone)]
struct Vertex{
    position: [f32; 2],
}
vk::impl_vertex!(Vertex, position);

fn model(app: &App) -> Model{

    app.new_window()
        .with_title("title")
        .with_dimensions(512, 512)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let device = app.main_window().swapchain().device().clone();

    let vertex_buffer = vk::CpuAccessibleBuffer::from_iter(
        device.clone(),
        vk::BufferUsage::all(),
        [
            Vertex{
                position: [-1.0, -1.0],
            },
            Vertex{
                position: [-1.0, 1.0],
            },
            Vertex{
                position: [1.0, -1.0],
            },
            Vertex{
                position: [1.0, 1.0],
            },
        ]
        .iter()
        .cloned(),
    )
    .unwrap();

    let vertex_shader = vs::Shader::load(device.clone()).unwrap();
    let fragment_shader = fs::Shader::load(device.clone()).unwrap();

    let render_pass = Arc::new(
        vk::single_pass_renderpass!(
            device.clone(),
            attachments:{
                color:{
                    load: Clear,
                    store: Store,
                    format: nannou::frame::COLOR_FORMAT,
                    samples: app.main_window().msaa_samples(),
                }
            },
            pass:{
                color: [color],
                depth_stencil:{}
            }
        )
        .unwrap(),
    );

    let (texture, _tex_future) = {

        let logo_path = app.assets_path().unwrap().join("images").join("Nannou.png");
        let img_path = app.assets_path().unwrap().join("images").join("john.jpg");

        let image = image::open(logo_path).unwrap().to_rgba();
        let (width, height) = image.dimensions();
        let image_data = image.into_raw().clone();

        vk::ImmutableImage::from_iter(
            image_data.iter().cloned(),
            vk::image::Dimensions::Dim2d{ width, height },
            vk::Format::R8G8B8A8Srgb,
            app.main_window().swapchain_queue().clone(),
        )
        .unwrap()
    };

    let sampler = vk::SamplerBuilder::new().build(device.clone()).unwrap();

    let pipeline = Arc::new(
        vk::GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vertex_shader.main_entry_point(), ())
            .triangle_strip()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fragment_shader.main_entry_point(), ())
            .blend_alpha_blending()
            .render_pass(vk::Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );

    let desciptor_set = Arc::new(
        vk::PersistentDescriptorSet::start(pipeline.clone(), 0)
            .add_sampled_image(texture.clone(), sampler.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let view_fbo = RefCell::new(ViewFbo::default());

    Model{
        render_pass,
        pipeline,
        vertex_buffer,
        view_fbo,
        desciptor_set,
    }

}

fn event(app: &App, model: &mut Model, event: WindowEvent){}

fn view(app: &App, model: &Model, frame: &Frame){

    let win = app.window_rect();
    let draw = app.draw();

    //draw.background().color(BLACK);
    //draw.to_frame(app, &frame).unwrap();

    let [w, h] = frame.swapchain_image().dimensions();
    let viewport = vk::ViewportBuilder::new().build([w as _, h as _]);
    let dynamic_state = vk::DynamicState::default().viewports(vec![viewport]);

    model
        .view_fbo
        .borrow_mut()
        .update(frame, model.render_pass.clone(), |builder, image|{
            builder.add(image)
        })
        .unwrap();

    let clear_values = vec![[0.0, 1.0, 0.0, 1.0].into()];

    frame
        .add_commands()
        .begin_render_pass(model.view_fbo.borrow().expect_inner(), false, clear_values)
        .unwrap()
        .draw(
            model.pipeline.clone(),
            &dynamic_state,
            vec![model.vertex_buffer.clone()],
            model.desciptor_set.clone(),
            (),
        )
        .unwrap()
        .end_render_pass()
        .expect("failed to add `end_render_pass` command");

}

mod vs{
    nannou::vk::shaders::shader!{
    ty: "vertex",
    src: "
        #version 450

        layout(location = 0) in vec2 position;
        layout(location = 0) out vec2 tex_coords;

        void main(){
            gl_Position = vec4(position, 0.0, 1.0);
            tex_coords = position + vec2(0.5);
        }
        "
    }
}

mod fs{
    nannou::vk::shaders::shader!{
    ty: "fragment",
    src: "
        #version 450

        layout(location = 0) in vec2 tex_coords;
        layout(location = 0) out vec4 f_color;
        layout(set = 0, binding = 0) uniform sampler2D tex;

        void main(){
            f_color = texture(tex, tex_coords);
        }
        "
    }
}

fn update(app: &App, model: &mut Model, update: Update){
}