use super::souffle::alea;
use crate::scene::{self, Instance, geometrie::{PLAT, Pose}};

const TRACES_PAR_JOKER: u32 = 4;

pub fn traces_sol(centre: [f32; 3], teinte: [f32; 3], graine: u32, couches: &[f32]) -> Vec<Instance> {
    if couches.is_empty() {
        return Vec::new();
    }
    (0..TRACES_PAR_JOKER)
        .map(|i| {
            let r = |k| alea(graine ^ 0x5bd1_e995, i, k);
            let decalage_x = (r(0) - 0.5) * 0.45;
            let decalage_z = (r(1) - 0.5) * 0.45;
            let orientation = (r(2) - 0.5) * 0.6;
            let pose = Pose {
                pos: [centre[0] + decalage_x, 0.005 + 0.0005 * i as f32, centre[2] + decalage_z],
                lacet: orientation,
                tangage: PLAT,
            };
            let largeur = 3.6 + 0.7 * r(3);
            let hauteur = 2.4 + 0.5 * r(4);
            let nuance = 0.85 + 0.25 * r(5);
            let couleur = teinte.map(|c| (c * nuance).min(1.0));
            let couche = couches[(r(6) * couches.len() as f32) as usize % couches.len()];
            scene::poudre(&pose, [largeur, hauteur], couleur, 0.75 + 0.25 * r(7), couche)
        })
        .collect()
}
