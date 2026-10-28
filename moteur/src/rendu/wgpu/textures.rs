use crate::rendu::{TEXTURE_HAUTEUR, TEXTURE_LARGEUR};
use crate::textures::COUCHES;
use wasm_bindgen::JsValue;

pub const FORMAT_PROFONDEUR: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
pub const FORMAT_MASQUE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
pub const FORMAT_OMBRE: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const RESOLUTION_OMBRE: u32 = 2048;

pub struct TexturesWgpu {
    pub tableau: wgpu::Texture,
    pub vue_tableau: wgpu::TextureView,
    pub echantillon: wgpu::Sampler,
    pub texture_ombre: wgpu::Texture,
    pub vue_ombre: wgpu::TextureView,
    pub echantillon_ombre: wgpu::Sampler,
    pub profondeur: Option<(wgpu::Texture, wgpu::TextureView)>,
}

impl TexturesWgpu {
    pub fn creer(device: &wgpu::Device) -> Self {
        let taille_tableau = wgpu::Extent3d { width: TEXTURE_LARGEUR, height: TEXTURE_HAUTEUR, depth_or_array_layers: COUCHES };
        let tableau = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("tableau_textures"),
            size: taille_tableau,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let vue_tableau = tableau.create_view(&wgpu::TextureViewDescriptor {
            label: Some("vue_tableau"),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        let echantillon = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("echantillon_carte"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let (texture_ombre, vue_ombre, echantillon_ombre) = Self::creer_ombre(device);
        Self { tableau, vue_tableau, echantillon, texture_ombre, vue_ombre, echantillon_ombre, profondeur: None }
    }

    fn creer_ombre(device: &wgpu::Device) -> (wgpu::Texture, wgpu::TextureView, wgpu::Sampler) {
        let taille = wgpu::Extent3d { width: RESOLUTION_OMBRE, height: RESOLUTION_OMBRE, depth_or_array_layers: 1 };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture_ombre"),
            size: taille,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: FORMAT_OMBRE,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let vue = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let echantillon = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("echantillon_ombre"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        (texture, vue, echantillon)
    }

    pub fn redimensionner(&mut self, device: &wgpu::Device, largeur: u32, hauteur: u32) {
        if let Some((ancienne, _)) = self.profondeur.take() {
            ancienne.destroy();
        }
        let taille = wgpu::Extent3d { width: largeur.max(1), height: hauteur.max(1), depth_or_array_layers: 1 };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture_profondeur"),
            size: taille,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: FORMAT_PROFONDEUR,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let vue = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.profondeur = Some((texture, vue));
    }

    #[cfg(target_arch = "wasm32")]
    pub fn charger_couche(&self, file: &wgpu::Queue, couche: u32, source: &JsValue) -> Result<(), JsValue> {
        use wasm_bindgen::JsCast;
        let (src, l, h) = if let Some(c) = source.dyn_ref::<web_sys::HtmlCanvasElement>() {
            (wgpu::ExternalImageSource::HTMLCanvasElement(c.clone()), c.width(), c.height())
        } else if let Some(b) = source.dyn_ref::<web_sys::ImageBitmap>() {
            (wgpu::ExternalImageSource::ImageBitmap(b.clone()), b.width(), b.height())
        } else {
            return Err(JsValue::from_str("source image invalide"));
        };
        let source_info = wgpu::CopyExternalImageSourceInfo { source: src, origin: wgpu::Origin2d::ZERO, flip_y: false };
        let dest_info = wgpu::CopyExternalImageDestInfo {
            texture: &self.tableau,
            mip_level: 0,
            origin: wgpu::Origin3d { x: 0, y: 0, z: couche },
            aspect: wgpu::TextureAspect::All,
            color_space: wgpu::PredefinedColorSpace::Srgb,
            premultiplied_alpha: false,
        };
        let taille = wgpu::Extent3d { width: l.min(TEXTURE_LARGEUR), height: h.min(TEXTURE_HAUTEUR), depth_or_array_layers: 1 };
        file.copy_external_image_to_texture(&source_info, dest_info, taille);
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn charger_couche(&self, _file: &wgpu::Queue, _couche: u32, _source: &JsValue) -> Result<(), JsValue> {
        Ok(())
    }
}
