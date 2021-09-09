use winit::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod camera;
mod chunk;
mod chunk_builder;
mod coordinate;
mod cube;
mod debug_info;
mod engine;
mod frustum_culling;
mod perlin_noise;
mod player;
mod quad;
mod renderer;
mod texture;
mod uniform;
mod world;

use crate::renderer::graphics::Graphics;
use engine::Engine;

struct Client {
    graphics: Graphics,
    engine: Engine,
    pool: uvth::ThreadPool,
}

impl Client {
    fn new(window: &winit::window::Window) -> Self {
        let graphics = Graphics::new(&window);
        let engine = Engine::new(&graphics);
        let pool = uvth::ThreadPoolBuilder::new()
            .name("Chunk Thread Pool".parse().unwrap())
            .build();
        Self {
            graphics,
            engine,
            pool,
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        //Updating
        self.engine.update(&self.graphics, &self.pool);

        //Rendering
        self.engine.render(&self.graphics)?;
        Ok(())
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.engine.resize(new_size, &mut self.graphics);
    }
}

fn main() {
    wgpu_subscriber::initialize_default_subscriber(None);
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    window.set_title("wgpu voxel engine");

    let mut client = Client::new(&window);
    let mut focus = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => quit(&mut client, control_flow),
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode,
                            ..
                        },
                    ..
                } => match virtual_keycode.unwrap() {
                    VirtualKeyCode::Escape => quit(&mut client, control_flow),
                    _ => (),
                },
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button,
                    ..
                } => match button {
                    MouseButton::Left => {
                        focus = true;
                        window.set_cursor_visible(false);
                        window.set_cursor_grab(true).unwrap();
                    }
                    MouseButton::Right => {
                        focus = false;
                        window.set_cursor_visible(true);
                        window.set_cursor_grab(false).unwrap();
                    }
                    _ => (),
                },
                WindowEvent::Resized(new_size) => client.resize(new_size),
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    client.resize(*new_inner_size)
                }
                _ => (),
            },
            Event::DeviceEvent { event, .. } if focus => {
                client.engine.input(&event);
                window
                    .set_cursor_position(winit::dpi::Position::Physical(
                        winit::dpi::PhysicalPosition::new(
                            (window.inner_size().width / 2) as i32,
                            (window.inner_size().height / 2) as i32,
                        ),
                    ))
                    .unwrap();
            }
            Event::MainEventsCleared => window.request_redraw(),

            Event::RedrawRequested(_) => {
                match client.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SurfaceError::Lost) => client.resize(client.graphics.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => (),
        }
    })
}

fn quit(client: &mut Client, control_flow: &mut ControlFlow) {
    client.pool.terminate();
    *control_flow = ControlFlow::Exit;
}
