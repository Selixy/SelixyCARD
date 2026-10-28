use crate::rendu::{Etape, SOMMETS_BLOC, SOMMETS_PAR_CARTE, Trace};

pub fn passer_ombres(
    encodeur: &mut wgpu::CommandEncoder,
    vue_ombre: &wgpu::TextureView,
    pipeline: &wgpu::RenderPipeline,
    groupe: &wgpu::BindGroup,
    tampon_instances: &wgpu::Buffer,
    nb_instances: u32,
    nb_blocs: u32,
) {
    if nb_instances + nb_blocs == 0 {
        return;
    }
    let mut passe = encodeur.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("passe_ombres"),
        color_attachments: &[],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: vue_ombre,
            depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
            stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
    });
    passe.set_pipeline(pipeline);
    passe.set_bind_group(0, groupe, &[]);
    passe.set_vertex_buffer(0, tampon_instances.slice(..));
    passe.draw(0..SOMMETS_PAR_CARTE, 0..nb_instances);
    passe.draw(0..SOMMETS_BLOC, nb_instances..nb_instances + nb_blocs);
}

pub fn passer_principale(
    encodeur: &mut wgpu::CommandEncoder,
    vue_cible: &wgpu::TextureView,
    vue_profondeur: &wgpu::TextureView,
    pipelines: &[wgpu::RenderPipeline; 3],
    groupe: &wgpu::BindGroup,
    tampon_instances: &wgpu::Buffer,
    traces: &[Trace],
) {
    let mut passe = encodeur.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("passe_principale"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: vue_cible,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.17, g: 0.19, b: 0.22, a: 1.0 }),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: vue_profondeur,
            depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
            stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
    });
    passe.set_vertex_buffer(0, tampon_instances.slice(..));
    for trace in traces.iter().filter(|t| t.etape != Etape::Selection) {
        passe.set_pipeline(&pipelines[trace.etape as usize]);
        passe.set_bind_group(0, groupe, &[]);
        passe.draw(0..trace.sommets, trace.debut..(trace.debut + trace.nombre));
    }
}

pub fn passer_masque(
    encodeur: &mut wgpu::CommandEncoder,
    vue_masque: &wgpu::TextureView,
    vue_profondeur: &wgpu::TextureView,
    pipelines: &[wgpu::RenderPipeline; 3],
    groupe: &wgpu::BindGroup,
    tampon_instances: &wgpu::Buffer,
    traces: &[Trace],
) {
    let mut passe = encodeur.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("passe_masque"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: vue_masque,
            resolve_target: None,
            ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: vue_profondeur, depth_ops: None, stencil_ops: None }),
        timestamp_writes: None,
        occlusion_query_set: None,
    });
    passe.set_vertex_buffer(0, tampon_instances.slice(..));
    passe.set_pipeline(&pipelines[Etape::Selection as usize]);
    passe.set_bind_group(0, groupe, &[]);
    for trace in traces.iter().filter(|t| t.etape == Etape::Selection) {
        passe.draw(0..trace.sommets, trace.debut..(trace.debut + trace.nombre));
    }
}
