use moteur::scene::{self, geometrie::{Pose, vers_ecran}};
use moteur::table::disposition::{ouverture, pose_main, pose_pioche};
use moteur::table::etat::EtatTable;
use moteur::table::pointage::{Vue, carte_sous_pointeur};

fn etat() -> EtatTable {
    EtatTable { main: vec!["a".into(), "b".into(), "c".into()], defausse: None, adversaires: vec![], pioche: 40, courant: 0, jouables: vec![0, 1, 2], ..Default::default() }
}

fn vue(largeur: f32, hauteur: f32) -> Vue {
    Vue { vue_proj: scene::camera(largeur / hauteur, true), largeur, hauteur }
}

fn centre(v: &Vue, pose: Pose) -> [f32; 2] {
    let c = pose.coins(1.5, 2.25);
    let milieu = std::array::from_fn(|i| (c[0][i] + c[2][i]) / 2.0);
    vers_ecran(&v.vue_proj, milieu, v.largeur, v.hauteur)
}

#[test]
fn pointage_trouve_la_main_la_pioche_et_le_vide() {
    let v = vue(1280.0, 720.0);
    let main = centre(&v, pose_main(2, 3, ouverture(&v.vue_proj)).glisser(0.35));
    assert_eq!(carte_sous_pointeur(&etat(), &v, main[0], main[1], -1), 2);
    let hors_tour = EtatTable { courant: 1, ..etat() };
    assert_eq!(carte_sous_pointeur(&hors_tour, &v, main[0], main[1], -1), -1);
    let non_jouable = EtatTable { jouables: vec![0, 1], ..etat() };
    assert_eq!(carte_sous_pointeur(&non_jouable, &v, main[0], main[1], -1), -1);
    let pioche = centre(&v, pose_pioche(9));
    assert_eq!(carte_sous_pointeur(&etat(), &v, pioche[0], pioche[1], -1), -2);
    assert_eq!(carte_sous_pointeur(&etat(), &v, 5.0, 5.0, -1), -1);
}

#[test]
fn pointage_fonctionne_en_portrait() {
    let v = vue(390.0, 844.0);
    let main = centre(&v, pose_main(0, 3, ouverture(&v.vue_proj)).glisser(0.35));
    assert_eq!(carte_sous_pointeur(&etat(), &v, main[0], main[1], -1), 0);
    assert!(main[0] > 0.0 && main[1] < 844.0);
}
