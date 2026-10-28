mod uno_aide;

use commun::uno::{Couleur, Options};
use uno_aide::partie;

#[test]
fn seules_les_cartes_posables_sont_jouables_a_mon_tour() {
    let p = partie(&[&["rouge-5-a", "bleu-3-a", "joker-1"], &["vert-2-a"]], "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    assert_eq!(p.jouables(0), vec!["rouge-5-a".to_string(), "joker-1".to_string()]);
}

#[test]
fn rien_n_est_jouable_hors_de_mon_tour() {
    let p = partie(&[&["rouge-5-a"], &["rouge-7-a"]], "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    assert!(p.jouables(1).is_empty());
}

#[test]
fn apres_une_pioche_jouable_seule_la_piochee_est_jouable() {
    let mut p = partie(&[&["rouge-5-a", "bleu-3-a"], &["vert-2-a"]], "bleu-9-a", Couleur::Bleu, Options::default(), &["bleu-1-a"]);
    p.piocher("a").unwrap();
    assert_eq!(p.piochee_de(0), Some("bleu-1-a"));
    assert_eq!(p.jouables(0), vec!["bleu-1-a".to_string()]);
    assert_eq!(p.piochee_de(1), None);
}
