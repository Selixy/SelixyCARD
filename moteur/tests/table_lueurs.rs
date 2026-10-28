use moteur::table::compose::cibles;
use moteur::table::etat::EtatTable;
use moteur::table::lueurs::{MAIN, PIOCHE, couleur, groupe};

fn etat(courant: i32, jouables: Vec<usize>, premier_plan: i32) -> EtatTable {
    EtatTable { main: vec!["a".into(), "b".into()], pioche: 40, courant, jouables, premier_plan, ..Default::default() }
}

fn groupe_pioche(e: &EtatTable) -> Option<u32> {
    let toutes = cibles(e, -1, &[], 0.0, 0.7);
    let sommet = toutes.iter().find(|c| (2000..3000).contains(&c.id)).expect("pioche");
    groupe(e, sommet, -1)
}

#[test]
fn la_pioche_brille_a_mon_tour_seulement() {
    assert_eq!(groupe_pioche(&etat(0, vec![1], -1)), Some(PIOCHE));
    assert_eq!(groupe_pioche(&etat(1, vec![1], -1)), None);
    assert_eq!(groupe_pioche(&etat(0, vec![1], 0)), None);
}

#[test]
fn la_pioche_brille_plus_fort_sans_carte_jouable() {
    let avec_choix = couleur(&etat(0, vec![1], -1), PIOCHE)[3];
    let sans_choix = couleur(&etat(0, vec![], -1), PIOCHE)[3];
    assert!(sans_choix > avec_choix * 2.0);
}

#[test]
fn obligee_de_piocher_la_main_s_efface_devant_la_pioche() {
    let oblige = etat(0, vec![], -1);
    assert!(couleur(&oblige, MAIN)[3] < couleur(&etat(0, vec![1], -1), MAIN)[3] / 2.0);
    assert!(couleur(&oblige, PIOCHE)[3] > couleur(&oblige, MAIN)[3] * 4.0);
}
