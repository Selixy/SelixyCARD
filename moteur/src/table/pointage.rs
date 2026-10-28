use super::disposition::{CARTES_PAR_PILE, GLISSE_JOUABLE, GLISSE_SURVOL, ouverture, pose_main, pose_pioche};
use super::etat::EtatTable;
use crate::maths::Mat4;
use crate::scene::geometrie::{Pose, vers_ecran};
use crate::scene::{HAUTEUR_CARTE, LARGEUR_CARTE};

pub struct Vue {
    pub vue_proj: Mat4,
    pub largeur: f32,
    pub hauteur: f32,
}

pub fn carte_sous_pointeur(etat: &EtatTable, vue: &Vue, x: f32, y: f32, survol: i32) -> i32 {
    let total = if etat.courant == 0 { etat.main.len() } else { 0 };
    let ouverture = ouverture(&vue.vue_proj);
    let touchee = (0..total).rev().find(|&i| {
        let base = pose_main(i, total, ouverture);
        let glissee = if etat.jouables.contains(&i) { base.glisser(GLISSE_JOUABLE) } else { base };
        let tenue = survol == i as i32 && contient(vue, &base.glisser(GLISSE_SURVOL), x, y);
        !etat.est_au_premier_plan(i) && (contient(vue, &glissee, x, y) || tenue)
    });
    if let Some(i) = touchee {
        return if etat.jouables.contains(&i) { i as i32 } else { -1 };
    }
    let sommet = etat.pioche.min(CARTES_PAR_PILE).saturating_sub(1);
    if etat.pioche > 0 && contient(vue, &pose_pioche(sommet), x, y) {
        return -2;
    }
    -1
}

fn contient(vue: &Vue, pose: &Pose, x: f32, y: f32) -> bool {
    let coins = pose.coins(LARGEUR_CARTE, HAUTEUR_CARTE).map(|c| vers_ecran(&vue.vue_proj, c, vue.largeur, vue.hauteur));
    let signes: Vec<f32> = (0..4)
        .map(|i| {
            let (a, b) = (coins[i], coins[(i + 1) % 4]);
            (b[0] - a[0]) * (y - a[1]) - (b[1] - a[1]) * (x - a[0])
        })
        .collect();
    signes.iter().all(|s| *s >= 0.0) || signes.iter().all(|s| *s <= 0.0)
}
