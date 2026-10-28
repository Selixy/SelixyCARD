use super::geometrie::{PLAT, Pose};
use super::{COUCHE_OMBRE, Instance, MODE_POUDRE, instance};

pub fn ombre(pose: &Pose, taille: [f32; 2], alpha: f32, hauteur: f32) -> Instance {
    let altitude = (pose.pos[1] - hauteur).max(0.0);
    let decalage_x = -0.04 - 0.36 * altitude;
    let decalage_z = 0.05 + 0.44 * altitude;
    let au_sol = Pose {
        pos: [pose.pos[0] + decalage_x, hauteur, pose.pos[2] + decalage_z],
        lacet: pose.lacet,
        tangage: PLAT,
    };
    let dilatation = 1.04 + 0.15 * altitude.min(3.0);
    let attenuation = 1.0 / (1.0 + 0.22 * altitude);
    let opacite = (alpha * 1.5 * attenuation).clamp(0.0, 0.90);
    instance(&au_sol, [0.015, 0.02, 0.025, opacite], [taille[0] * dilatation, taille[1] * dilatation], COUCHE_OMBRE)
}

pub fn halo(pose: &Pose, taille: [f32; 2], couleur: [f32; 4]) -> Instance {
    instance(pose, couleur, taille, COUCHE_OMBRE)
}

pub fn poudre(pose: &Pose, taille: [f32; 2], teinte: [f32; 3], opacite: f32, couche: f32) -> Instance {
    instance(pose, [teinte[0], teinte[1], teinte[2], MODE_POUDRE + 0.95 * opacite], taille, couche)
}
