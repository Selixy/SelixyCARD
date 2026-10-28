use crate::maths::{Mat4, inverser};
use crate::scene::decal::VolumeDecal;

pub const SOMMETS_PAR_BOITE: u32 = 36;

pub const SOMMETS_BOITE: [[f32; 3]; 36] = [
    [-0.5, -0.5, -0.5], [ 0.5,  0.5, -0.5], [ 0.5, -0.5, -0.5],
    [-0.5, -0.5, -0.5], [-0.5,  0.5, -0.5], [ 0.5,  0.5, -0.5],
    [-0.5, -0.5,  0.5], [ 0.5, -0.5,  0.5], [ 0.5,  0.5,  0.5],
    [-0.5, -0.5,  0.5], [ 0.5,  0.5,  0.5], [-0.5,  0.5,  0.5],
    [-0.5, -0.5, -0.5], [-0.5, -0.5,  0.5], [-0.5,  0.5,  0.5],
    [-0.5, -0.5, -0.5], [-0.5,  0.5,  0.5], [-0.5,  0.5, -0.5],
    [ 0.5, -0.5, -0.5], [ 0.5,  0.5,  0.5], [ 0.5, -0.5,  0.5],
    [ 0.5, -0.5, -0.5], [ 0.5,  0.5, -0.5], [ 0.5,  0.5,  0.5],
    [-0.5, -0.5, -0.5], [ 0.5, -0.5, -0.5], [ 0.5, -0.5,  0.5],
    [-0.5, -0.5, -0.5], [ 0.5, -0.5,  0.5], [-0.5, -0.5,  0.5],
    [-0.5,  0.5, -0.5], [ 0.5,  0.5,  0.5], [ 0.5,  0.5, -0.5],
    [-0.5,  0.5, -0.5], [-0.5,  0.5,  0.5], [ 0.5,  0.5,  0.5],
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConfigurationPasseDecal {
    pub test_profondeur: bool,
    pub ecriture_profondeur: bool,
    pub lecture_profondeur: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParametresProjection {
    pub inv_vue_proj: Mat4,
    pub resolution: [f32; 2],
}

pub struct PasseDecal {
    pub decals: Vec<VolumeDecal>,
}

impl PasseDecal {
    pub fn nouveau() -> Self {
        Self { decals: Vec::new() }
    }

    pub fn ajouter(&mut self, decal: VolumeDecal) {
        self.decals.push(decal);
    }

    pub fn configuration() -> ConfigurationPasseDecal {
        ConfigurationPasseDecal {
            test_profondeur: true,
            ecriture_profondeur: false,
            lecture_profondeur: true,
        }
    }

    pub fn preparer_parametres(vue_proj: &Mat4, resolution: [f32; 2]) -> Option<ParametresProjection> {
        let inv_vue_proj = inverser(vue_proj)?;
        Some(ParametresProjection { inv_vue_proj, resolution })
    }

    pub fn decals_visibles(&self) -> &[VolumeDecal] {
        &self.decals
    }
}
