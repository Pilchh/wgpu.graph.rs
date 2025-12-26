use wgpu::{
    BlendState, Buffer, BufferUsages, ColorTargetState, ColorWrites, Device, FragmentState,
    MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState,
    RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource,
    TextureFormat, VertexState,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::renderer::vertex::Vertex2D;

pub struct AxisRenderer {
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    vertex_count: u32,
}

impl AxisRenderer {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        // Thickness in clip-space units
        let thickness = 0.002;

        // Create vertices
        let vertices = [
            // Horizontal Axis
            Vertex2D {
                pos: [-1.0, thickness],
            },
            Vertex2D {
                pos: [-1.0, -thickness],
            },
            Vertex2D {
                pos: [1.0, thickness],
            },
            Vertex2D {
                pos: [1.0, -thickness],
            },
            // Vertical Axis
            Vertex2D {
                pos: [thickness, 1.0],
            },
            Vertex2D {
                pos: [-thickness, 1.0],
            },
            Vertex2D {
                pos: [thickness, -1.0],
            },
            Vertex2D {
                pos: [-thickness, -1.0],
            },
        ];

        // Create vertex buffer
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Axis Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        // Load shader
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Axis Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/axis.wgsl").into()),
        });

        // Create pipeline layout
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Axis Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        // Create pipeline
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Axis Pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex2D::desc()],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            vertex_buffer,
            vertex_count: vertices.len() as u32,
        }
    }

    pub fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        // Horizontal
        pass.draw(0..4, 0..1);

        // Vertical
        pass.draw(4..8, 0..1);
    }
}
