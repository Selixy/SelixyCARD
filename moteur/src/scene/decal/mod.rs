mod fondu;

use crate::maths::{Mat4, identite, inverser, mise_a_echelle, multiplier, rotation_euler, transformer, translation};
pub use fondu::{attenuation_angulaire, attenuation_distance};

pub type DecalProjete = VolumeDecal;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VolumeDecal {
    pub transformation: Mat4,
    pub inverse: Mat4,
    pub texture: u32,
    pub opacite: f32,
    pub fondu: f32,
}

impl VolumeDecal {
    pub fn composer(pos: [f32; 3], rot: [f32; 3], dim: [f32; 3], texture: u32, opacite: f32, fondu: f32) -> Self {
        let t = translation(pos);
        let r = rotation_euler(rot);
        let s = mise_a_echelle(dim);
        let tr = multiplier(&t, &r);
        let transformation = multiplier(&tr, &s);
        let inverse = inverser(&transformation).unwrap_or(identite());
        Self { transformation, inverse, texture, opacite, fondu }
    }

    pub fn depuis_matrice(transformation: Mat4, texture: u32, opacite: f32, fondu: f32) -> Option<Self> {
        let inverse = inverser(&transformation)?;
        Some(Self { transformation, inverse, texture, opacite, fondu })
    }

    pub fn projeter_point(&self, point: [f32; 3]) -> [f32; 3] {
        let h = transformer(&self.inverse, point);
        let w = if h[3].abs() > 1e-6 { h[3] } else { 1.0 };
        [h[0] / w, h[1] / w, h[2] / w]
    }

    pub fn contient(&self, point: [f32; 3]) -> bool {
        let local = self.projeter_point(point);
        local[0].abs() <= 0.5 && local[1].abs() <= 0.5 && local[2].abs() <= 0.5
    }

    pub fn projeter_uv(&self, point: [f32; 3]) -> Option<[f32; 2]> {
        let local = self.projeter_point(point);
        if local[0].abs() > 0.5 || local[1].abs() > 0.5 || local[2].abs() > 0.5 {
            return None;
        }
        Some([local[0] + 0.5, local[2] + 0.5])
    }
}
