use super::VolumeDecal;
use crate::maths::vecteurs::{normaliser, scalaire};

pub fn attenuation_distance(local: [f32; 3], fondu: f32, opacite: f32) -> f32 {
    if local[0].abs() > 0.5 || local[1].abs() > 0.5 || local[2].abs() > 0.5 {
        return 0.0;
    }
    if fondu <= 0.0 {
        return opacite;
    }
    let dx = 0.5 - local[0].abs();
    let dy = 0.5 - local[1].abs();
    let dz = 0.5 - local[2].abs();
    let dist = dx.min(dy).min(dz);
    let facteur = (dist / fondu).clamp(0.0, 1.0);
    opacite * facteur
}

pub fn attenuation_angulaire(normale_monde: [f32; 3], axe_decal: [f32; 3], seuil: f32) -> f32 {
    let n = normaliser(normale_monde);
    let a = normaliser(axe_decal);
    let alignement = scalaire(n, a).abs();
    if seuil <= 0.0 {
        return 1.0;
    }
    ((alignement - (1.0 - seuil)) / seuil).clamp(0.0, 1.0)
}

impl VolumeDecal {
    pub fn attenuation(&self, point: [f32; 3]) -> f32 {
        attenuation_distance(self.projeter_point(point), self.fondu, self.opacite)
    }
}
