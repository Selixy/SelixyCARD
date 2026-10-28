use crate::scene::geometrie::{PLAT, Pose};

pub const ECHELLE_ADVERSAIRE: f32 = 0.62;
pub const CARTES_VISIBLES_ADVERSAIRE: u32 = 9;
const INCLINAISON_ADVERSAIRE: f32 = 1.38;

pub fn serrage(ouverture: f32) -> f32 {
    (ouverture / super::disposition::OUVERTURE_MAX).clamp(0.5, 1.0)
}

pub fn centre_adversaire(rang: usize, total: usize, serre: f32) -> [f32; 2] {
    let phi = std::f32::consts::PI * (rang + 1) as f32 / (total + 1) as f32;
    [-6.0 * serre * phi.cos(), -2.8 * phi.sin()]
}

pub fn pose_adversaire(rang: usize, total: usize, carte: u32, visibles: u32, serre: f32) -> Pose {
    let [cx, cz] = centre_adversaire(rang, total, serre);
    let theta = (-cx).atan2(1.0 - cz);
    let angle = (carte as f32 - (visibles as f32 - 1.0) / 2.0) * 0.085;
    let rayon = 3.5;
    let lx = rayon * angle.sin();
    let lz = rayon * (1.0 - angle.cos());
    Pose {
        pos: [
            cx + lx * theta.cos() + lz * theta.sin(),
            0.8,
            cz - lx * theta.sin() + lz * theta.cos(),
        ],
        lacet: theta - angle,
        tangage: PLAT + INCLINAISON_ADVERSAIRE,
    }
    .decaler(super::disposition::ECART_EVENTAIL * carte as f32)
}

pub fn pose_premier_plan() -> Pose {
    Pose { pos: [0.0, 4.6, 5.6], lacet: 0.0, tangage: PLAT + 0.65 }
}
