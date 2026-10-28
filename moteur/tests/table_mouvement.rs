use moteur::scene::geometrie::{PLAT, Pose};
use moteur::table::compose::identifiant;
use moteur::table::etat::EtatTable;
use moteur::table::mouvement::Mouvements;
use moteur::table::origines::preparer;
use moteur::table::souffle::Souffles;

fn pose(x: f32, y: f32) -> Pose {
    Pose { pos: [x, y, 0.0], lacet: 0.0, tangage: PLAT }
}

#[test]
fn un_grand_deplacement_atterrit_une_seule_fois() {
    let mut m = Mouvements::default();
    m.apparaitre(1, pose(0.0, 0.0));
    let atterrissages = (0..200).filter(|_| m.suivre(1, &pose(8.0, 0.0), 0.016, false).1).count();
    assert_eq!(atterrissages, 1);
}

#[test]
fn la_carte_jouee_part_de_la_main() {
    let ancien = EtatTable { main: vec!["a".into(), "b".into()], courant: 0, ..Default::default() };
    let nouveau = EtatTable { main: vec!["b".into()], defausse: Some("a".into()), ..Default::default() };
    let mut m = Mouvements::default();
    m.apparaitre(identifiant("a"), pose(3.0, 2.0));
    preparer(&ancien, &nouveau, 3001, &mut m, &mut Souffles::default());
    assert_eq!(m.pose(3001).unwrap().pos, [3.0, 2.0, 0.0]);
}

#[test]
fn une_carte_piochee_part_de_la_pioche() {
    let ancien = EtatTable { main: vec!["a".into()], pioche: 20, ..Default::default() };
    let nouveau = EtatTable { main: vec!["a".into(), "z".into()], pioche: 19, ..Default::default() };
    let mut m = Mouvements::default();
    preparer(&ancien, &nouveau, 3000, &mut m, &mut Souffles::default());
    assert!(m.pose(identifiant("z")).unwrap().pos[0] < -1.0);
}

#[test]
fn une_cible_qui_bouge_n_empeche_pas_d_arriver() {
    let mut m = Mouvements::default();
    m.apparaitre(1, pose(0.0, 0.0));
    let fin = (0..300).map(|i| m.suivre(1, &pose(8.0 + i as f32 * 0.01, 0.0), 0.016, false).0).last().unwrap();
    assert!((fin.pos[0] - 10.99).abs() < 0.2);
}
