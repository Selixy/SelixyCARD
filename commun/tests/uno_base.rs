mod uno_aide;
use commun::uno::{Couleur, Erreur, Options};
use uno_aide::*;

#[test]
fn couleur_et_valeur_se_jouent() {
    let mains: &[&[&str]] = &[&["rouge-5-a", "bleu-5-a", "vert-1-a"], &["bleu-5-b", "jaune-2-a", "vert-1-b"], &["jaune-3-a", "jaune-4-a"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    assert_eq!(p.jouer("a", pose("rouge-5-a")), Ok(()));
    assert_eq!((p.courant, p.couleur, p.haut().id.as_str()), (1, Couleur::Rouge, "rouge-5-a"));
    assert_eq!(p.joueurs[0].main.len(), 2);
    assert_eq!(p.jouer("b", pose("bleu-5-b")), Ok(()));
    assert_eq!((p.courant, p.couleur), (2, Couleur::Bleu));
}

#[test]
fn coups_refuses() {
    let mains: &[&[&str]] = &[&["vert-1-a", "joker-1", "rouge-2-a"], &["rouge-3-a", "rouge-4-a"], &["jaune-3-a", "jaune-4-a"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    assert_eq!(p.jouer("a", pose("vert-1-a")), Err(Erreur::CarteInjouable));
    assert_eq!(p.jouer("b", pose("rouge-3-a")), Err(Erreur::PasTonTour));
    assert_eq!(p.jouer("a", pose("rouge-3-a")), Err(Erreur::CarteAbsente));
    assert_eq!(p.jouer("z", pose("rouge-3-a")), Err(Erreur::JoueurInconnu));
    assert_eq!(p.jouer("a", pose("joker-1")), Err(Erreur::CouleurRequise));
    assert_eq!(p.joueurs[0].main.len(), 3);
    assert_eq!(p.jouer("a", avec_couleur("joker-1", Couleur::Vert)), Ok(()));
    assert_eq!(p.couleur, Couleur::Vert);
    assert_eq!(p.jouer("b", pose("rouge-3-a")), Err(Erreur::CarteInjouable));
}

#[test]
fn passer_et_inverser() {
    let mains: &[&[&str]] = &[&["rouge-passer-a", "rouge-inverser-a", "rouge-1-a"], &["bleu-1-a", "bleu-2-a"], &["vert-1-a", "vert-2-a"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.jouer("a", pose("rouge-passer-a")).unwrap();
    assert_eq!(p.courant, 2);
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.jouer("a", pose("rouge-inverser-a")).unwrap();
    assert_eq!((p.sens, p.courant), (-1, 2));
    let duel: &[&[&str]] = &[&["rouge-inverser-a", "rouge-1-a"], &["bleu-1-a", "bleu-2-a"]];
    let mut p = partie(duel, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.jouer("a", pose("rouge-inverser-a")).unwrap();
    assert_eq!(p.courant, 0);
    let mut p = partie(duel, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.joueurs[0].main.push(carte("rouge-passer-b"));
    p.jouer("a", pose("rouge-passer-b")).unwrap();
    assert_eq!(p.courant, 0);
}

#[test]
fn plus_deux_immediat_sans_cumul() {
    let mains: &[&[&str]] = &[&["rouge-plus2-a", "rouge-1-a"], &["jaune-plus2-a", "bleu-2-a", "bleu-3-a"], &["vert-1-a", "vert-2-a"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.jouer("a", pose("rouge-plus2-a")).unwrap();
    assert_eq!((p.joueurs[1].main.len(), p.courant, p.penalite), (5, 2, 0));
    assert_eq!(p.joueurs[2].main.len(), 2);
}

#[test]
fn derniere_carte_gagne() {
    let mains: &[&[&str]] = &[&["rouge-5-a"], &["bleu-1-a", "bleu-2-a"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.jouer("a", pose("rouge-5-a")).unwrap();
    assert_eq!(p.gagnant, Some(0));
    assert_eq!(p.courant, 0);
    assert_eq!(p.jouer("b", pose("bleu-1-a")), Err(Erreur::PartieTerminee));
    assert_eq!(p.piocher("b"), Err(Erreur::PartieTerminee));
    assert_eq!(p.dire_uno("a"), Err(Erreur::PartieTerminee));
}
