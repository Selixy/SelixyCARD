use moteur::scene::{self, geometrie::vers_ecran};
use moteur::table::disposition::{ouverture, pose_main};
use moteur::table::pointage::Vue;

fn vue(largeur: f32, hauteur: f32) -> Vue {
    Vue { vue_proj: scene::camera(largeur / hauteur, true), largeur, hauteur }
}

#[test]
fn la_main_se_resserre_en_portrait() {
    assert!(ouverture(&vue(390.0, 844.0).vue_proj) < ouverture(&vue(1280.0, 720.0).vue_proj));
}

#[test]
fn une_grande_main_reste_dans_l_ecran() {
    let v = vue(390.0, 844.0);
    let o = ouverture(&v.vue_proj);
    for i in [0, 24] {
        let c = pose_main(i, 25, o).coins(1.5, 2.25).map(|p| vers_ecran(&v.vue_proj, p, v.largeur, v.hauteur));
        assert!(c.iter().all(|p| p[0] >= 0.0 && p[0] <= v.largeur), "carte {i} hors écran : {c:?}");
    }
}
