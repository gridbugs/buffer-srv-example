#[macro_use] extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;

use glutin::{Event, WindowEvent, GlContext};
use gfx::traits::FactoryExt;
use gfx::Device;

type ColourFormat = gfx::format::Srgba8;
type DepthFormat = gfx::format::DepthStencil;


const QUAD_VERTICES: [[f32; 2]; 4] = [[-1.0, 1.0],
                                      [-1.0, -1.0],
                                      [1.0, -1.0],
                                      [1.0, 1.0]];

pub const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

gfx_vertex_struct!( Vertex {
    pos: [f32; 2] = "a_Pos",
});

gfx_pipeline!( pipe {
    vertex: gfx::VertexBuffer<Vertex> = (),
    out_colour: gfx::BlendTarget<ColourFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
});

fn main() {
    let builder = glutin::WindowBuilder::new();

    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new();

    let (window, mut device, mut factory, rtv, _dsv) =
        gfx_window_glutin::init::<ColourFormat, DepthFormat>(builder, context, &events_loop);

    let mut encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shdr.vert"),
        include_bytes!("shdr.frag"),
        pipe::new()).expect("Failed to create pipeline");

    let vertex_data: Vec<Vertex> = QUAD_VERTICES.iter()
        .map(|v| {
            Vertex {
                pos: *v,
            }
        }).collect();

    let (vertex_buffer, slice) =
        factory.create_vertex_buffer_with_slice(
            &vertex_data,
            &QUAD_INDICES[..]);

    let data = pipe::Data {
        vertex: vertex_buffer,
        out_colour: rtv,
    };

    let bundle = gfx::pso::bundle::Bundle::new(slice, pso, data);

    let mut running = true;
    loop {
        events_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                if let WindowEvent::Closed = event {
                    running = false;
                }
            }

            encoder.clear(&bundle.data.out_colour, [0.0, 0.0, 0.0, 1.0]);
            bundle.encode(&mut encoder);
            encoder.flush(&mut device);
            window.swap_buffers().unwrap();
            device.cleanup();
        });

        if !running {
            break;
        }
    }
}
