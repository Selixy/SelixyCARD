mod uno_aide;
use commun::uno::{Couleur, Erreur, Options};
use uno_aide::*;

const MAINS: &[&[&str]] = &[&["vert-1-a", "vert-2-a"], &["rouge-5-b", "bleu-5-a", "jaune-1-a"], &["jaune-3-a", "jaune-4-a"]];

fn actif() -> Options {
    Options { jump_in: true, ..Options::default() }
}

#[test]
fn carte_identique_hors_tour_quand_actif() {
    let mut p = partie(MAINS, "rouge-5-a", Couleur::Rouge, actif(), &[]);
    assert_eq!(p.jouer("b", pose("rouge-5-b")), Ok(()));
    assert_eq!((p.courant, p.haut().id.as_str()), (2, "rouge-5-b"));
    assert_eq!(p.joueurs[0].main.len(), 2);
}

#[test]
fn carte_seulement_compatible_refusee_hors_tour() {
    let mut p = partie(MAINS, "rouge-5-a", Couleur::Rouge, actif(), &[]);
    assert_eq!(p.jouer("b", pose("bleu-5-a")), Err(Erreur::PasTonTour));
    assert_eq!(p.jouer("b", pose("jaune-1-a")), Err(Erreur::PasTonTour));
    assert_eq!(p.courant, 0);
}

#[test]
fn saut_desactive_refuse() {
    let mut p = partie(MAINS, "rouge-5-a", Couleur::Rouge, Options::default(), &[]);
    assert_eq!(p.jouer("b", pose("rouge-5-b")), Err(Erreur::PasTonTour));
    assert_eq!(p.courant, 0);
}

#[test]
fn saut_applique_l_effet_et_ordre_depuis_le_sauteur() {
    let mains: &[&[&str]] = &[&["vert-1-a", "vert-2-a"], &["rouge-plus2-b", "bleu-5-a"], &["jaune-3-a", "jaune-4-a"]];
    let mut p = partie(mains, "rouge-plus2-a", Couleur::Rouge, actif(), &[]);
    p.jouer("b", pose("rouge-plus2-b")).unwrap();
    assert_eq!((p.joueurs[2].main.len(), p.courant), (4, 0));
    let mut q = partie(MAINS, "rouge-5-a", Couleur::Rouge, Options { cumul_plus: true, ..actif() }, &[]);
    q.penalite = 2;
    assert_eq!(q.jouer("b", pose("rouge-5-b")), Err(Erreur::PasTonTour));
}
