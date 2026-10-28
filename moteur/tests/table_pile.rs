use moteur::table::Table;
use moteur::table::disposition::{Face, PILE_DEFAUSSE, pose_defausse};
use moteur::table::etat::EtatTable;
use moteur::textures::Textures;

#[test]
fn la_defausse_s_accumule_sans_depasser_la_pile() {
    let mut table = Table::default();
    for n in 0..20 {
        table.definir(EtatTable { defausse: Some(format!("defaut:c{n}")), ..Default::default() });
    }
    let (instances, _) = table.instances(0.016, -1, &Textures::default(), 0.7);
    let cartes = instances.iter().filter(|i| i.couleur == [0.55, 0.55, 0.6, 1.0] && i.forme[3] == -2.0 && i.position_lacet[1] > 0.04).count();
    assert_eq!(cartes, PILE_DEFAUSSE);
}

#[test]
fn la_pile_tourne_au_hasard_et_monte() {
    let angles: Vec<f32> = (0..6).map(|n| pose_defausse(0, n).lacet).collect();
    let ecart = angles.iter().cloned().fold(f32::MIN, f32::max) - angles.iter().cloned().fold(f32::MAX, f32::min);
    assert!(ecart > 0.3);
    assert!(pose_defausse(5, 1).pos[1] > pose_defausse(0, 1).pos[1]);
    let _ = Face::Defausse(0);
}
