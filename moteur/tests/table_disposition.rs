use moteur::table::compose::cibles;
use moteur::table::etat::EtatTable;

fn exemple() -> EtatTable {
    EtatTable { main: vec!["a".into(), "b".into(), "c".into()], defausse: Some("d".into()), adversaires: vec![7, 3], pioche: 40, ..Default::default() }
}

#[test]
fn nombre_de_cartes_a_dessiner() {
    assert_eq!(cibles(&exemple(), -1, &[(1, "d".into())], 0.0, 0.7).len(), 3 + 1 + 1 + 7 + 3);
}

#[test]
fn survol_fait_glisser_une_carte_jouable_dans_son_plan() {
    let e = EtatTable { courant: 0, jouables: vec![1], ..exemple() };
    let sans = cibles(&e, -1, &[(1, "d".into())], 0.0, 0.7);
    let avec = cibles(&e, 1, &[(1, "d".into())], 0.0, 0.7);
    assert!(avec[1].pose.pos[1] > sans[1].pose.pos[1] + 0.3);
    let (a, b) = (sans[1].pose, avec[1].pose);
    let n = [a.lacet.sin() * a.tangage.cos(), -a.tangage.sin(), a.lacet.cos() * a.tangage.cos()];
    let profondeur: f32 = (0..3).map(|i| (b.pos[i] - a.pos[i]) * n[i]).sum();
    assert!(profondeur.abs() < 1e-4);
    assert_eq!(avec[0].pose.pos, sans[0].pose.pos);
    let non_jouable = cibles(&e, 0, &[(1, "d".into())], 0.0, 0.7);
    assert_eq!(non_jouable[0].pose.pos, sans[0].pose.pos);
}

#[test]
fn hors_tour_la_main_retrecit_et_descend() {
    let a_moi = cibles(&exemple(), -1, &[(1, "d".into())], 0.0, 0.7);
    let ailleurs = cibles(&exemple(), -1, &[(1, "d".into())], 1.0, 0.7);
    assert!(ailleurs[0].echelle < a_moi[0].echelle);
    assert!(ailleurs[0].pose.pos[1] < a_moi[0].pose.pos[1]);
    assert_eq!(ailleurs[3].pose.pos, a_moi[3].pose.pos);
}
