use anyhow::{Ok, Result};
use winit::window::Window;

pub struct Renderer {}

impl Renderer {
    pub fn new(_window: &Window) -> Result<Self> {
        Ok(Self {})
    }

    pub fn draw(&mut self) {}

    pub fn resize(&mut self, _window: &Window) {}
}
