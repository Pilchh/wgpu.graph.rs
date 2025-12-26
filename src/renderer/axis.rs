use wgpu::{
    Buffer, BufferUsages, Device, RenderPass, RenderPipeline, ShaderModuleDescriptor, ShaderSource,
    TextureFormat,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::renderer::{pipeline::create_pipeline, vertex::Vertex2D};

pub struct AxisRenderer {
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
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

        // Create pipeline
        let pipeline = create_pipeline(
            device,
            &shader,
            &[Vertex2D::desc()],
            format,
            wgpu::PrimitiveTopology::TriangleStrip,
            "Axis Pipeline",
        );

        Self {
            pipeline,
            vertex_buffer,
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
