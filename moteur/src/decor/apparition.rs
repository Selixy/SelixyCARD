use crate::scene::geometrie::Pose;
use crate::scene::{self, Instance, MODE_CARTE, MODE_FEUILLE};
use crate::textures::Textures;
use super::etat::Element;

const DELAI_PAR_CARTE: f32 = 0.10;
const DUREE_DESCENTE: f32 = 0.75;
const HAUTEUR_PLATEAU: f32 = 0.006;

pub fn est_carte(e: &Element) -> bool {
    e.est_mobile() && !e.feuille
}

pub fn est_proche(pos: &[f32; 3], vise: &[f32; 2]) -> bool {
    let dx = pos[0] - vise[0];
    let dz = pos[2] - vise[1];
    dx * dx + dz * dz <= 256.0
}

pub fn amorti(apparition: f32, rang: usize) -> f32 {
    if rang == usize::MAX {
        return 0.0;
    }
    let depart = rang as f32 * DELAI_PAR_CARTE;
    if apparition <= depart {
        return 1.0;
    }
    let p = ((apparition - depart) / DUREE_DESCENTE).min(1.0);
    (1.0 - p) * (1.0 - p) * (1.0 + 2.0 * p)
}

pub fn ajuster_pose(e: &Element, temps: f32, apparition: f32, rang: usize) -> (Pose, f32) {
    let mut pose = Pose { pos: e.pos, lacet: e.lacet, tangage: e.tangage };
    if !est_carte(e) {
        return (pose, 1.0);
    }
    let k = amorti(apparition, rang);
    let flottement = 1.0 - k;
    let p = e.phase + e.pos[0] * 0.14 + e.pos[2] * 0.08;
    let erratique_y = 0.7 * (temps * 0.85 + p).sin() + 0.3 * (temps * 1.35 + p * 1.6).cos();
    let amp_y = e.flotte.min(0.065);
    pose.pos[1] += 2.2 * k + amp_y * (0.5 + 0.5 * erratique_y) * flottement;
    pose.pos[2] += 0.40 * k + 0.02 * (temps * 0.75 + p).sin() * flottement;
    let lacet_depart = e.lacet * 0.65;
    let erratique_lacet = 0.016 * (temps * 0.65 + p * 1.1).cos() + 0.007 * (temps * 1.05 + p * 1.7).sin();
    pose.lacet = lacet_depart * k + e.lacet * (1.0 - k) + erratique_lacet * flottement;
    let erratique_tang = 0.018 * (temps * 0.75 + p * 0.9).sin() + 0.008 * (temps * 1.25 + p * 1.4).cos();
    pose.tangage += 0.14 * k + erratique_tang * flottement;
    (pose, ((1.0 - k) * 1.35).min(1.0))
}

pub fn pousser(e: &Element, temps: f32, apparition: f32, rang: usize, textures: &Textures, sortie: &mut Vec<Instance>) {
    let (pose, opacite) = ajuster_pose(e, temps, apparition, rang);
    let couche = textures.couche(&e.cle).map_or(scene::COUCHE_ABSENTE, |c| c as f32);
    let mode = if e.feuille { MODE_FEUILLE } else { MODE_CARTE * opacite };
    if e.est_mobile() || (!e.feuille && e.pos[1] > 0.05) {
        let alpha = 0.55 * opacite;
        sortie.push(scene::ombre(&pose, e.taille, alpha, HAUTEUR_PLATEAU));
    }
    sortie.push(scene::piece(&pose, e.taille, couche, mode));
}
