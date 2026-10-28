mod uno_aide;
use commun::uno::{Couleur, Erreur, Options};
use uno_aide::*;

fn actif() -> Options {
    Options { sept_zero: true, ..Options::default() }
}

#[test]
fn sept_echange_les_mains_quand_actif() {
    let mains: &[&[&str]] = &[&["rouge-7-a", "bleu-1-a"], &["vert-2-a", "vert-3-a", "vert-4-a"], &["jaune-1-a", "jaune-2-a"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, actif(), &[]);
    assert_eq!(p.jouer("a", pose("rouge-7-a")), Err(Erreur::CibleRequise));
    assert_eq!(p.jouer("a", avec_cible("rouge-7-a", "a")), Err(Erreur::CibleInvalide));
    assert_eq!(p.jouer("a", avec_cible("rouge-7-a", "z")), Err(Erreur::CibleInvalide));
    p.jouer("a", avec_cible("rouge-7-a", "b")).unwrap();
    assert_eq!((p.joueurs[0].main.len(), p.joueurs[1].main.len(), p.courant), (3, 1, 1));
    assert_eq!(p.joueurs[1].main[0].id, "bleu-1-a");
}

#[test]
fn sept_sans_effet_quand_inactif() {
    let mains: &[&[&str]] = &[&["rouge-7-a", "bleu-1-a"], &["vert-2-a", "vert-3-a", "vert-4-a"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.jouer("a", avec_cible("rouge-7-a", "b")).unwrap();
    assert_eq!((p.joueurs[0].main.len(), p.joueurs[1].main.len(), p.courant), (1, 3, 1));
    let mut q = partie(mains, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    q.jouer("a", pose("rouge-7-a")).unwrap();
    assert_eq!(q.joueurs[0].main[0].id, "bleu-1-a");
}

#[test]
fn zero_fait_tourner_les_mains_quand_actif() {
    let mains: &[&[&str]] = &[&["rouge-0", "bleu-1-a"], &["vert-2-a"], &["jaune-3-a", "jaune-4-a"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, actif(), &[]);
    p.jouer("a", pose("rouge-0")).unwrap();
    let ids = |i: usize| p.joueurs[i].main.iter().map(|c| c.id.clone()).collect::<Vec<_>>();
    assert_eq!(ids(0), ["jaune-3-a", "jaune-4-a"]);
    assert_eq!(ids(1), ["bleu-1-a"]);
    assert_eq!(ids(2), ["vert-2-a"]);
    let mut q = partie(mains, "rouge-9-a", Couleur::Rouge, actif(), &[]);
    q.sens = -1;
    q.jouer("a", pose("rouge-0")).unwrap();
    assert_eq!(q.joueurs[2].main[0].id, "bleu-1-a");
}

#[test]
fn zero_sans_effet_quand_inactif_et_victoire_avant_echange() {
    let mains: &[&[&str]] = &[&["rouge-0", "bleu-1-a"], &["vert-2-a"], &["jaune-3-a", "jaune-4-a"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.jouer("a", pose("rouge-0")).unwrap();
    assert_eq!((p.joueurs[0].main.len(), p.joueurs[1].main.len(), p.joueurs[2].main.len()), (1, 1, 2));
    let derniere: &[&[&str]] = &[&["rouge-7-a"], &["vert-2-a", "vert-3-a"]];
    let mut q = partie(derniere, "rouge-9-a", Couleur::Rouge, actif(), &[]);
    q.jouer("a", avec_cible("rouge-7-a", "b")).unwrap();
    assert_eq!((q.gagnant, q.joueurs[1].main.len()), (Some(0), 2));
}
