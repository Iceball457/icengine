use common::error::EngineError;
use std::sync::Arc;

pub mod scene;
pub use scene::*;

pub struct EngineCtl<'frame> {
    render_server: &'frame mut render::Server<'static>,
}

pub struct Engine {
    window: Option<Arc<winit::window::Window>>,
    render_server: Option<render::Server<'static>>,
    errors: Vec<EngineError>,
}

impl Engine {
    fn new() -> Self {
        Self {
            window: None,
            render_server: None,
            errors: vec![],
        }
    }
}

impl winit::application::ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = winit::window::WindowAttributes::default().with_title("Icengine");
        let window = match event_loop.create_window(window_attributes) {
            Ok(window) => window,
            Err(error) => {
                self.errors.push(EngineError::new_fatal(error.into()));
                return;
            }
        };
        let window = Arc::new(window);
        let size = window.inner_size();
        let size = common::texture::FixedSize::new(size.width, size.height);
        self.render_server =
            match pollster::block_on(render::Server::new(Arc::clone(&window), size)) {
                Ok(rs) => Some(rs),
                Err(error) => {
                    self.errors.push(EngineError::new_fatal(error));
                    return;
                }
            };
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        for error in self.errors.drain(..) {
            println!("{error}");
            if error.fatal() {
                event_loop.exit();
                return;
            }
        }
        // SAFETY: If render_server was none, we already bailed
        let render_server = unsafe { self.render_server.as_mut().unwrap_unchecked() };
        match event {
            winit::event::WindowEvent::Resized(physical_size) => (),
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key,
                        state,
                        repeat,
                        ..
                    },
                ..
            } => {
                if physical_key == winit::keyboard::KeyCode::Escape
                    && state == winit::event::ElementState::Pressed
                    && !repeat
                {
                    event_loop.exit();
                }
            }
            winit::event::WindowEvent::CursorMoved {
                device_id,
                position,
            } => (),
            winit::event::WindowEvent::CursorEntered { device_id } => (),
            winit::event::WindowEvent::CursorLeft { device_id } => (),
            winit::event::WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => (),
            winit::event::WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => (),
            winit::event::WindowEvent::RedrawRequested => match render_server.render() {
                Ok(()) => (),
                Err(error) => self.errors.push(EngineError::new(error)),
            },
            _ => (),
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.run_app(&mut Engine::new())?;
    Ok(())
}
