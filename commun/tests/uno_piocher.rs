mod uno_aide;
use commun::uno::{Couleur, Erreur, Options};
use uno_aide::*;

const MAINS: &[&[&str]] = &[&["rouge-5-a", "bleu-1-a"], &["jaune-2-a", "jaune-3-a"]];

#[test]
fn carte_non_jouable_passe_le_tour() {
    let mut p = partie(MAINS, "rouge-9-a", Couleur::Rouge, Options::default(), &["vert-1-a", "rouge-3-a"]);
    p.piocher("a").unwrap();
    assert_eq!((p.joueurs[0].main.len(), p.courant, p.piochee.clone()), (3, 1, None));
    assert_eq!(p.piocher("a"), Err(Erreur::PasTonTour));
}

#[test]
fn carte_jouable_reste_a_jouer() {
    let mut p = partie(MAINS, "rouge-9-a", Couleur::Rouge, Options::default(), &["rouge-3-a", "vert-1-a"]);
    p.piocher("a").unwrap();
    assert_eq!((p.courant, p.piochee.as_deref()), (0, Some("rouge-3-a")));
    assert_eq!(p.jouer("a", pose("rouge-5-a")), Err(Erreur::SeulementPiochee));
    assert_eq!(p.jouer("a", pose("rouge-3-a")), Ok(()));
    assert_eq!((p.courant, p.piochee.clone()), (1, None));
    let mut q = partie(MAINS, "rouge-9-a", Couleur::Rouge, Options::default(), &["rouge-3-a"]);
    q.piocher("a").unwrap();
    q.piocher("a").unwrap();
    assert_eq!((q.joueurs[0].main.len(), q.courant), (3, 1));
}

#[test]
fn piocher_jusqua_jouable_active() {
    let options = Options { piocher_jusqua_jouable: true, ..Options::default() };
    let mut p = partie(MAINS, "rouge-9-a", Couleur::Rouge, options, &["vert-1-a", "bleu-2-a", "rouge-3-a", "jaune-1-a"]);
    p.piocher("a").unwrap();
    assert_eq!(p.joueurs[0].main.len(), 5);
    assert_eq!((p.courant, p.piochee.as_deref()), (0, Some("rouge-3-a")));
    assert_eq!(p.pioche.len(), 100);
}

#[test]
fn piocher_jusqua_jouable_desactive() {
    let mut p = partie(MAINS, "rouge-9-a", Couleur::Rouge, Options::default(), &["vert-1-a", "bleu-2-a", "rouge-3-a"]);
    p.piocher("a").unwrap();
    assert_eq!((p.joueurs[0].main.len(), p.courant), (3, 1));
}

#[test]
fn recyclage_de_la_defausse() {
    let mut p = partie(MAINS, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.defausse = vec![carte("vert-1-a"), carte("vert-2-a"), carte("rouge-9-a")];
    p.pioche.clear();
    p.piocher("a").unwrap();
    assert_eq!(p.joueurs[0].main.len(), 3);
    assert_eq!((p.pioche.len(), p.defausse.len()), (1, 1));
    assert_eq!(p.haut().id, "rouge-9-a");
    p.defausse.truncate(1);
    p.pioche.clear();
    p.piocher("b").unwrap();
    assert_eq!((p.joueurs[1].main.len(), p.courant), (2, 0));
}
