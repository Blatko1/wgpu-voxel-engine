use futures::executor::block_on;
use winit::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod camera;
mod chunk;
mod chunk_generator;
mod coordinate;
mod cube;
mod engine;
mod player;
mod quad;
mod region;
mod renderer;
mod terrain_generator;
mod texture;
mod uniform;
mod world;

use crate::renderer::graphics::Graphics;
use engine::Engine;
use std::sync::Arc;
use winit::dpi::PhysicalSize;

struct Client {
    graphics: Graphics,
    engine: Engine,
    pool: rayon::ThreadPool,
}

impl Client {
    fn new(window: &winit::window::Window) -> Self {
        let mut graphics = block_on(Graphics::new(&window));
        let engine = Engine::new(&graphics);
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();
        Self {
            graphics,
            engine,
            pool,
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
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
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode,
                            ..
                        },
                    ..
                } => match virtual_keycode.unwrap() {
                    VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
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
                window.set_cursor_position(winit::dpi::Position::Physical(
                    winit::dpi::PhysicalPosition::new(
                        (window.inner_size().width / 2) as i32,
                        (window.inner_size().height / 2) as i32,
                    ),
                ));
            }
            Event::MainEventsCleared => window.request_redraw(),

            Event::RedrawRequested(_) => {
                match client.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => client.resize(client.graphics.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => (),
        }
    })
}
