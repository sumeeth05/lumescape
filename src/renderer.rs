use std::sync::Arc;

use anyhow::{Ok, Result};
use wgpu::{Color, InstanceDescriptor};
use winit::{dpi::PhysicalSize, window::Window};

use crate::renderer::{pipelines::Pipelines, surface::Surface};

mod pipelines;
mod surface;

pub struct Renderer {
    surface: Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipelines: Pipelines,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Result<Self> {
        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: wgpu::Backends::all(),
            backend_options: Default::default(),
            flags: Default::default(),
            display: None,
            memory_budget_thresholds: Default::default(),
        });

        let surface = Surface::new(&window, &instance)?;

        let (device, queue) = pollster::block_on(
            surface
                .adapter
                .request_device(&wgpu::DeviceDescriptor::default()),
        )
        .unwrap();

        let pipelines = Pipelines::new(&device, &surface.config)?;

        surface.surface.configure(&device, &surface.config);

        Ok(Self {
            surface,
            device,
            queue,
            pipelines,
        })
    }

    pub fn resize(&mut self, window: Arc<Window>) {
        let size: PhysicalSize<u32> = window.inner_size();
        self.surface.config.width = size.width;
        self.surface.config.height = size.height;

        self.surface
            .surface
            .configure(&self.device, &self.surface.config);
    }

    pub fn draw(&mut self, window: Arc<Window>) {
        let output = match self.surface.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture)
            | wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture,

            wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Outdated => {
                self.resize(window);
                return;
            }

            _ => {
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.pipelines.graphics);
            render_pass.set_vertex_buffer(0, self.pipelines.vertex_buffer.slice(..));
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(output);
    }
}
