use winit::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use futures::executor::block_on;

mod graphics;
mod engine;

use graphics::Graphics;
use engine::Engine;

struct Client {
    graphics: Graphics,

    engine: Engine,
}

impl Client {
    fn new(window: &winit::window::Window) -> Self {
        let graphics = block_on( Graphics::new(&window) );
        let engine = Engine::new(&graphics);
        Self {
            graphics,
            engine
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    window.set_title("wgpu voxel engine");

    let client = Client::new(&window);
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
                WindowEvent::Resized(_) => {}
                WindowEvent::ScaleFactorChanged { .. } => {}
                _ => (),
            },
            Event::DeviceEvent { event, .. } if focus => (),
            Event::MainEventsCleared => window.request_redraw(),

            Event::RedrawRequested(_) => {}
            _ => (),
        }
    })
}
