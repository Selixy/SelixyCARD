mod uno_aide;
use commun::uno::{Couleur, Erreur, Options};
use uno_aide::*;

const MAINS: &[&[&str]] = &[&["rouge-5-a", "bleu-1-a"], &["jaune-2-a", "jaune-3-a"], &["vert-1-a", "vert-2-a"]];

#[test]
fn oubli_signale_a_temps_donne_deux_cartes() {
    let mut p = partie(MAINS, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.jouer("a", pose("rouge-5-a")).unwrap();
    assert_eq!(p.signalable, Some(0));
    assert_eq!(p.signaler("a", "a"), Err(Erreur::SignalementInvalide));
    assert_eq!(p.signaler("b", "c"), Err(Erreur::SignalementInvalide));
    p.signaler("b", "a").unwrap();
    assert_eq!((p.joueurs[0].main.len(), p.signalable), (3, None));
    assert_eq!(p.signaler("c", "a"), Err(Erreur::SignalementInvalide));
}

#[test]
fn uno_dit_a_temps_protege() {
    let mut p = partie(MAINS, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.dire_uno("a").unwrap();
    p.jouer("a", pose("rouge-5-a")).unwrap();
    assert_eq!(p.signalable, None);
    let mut q = partie(MAINS, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    q.jouer("a", pose("rouge-5-a")).unwrap();
    q.dire_uno("a").unwrap();
    assert_eq!(q.signaler("b", "a"), Err(Erreur::SignalementInvalide));
    assert!(q.joueurs[0].uno);
}

#[test]
fn signalement_trop_tard_apres_le_coup_suivant() {
    let mut p = partie(MAINS, "rouge-9-a", Couleur::Rouge, Options::default(), &["vert-3-a"]);
    p.jouer("a", pose("rouge-5-a")).unwrap();
    p.piocher("b").unwrap();
    assert_eq!(p.signalable, None);
    assert_eq!(p.signaler("c", "a"), Err(Erreur::SignalementInvalide));
    p.joueurs[2].main.extend(["rouge-1-a", "rouge-2-a"].map(carte));
    assert_eq!(p.dire_uno("c"), Err(Erreur::TropDeCartes));
}

#[test]
fn penalite_uno_desactivee() {
    let options = Options { penalite_uno: false, ..Options::default() };
    let mut p = partie(MAINS, "rouge-9-a", Couleur::Rouge, options, &[]);
    p.jouer("a", pose("rouge-5-a")).unwrap();
    assert_eq!(p.signalable, None);
    assert_eq!(p.signaler("b", "a"), Err(Erreur::OptionDesactivee));
    p.dire_uno("a").unwrap();
    p.tirer(0, 1);
    assert!(!p.joueurs[0].uno);
}
