use super::tampons::{TAILLE_GLOBALES, TAILLE_LUMIERE, Tampons};
use super::textures::{FORMAT_MASQUE, FORMAT_OMBRE, FORMAT_PROFONDEUR, TexturesWgpu};
use crate::rendu::{Etape, FLOTTANTS_PAR_INSTANCE};

pub struct Pipelines {
    pub disposition_carte: wgpu::BindGroupLayout,
    pub disposition_ombre: wgpu::BindGroupLayout,
    pub pipelines_carte: [wgpu::RenderPipeline; 3],
    pub pipeline_ombre: wgpu::RenderPipeline,
    pub groupe_carte: wgpu::BindGroup,
    pub groupe_ombre: wgpu::BindGroup,
}

impl Pipelines {
    pub fn creer(device: &wgpu::Device, format_surface: wgpu::TextureFormat, tampons: &Tampons, textures: &TexturesWgpu) -> Self {
        let (disposition_carte, disposition_ombre) = Self::creer_dispositions(device);
        let layout_carte = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("layout_carte"),
            bind_group_layouts: &[&disposition_carte],
            push_constant_ranges: &[],
        });
        let layout_ombre = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("layout_ombre"),
            bind_group_layouts: &[&disposition_ombre],
            push_constant_ranges: &[],
        });
        let module_carte = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader_carte"),
            source: wgpu::ShaderSource::Wgsl(concat!(include_str!("../../../shaders/bloc.wgsl"), include_str!("../../../shaders/carte.wgsl")).into()),
        });
        let module_ombre = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader_ombre"),
            source: wgpu::ShaderSource::Wgsl(concat!(include_str!("../../../shaders/bloc.wgsl"), include_str!("../../../shaders/ombre_profondeur.wgsl")).into()),
        });
        let pipeline_ombre = Self::creer_pipeline_ombre(device, &layout_ombre, &module_ombre);
        let etapes = [Etape::Scene, Etape::Voile, Etape::Selection];
        let pipelines_carte = etapes.map(|e| Self::creer_pipeline_carte(device, &layout_carte, format_surface, &module_carte, e));
        let groupe_carte = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("groupe_carte"),
            layout: &disposition_carte,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: tampons.globales.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&textures.vue_tableau) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&textures.echantillon) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&textures.vue_ombre) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&textures.echantillon_ombre) },
            ],
        });
        let groupe_ombre = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("groupe_ombre"),
            layout: &disposition_ombre,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: tampons.lumiere.as_entire_binding() }],
        });
        Self { disposition_carte, disposition_ombre, pipelines_carte, pipeline_ombre, groupe_carte, groupe_ombre }
    }

    fn creer_dispositions(device: &wgpu::Device) -> (wgpu::BindGroupLayout, wgpu::BindGroupLayout) {
        let dc = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("disp_carte"),
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: wgpu::BufferSize::new(TAILLE_GLOBALES) }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2Array, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
                wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Depth, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 4, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison), count: None },
            ],
        });
        let dombre = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("disp_ombre"),
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: wgpu::BufferSize::new(TAILLE_LUMIERE) }, count: None },
            ],
        });
        (dc, dombre)
    }

    fn creer_pipeline_carte(device: &wgpu::Device, layout: &wgpu::PipelineLayout, format: wgpu::TextureFormat, module: &wgpu::ShaderModule, etape: Etape) -> wgpu::RenderPipeline {
        let (melange, profondeur) = Self::etats_etape(etape);
        let format = if etape == Etape::Selection { FORMAT_MASQUE } else { format };
        let attr = [
            wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 0, shader_location: 0 },
            wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 16, shader_location: 1 },
            wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 32, shader_location: 2 },
        ];
        let layout_tampon = wgpu::VertexBufferLayout {
            array_stride: (FLOTTANTS_PAR_INSTANCE * 4) as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &attr,
        };
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pipeline_carte"),
            layout: Some(layout),
            vertex: wgpu::VertexState { module, entry_point: Some("vs"), buffers: &[layout_tampon], compilation_options: Default::default() },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(profondeur),
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState { module, entry_point: Some("fs"), targets: &[Some(wgpu::ColorTargetState { format, blend: melange, write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
            multiview: None,
            cache: None,
        })
    }

    fn creer_pipeline_ombre(device: &wgpu::Device, layout: &wgpu::PipelineLayout, module: &wgpu::ShaderModule) -> wgpu::RenderPipeline {
        let attr = [
            wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 0, shader_location: 0 },
            wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 16, shader_location: 1 },
            wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 32, shader_location: 2 },
        ];
        let layout_tampon = wgpu::VertexBufferLayout {
            array_stride: (FLOTTANTS_PAR_INSTANCE * 4) as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &attr,
        };
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pipeline_ombre"),
            layout: Some(layout),
            vertex: wgpu::VertexState { module, entry_point: Some("vs"), buffers: &[layout_tampon], compilation_options: Default::default() },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: FORMAT_OMBRE,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState { constant: 2, slope_scale: 2.0, clamp: 0.0 },
            }),
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState { module, entry_point: Some("fs"), targets: &[], compilation_options: Default::default() }),
            multiview: None,
            cache: None,
        })
    }

    fn etats_etape(etape: Etape) -> (Option<wgpu::BlendState>, wgpu::DepthStencilState) {
        let melange = match etape {
            Etape::Selection => {
                let maximum = wgpu::BlendComponent { src_factor: wgpu::BlendFactor::One, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Max };
                Some(wgpu::BlendState { color: maximum, alpha: maximum })
            }
            _ => Some(wgpu::BlendState {
                color: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::SrcAlpha, dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha, operation: wgpu::BlendOperation::Add },
                alpha: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::Zero, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add },
            }),
        };
        let profondeur = wgpu::DepthStencilState {
            format: FORMAT_PROFONDEUR,
            depth_write_enabled: etape == Etape::Scene,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        };
        (melange, profondeur)
    }
}
