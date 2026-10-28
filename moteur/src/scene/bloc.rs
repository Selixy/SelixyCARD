use super::geometrie::Pose;
use super::{COUCHE_BLOC, HAUTEUR_CARTE, Instance, LARGEUR_CARTE};

pub const SURELEVATION: f32 = 0.035;

pub fn bloc(sommet: &Pose, couleur: [f32; 4]) -> Instance {
    let haut = (sommet.pos[1] - SURELEVATION).max(0.01);
    Instance {
        position_lacet: [sommet.pos[0], haut, sommet.pos[2], 0.0],
        couleur,
        forme: [LARGEUR_CARTE, HAUTEUR_CARTE, haut, COUCHE_BLOC],
    }
}
