use std::sync::Arc;

use anyhow::Result;
use winit::window::Window;

pub struct Surface {
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
}

impl Surface {
    pub fn new(window: &Arc<Window>, instance: &wgpu::Instance) -> Result<Self> {
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
                apply_limit_buckets: false,
            }))
            .unwrap();

        let size = window.inner_size();

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };

        Ok(Self {
            surface,
            config,
            adapter,
        })
    }
}
