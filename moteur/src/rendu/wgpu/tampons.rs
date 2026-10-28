use crate::rendu::{FLOTTANTS_PAR_INSTANCE, MAX_INSTANCES, octets};

pub const TAILLE_GLOBALES: u64 = 144;
pub const TAILLE_LUMIERE: u64 = 64;

pub struct Tampons {
    pub globales: wgpu::Buffer,
    pub lumiere: wgpu::Buffer,
    pub instances: wgpu::Buffer,
    pub instances_ombres: wgpu::Buffer,
}

impl Tampons {
    pub fn creer(device: &wgpu::Device) -> Self {
        let usage_u = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;
        let usage_v = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST;
        let taille_inst = (MAX_INSTANCES * FLOTTANTS_PAR_INSTANCE * 4) as u64;
        let globales = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("tampon_globales"),
            size: TAILLE_GLOBALES,
            usage: usage_u,
            mapped_at_creation: false,
        });
        let lumiere = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("tampon_lumiere"),
            size: TAILLE_LUMIERE,
            usage: usage_u,
            mapped_at_creation: false,
        });
        let instances = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("tampon_instances"),
            size: taille_inst,
            usage: usage_v,
            mapped_at_creation: false,
        });
        let instances_ombres = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("tampon_instances_ombres"),
            size: taille_inst,
            usage: usage_v,
            mapped_at_creation: false,
        });
        Self { globales, lumiere, instances, instances_ombres }
    }

    pub fn ecrire_globales(&self, file: &wgpu::Queue, vue_proj: &[f32; 16], vue_proj_lum: &[f32; 16], oeil: [f32; 3]) {
        file.write_buffer(&self.globales, 0, octets(vue_proj));
        file.write_buffer(&self.globales, 64, octets(vue_proj_lum));
        file.write_buffer(&self.globales, 128, octets(&[oeil[0], oeil[1], oeil[2], 1.0]));
    }

    pub fn ecrire_lumiere(&self, file: &wgpu::Queue, vue_proj_lum: &[f32; 16]) {
        file.write_buffer(&self.lumiere, 0, octets(vue_proj_lum));
    }

    pub fn ecrire_instances(&self, file: &wgpu::Queue, donnees: &[f32]) {
        file.write_buffer(&self.instances, 0, octets(donnees));
    }

    pub fn ecrire_instances_ombres(&self, file: &wgpu::Queue, donnees: &[f32]) {
        file.write_buffer(&self.instances_ombres, 0, octets(donnees));
    }
}
