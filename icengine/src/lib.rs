use common::error::Error;
use std::sync::Arc;

mod scene;
pub use scene::*;

mod command;
pub use command::*;

pub struct EngineCtl<'a> {
    render_server: &'a mut render::Server,
}

impl EngineCtl<'_> {
    pub const fn render_server_mut(&mut self) -> &mut render::Server {
        self.render_server
    }
}

pub struct Engine {
    window: Option<Arc<winit::window::Window>>,
    active_scene: Box<dyn Scene>,
    render_server: Option<render::Server>,
    commands: Vec<Command>,
    errors: Vec<Error>,
}

impl Engine {
    fn new(active_scene: Box<dyn Scene>) -> Self {
        Self {
            window: None,
            active_scene,
            render_server: None,
            commands: vec![],
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
                self.errors.push(Error::new_fatal(error.into()));
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
                    self.errors.push(Error::new_fatal(error));
                    return;
                }
            };
        self.window = Some(window);
        self.active_scene.start(EngineCtl {
            render_server: self.render_server.as_mut().unwrap(),
        });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        for error in self.errors.drain(..) {
            println!("{error}");
            if error.fatal() {
                event_loop.exit();
                return;
            }
        }
        // SAFETY: If any of these resources are none, we already bailed
        let render_server = unsafe { self.render_server.as_mut().unwrap_unchecked() };
        let window = unsafe { self.window.as_mut().unwrap_unchecked() };
        match event {
            winit::event::WindowEvent::Resized(physical_size) => {
                render_server.resize(common::texture::FixedSize::new(
                    physical_size.width,
                    physical_size.height,
                ));
            }
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
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
            winit::event::WindowEvent::RedrawRequested => {
                // respond to all engine commands
                for command in self.commands.drain(..) {
                    match command {
                        Command::Exit => {
                            event_loop.exit();
                            return;
                        }
                        Command::SetScene(scene) => {
                            self.active_scene = scene;
                            self.active_scene.start(EngineCtl { render_server });
                        }
                    }
                }
                // check time, tick if enough time has elapsed

                // display

                // render
                match render_server.render() {
                    Ok(()) => (),
                    Err(error) => self.errors.push(Error::new(error)),
                }
                // do it again
                window.request_redraw();
            }
            _ => (),
        }
    }
}

/// # Errors
///
/// During setup, can error while gathering devices like GPUs.
/// During the core loop, errors only when the engine ancounters a fatal error.
/// This can be because a device was lost, or because the user's code submitted an error marked fatal.
pub fn run(initial_scene: Box<dyn Scene>) -> anyhow::Result<()> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.run_app(&mut Engine::new(initial_scene))?;
    Ok(())
}
