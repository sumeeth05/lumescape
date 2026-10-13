#![windows_subsystem = "windows"]
#![allow(dead_code)]

use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, Size::Logical},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow::Poll, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::renderer::Renderer;

mod renderer;

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attr = WindowAttributes::default();
        attr.title = "Lumescape".to_string();
        attr.min_inner_size = Some(Logical(LogicalSize::new(1280.0, 720.0)));
        attr.visible = false;

        let window = Arc::new(event_loop.create_window(attr).unwrap());

        self.renderer = Some(Renderer::new(window.clone()).unwrap());

        window.set_visible(true);
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let Some(window) = self.window.as_ref() {
                    let renderer = self.renderer.as_mut().unwrap();

                    let size = window.inner_size();

                    if size.width > 0 && size.height > 0 {
                        renderer.draw(window.clone());
                    }
                }

                self.window.as_ref().unwrap().request_redraw();
            }

            WindowEvent::Resized(_) => {
                if let (Some(renderer), Some(window)) =
                    (self.renderer.as_mut(), self.window.as_ref())
                {
                    let size = window.inner_size();

                    if size.width == 0 || size.height == 0 {
                        return;
                    };

                    renderer.resize(window.clone());
                }
            }
            _ => {}
        }
    }
}

fn main() {
    env_logger::init();

    let mut app = App::default();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(Poll);
    event_loop.run_app(&mut app).unwrap();
}
