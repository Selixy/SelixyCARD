mod cibles;
mod pipelines;

use crate::rendu::octets;
use cibles::{Cibles, pas_de_flou};
use pipelines::PipelinesLueur;

pub struct Lueur {
    pipelines: PipelinesLueur,
    echantillon: wgpu::Sampler,
    reglage_horizontal: wgpu::Buffer,
    reglage_vertical: wgpu::Buffer,
    cibles: Option<Cibles>,
    groupes: Option<[wgpu::BindGroup; 3]>,
}

fn tampon_reglage(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor { label: Some("reglage_lueur"), size: 16, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false })
}

impl Lueur {
    pub fn creer(device: &wgpu::Device, format_surface: wgpu::TextureFormat) -> Self {
        let echantillon = device.create_sampler(&wgpu::SamplerDescriptor { mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, ..Default::default() });
        Self { pipelines: PipelinesLueur::creer(device, format_surface), echantillon, reglage_horizontal: tampon_reglage(device), reglage_vertical: tampon_reglage(device), cibles: None, groupes: None }
    }

    pub fn vue_masque(&self) -> Option<&wgpu::TextureView> {
        self.cibles.as_ref().map(|c| &c.masque)
    }

    pub fn redimensionner(&mut self, device: &wgpu::Device, file: &wgpu::Queue, largeur: u32, hauteur: u32) {
        let (horizontal, vertical) = pas_de_flou(largeur, hauteur);
        file.write_buffer(&self.reglage_horizontal, 0, octets(&[horizontal[0], horizontal[1], 0.0, 0.0]));
        file.write_buffer(&self.reglage_vertical, 0, octets(&[vertical[0], vertical[1], 0.0, 0.0]));
        let cibles = Cibles::creer(device, largeur, hauteur);
        let groupe = |reglage: &wgpu::Buffer, source: &wgpu::TextureView| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("groupe_lueur"),
                layout: &self.pipelines.disposition,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: reglage.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(source) },
                    wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&cibles.masque) },
                    wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&self.echantillon) },
                ],
            })
        };
        self.groupes = Some([groupe(&self.reglage_horizontal, &cibles.masque), groupe(&self.reglage_vertical, &cibles.flou_horizontal), groupe(&self.reglage_horizontal, &cibles.flou_vertical)]);
        self.cibles = Some(cibles);
    }

    pub fn composer(&self, encodeur: &mut wgpu::CommandEncoder, vue_cible: &wgpu::TextureView) {
        let (Some(cibles), Some(groupes)) = (&self.cibles, &self.groupes) else { return };
        let etapes = [(&cibles.flou_horizontal, &self.pipelines.flou, &groupes[0], wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)), (&cibles.flou_vertical, &self.pipelines.flou, &groupes[1], wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)), (vue_cible, &self.pipelines.composer, &groupes[2], wgpu::LoadOp::Load)];
        for (vue, pipeline, groupe, chargement) in etapes {
            let mut passe = encodeur.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("passe_lueur"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: vue, resolve_target: None, ops: wgpu::Operations { load: chargement, store: wgpu::StoreOp::Store } })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            passe.set_pipeline(pipeline);
            passe.set_bind_group(0, groupe, &[]);
            passe.draw(0..3, 0..1);
        }
    }
}
