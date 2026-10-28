pub mod bloc;
pub mod camera;
pub mod decal;
pub mod effets;
pub mod geometrie;
pub mod selection;

pub use decal::{DecalProjete, VolumeDecal};

use geometrie::{PLAT, Pose};

pub use camera::{VueDecor, camera, camera_decor, oeil, oeil_decor};
pub use effets::{halo, ombre, poudre};
pub use selection::Selection;

pub const LARGEUR_CARTE: f32 = 1.5;
pub const HAUTEUR_CARTE: f32 = 2.25;
pub const COUCHE_TAPIS: f32 = -1.0;
pub const COUCHE_ABSENTE: f32 = -2.0;
pub const COUCHE_OMBRE: f32 = -3.0;
pub const COUCHE_BLOC: f32 = -5.0;
pub const MODE_CARTE: f32 = 1.0;
pub const MODE_FEUILLE: f32 = 2.0;
pub const MODE_POUDRE: f32 = 4.0;
pub const MODE_SILHOUETTE: f32 = 8.0;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Instance {
    pub position_lacet: [f32; 4],
    pub couleur: [f32; 4],
    pub forme: [f32; 4],
}

pub const THEME_ARDOISE: f32 = 1.0;
pub const THEME_KRAFT: f32 = 2.0;

pub fn tapis(theme: f32) -> Instance {
    let pose = Pose { pos: [0.0, 0.0, 0.5], lacet: 0.0, tangage: PLAT };
    let teinte = if theme == THEME_KRAFT { [0.3, 0.21, 0.14] } else { [0.2, 0.215, 0.225] };
    instance(&pose, [teinte[0], teinte[1], teinte[2], theme], [130.0, 86.0], COUCHE_TAPIS)
}

pub fn carte(pose: &Pose, echelle: f32, couche: f32) -> Instance {
    let taille = [LARGEUR_CARTE * echelle, HAUTEUR_CARTE * echelle];
    piece(pose, taille, couche, MODE_CARTE)
}

pub fn tranche(pose: &Pose, echelle: f32) -> Instance {
    let taille = [LARGEUR_CARTE * echelle, HAUTEUR_CARTE * echelle];
    instance(&pose.decaler(-0.018 * echelle), [0.84, 0.83, 0.79, MODE_CARTE], taille, COUCHE_ABSENTE)
}

pub fn piece(pose: &Pose, taille: [f32; 2], couche: f32, mode: f32) -> Instance {
    instance(pose, [0.55, 0.55, 0.6, mode], taille, couche)
}

pub(crate) fn instance(pose: &Pose, couleur: [f32; 4], taille: [f32; 2], couche: f32) -> Instance {
    Instance {
        position_lacet: [pose.pos[0], pose.pos[1], pose.pos[2], pose.lacet],
        couleur,
        forme: [taille[0], taille[1], pose.tangage, couche],
    }
}

