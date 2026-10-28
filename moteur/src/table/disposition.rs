use crate::maths::{Mat4, transformer};
use crate::scene::geometrie::{PLAT, Pose};

pub const RAYON_EVENTAIL: f32 = 10.0;
pub const CARTES_PAR_PILE: u32 = 10;
pub const INCLINAISON_MAIN: f32 = 0.62;
pub const ECART_EVENTAIL: f32 = 0.03;
pub const OUVERTURE_MAX: f32 = 0.7;
pub const PILE_DEFAUSSE: usize = 12;
pub const GLISSE_JOUABLE: f32 = 0.35;
pub const GLISSE_SURVOL: f32 = 1.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Face {
    Main(usize),
    Defausse(usize),
    Dos,
    Bloc,
}

pub struct Cible {
    pub id: u32,
    pub face: Face,
    pub pose: Pose,
    pub echelle: f32,
}

pub fn ouverture(vue_proj: &Mat4) -> f32 {
    let centre = transformer(vue_proj, [0.0, 1.1, 4.6]);
    let bord = transformer(vue_proj, [1.0, 1.1, 4.6]);
    let par_unite = bord[0] / bord[3] - centre[0] / centre[3];
    let demi = 0.94 / par_unite.max(0.001);
    let sinus = ((demi - 0.95) / RAYON_EVENTAIL).clamp(0.02, (OUVERTURE_MAX / 2.0).sin());
    2.0 * sinus.asin()
}

pub fn pose_main(indice: usize, total: usize, ouverture: f32) -> Pose {
    let pas = if total > 1 { 0.13_f32.min(ouverture / (total - 1) as f32) } else { 0.0 };
    let angle = (indice as f32 - (total as f32 - 1.0) / 2.0) * pas;
    Pose {
        pos: [
            RAYON_EVENTAIL * angle.sin(),
            1.1,
            4.2 + RAYON_EVENTAIL * (1.0 - angle.cos()),
        ],
        lacet: -angle,
        tangage: PLAT + INCLINAISON_MAIN,
    }
    .decaler(ECART_EVENTAIL * indice as f32)
}

pub fn pose_pioche(rang: u32) -> Pose {
    Pose { pos: [-1.4 - 0.02 * rang as f32, 0.03 + 0.025 * rang as f32, -0.02 * rang as f32], lacet: 0.0, tangage: PLAT }
}

pub fn pose_defausse(hauteur: usize, graine: u32) -> Pose {
    let alea = |k: f32| ((graine as f32 * 12.9898 + k * 78.233).sin() * 43_758.545).fract().abs() - 0.5;
    Pose {
        pos: [1.4 + 0.35 * alea(1.0), 0.05 + 0.012 * hauteur as f32, 0.3 * alea(2.0)],
        lacet: 0.15 + 1.1 * alea(3.0),
        tangage: PLAT,
    }
}

pub fn depart() -> Pose {
    pose_pioche(CARTES_PAR_PILE)
}
