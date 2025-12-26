use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Device, FragmentState, MultisampleState,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology,
    RenderPipeline, RenderPipelineDescriptor, ShaderModule, TextureFormat, VertexBufferLayout,
    VertexState,
};

pub fn create_pipeline(
    device: &Device,
    shader: &ShaderModule,
    vertex_layouts: &[VertexBufferLayout],
    format: TextureFormat,
    topology: PrimitiveTopology,
    label: &str,
) -> RenderPipeline {
    // Create pipeline layout
    let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some(&format!("{label} Layout")),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&RenderPipelineDescriptor {
        // Create pipeline
        label: Some(label),
        layout: Some(&layout),
        vertex: VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: vertex_layouts,
            compilation_options: PipelineCompilationOptions::default(),
        },
        fragment: Some(FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            targets: &[Some(ColorTargetState {
                format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            })],
            compilation_options: PipelineCompilationOptions::default(),
        }),
        primitive: PrimitiveState {
            topology,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}
