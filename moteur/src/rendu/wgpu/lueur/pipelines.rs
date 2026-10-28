use crate::rendu::wgpu::textures::FORMAT_MASQUE;

pub struct PipelinesLueur {
    pub disposition: wgpu::BindGroupLayout,
    pub flou: wgpu::RenderPipeline,
    pub composer: wgpu::RenderPipeline,
}

fn entree_texture(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
        count: None,
    }
}

fn disposition(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let reglage = wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: wgpu::BufferSize::new(16) },
        count: None,
    };
    let echantillon = wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None };
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("disp_lueur"), entries: &[reglage, entree_texture(1), entree_texture(2), echantillon] })
}

fn pipeline(device: &wgpu::Device, layout: &wgpu::PipelineLayout, module: &wgpu::ShaderModule, entree: &str, format: wgpu::TextureFormat, melange: Option<wgpu::BlendState>) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(entree),
        layout: Some(layout),
        vertex: wgpu::VertexState { module, entry_point: Some("vs"), buffers: &[], compilation_options: Default::default() },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState { module, entry_point: Some(entree), targets: &[Some(wgpu::ColorTargetState { format, blend: melange, write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
        multiview: None,
        cache: None,
    })
}

impl PipelinesLueur {
    pub fn creer(device: &wgpu::Device, format_surface: wgpu::TextureFormat) -> Self {
        let disposition = disposition(device);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("layout_lueur"), bind_group_layouts: &[&disposition], push_constant_ranges: &[] });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader_lueur"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../../shaders/lueur.wgsl").into()),
        });
        let normal = wgpu::BlendState {
            color: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::SrcAlpha, dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha, operation: wgpu::BlendOperation::Add },
            alpha: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::Zero, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add },
        };
        let flou = pipeline(device, &layout, &module, "fs_flou", FORMAT_MASQUE, None);
        let composer = pipeline(device, &layout, &module, "fs_composer", format_surface, Some(normal));
        Self { disposition, flou, composer }
    }
}
