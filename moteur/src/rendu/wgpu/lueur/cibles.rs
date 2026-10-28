use crate::rendu::wgpu::textures::FORMAT_MASQUE;

pub struct Cibles {
    pub masque: wgpu::TextureView,
    pub flou_horizontal: wgpu::TextureView,
    pub flou_vertical: wgpu::TextureView,
}

fn creer(device: &wgpu::Device, nom: &str, largeur: u32, hauteur: u32) -> wgpu::TextureView {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(nom),
        size: wgpu::Extent3d { width: largeur.max(1), height: hauteur.max(1), depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: FORMAT_MASQUE,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}

impl Cibles {
    pub fn creer(device: &wgpu::Device, largeur: u32, hauteur: u32) -> Self {
        let (demi_l, demi_h) = (largeur / 2, hauteur / 2);
        Self {
            masque: creer(device, "masque_selection", largeur, hauteur),
            flou_horizontal: creer(device, "flou_horizontal", demi_l, demi_h),
            flou_vertical: creer(device, "flou_vertical", demi_l, demi_h),
        }
    }
}

pub fn pas_de_flou(largeur: u32, hauteur: u32) -> ([f32; 2], [f32; 2]) {
    let (demi_l, demi_h) = ((largeur / 2).max(1) as f32, (hauteur / 2).max(1) as f32);
    let pas = (0.0012 * hauteur as f32).max(0.75);
    ([pas / demi_l, 0.0], [0.0, pas / demi_h])
}
