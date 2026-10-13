use anyhow::{Ok, Result};
use wgpu::{SurfaceConfiguration, util::DeviceExt};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5],
        color: [0.0, 0.0, 1.0],
    },
];
pub struct Pipelines {
    pub graphics: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
}

impl Pipelines {
    pub fn new(device: &wgpu::Device, config: &SurfaceConfiguration) -> Result<Self> {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("init buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let graphics_pipeline = Self::create_graphics_pipeline(device, config)?;

        Ok(Self {
            graphics: graphics_pipeline,
            vertex_buffer,
        })
    }

    fn create_graphics_pipeline(
        device: &wgpu::Device,
        config: &SurfaceConfiguration,
    ) -> Result<wgpu::RenderPipeline> {
        let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(include_str!(".././shaders/vert.glsl")),
                stage: wgpu::naga::ShaderStage::Vertex,
                defines: &[],
            },
        });

        let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Glsl {
                shader: std::borrow::Cow::Borrowed(include_str!(".././shaders/frag.glsl")),
                stage: wgpu::naga::ShaderStage::Fragment,
                defines: &[],
            },
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                immediate_size: 0,
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: Some("main"),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![
                        0 => Float32x2,
                        1 => Float32x3
                    ],
                })],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: Some("main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            cache: None,
            multiview_mask: None,
        });

        Ok(render_pipeline)
    }
}
