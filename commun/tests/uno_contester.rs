mod uno_aide;
use commun::uno::{Attente, Couleur, Erreur, Options};
use uno_aide::*;

const MAINS: &[&[&str]] = &[&["joker_plus4-1", "rouge-5-a", "bleu-1-a"], &["jaune-1-a", "jaune-2-a"], &["vert-1-a", "vert-2-a"]];

#[test]
fn bluff_decouvert_le_poseur_pioche() {
    let mut p = partie(MAINS, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.jouer("a", avec_couleur("joker_plus4-1", Couleur::Bleu)).unwrap();
    assert_eq!((p.penalite, p.courant), (4, 1));
    assert_eq!(p.attente, Attente::Contestation { poseur: 0, avait_couleur: true });
    assert_eq!(p.contester("c"), Err(Erreur::PasTonTour));
    p.contester("b").unwrap();
    assert_eq!((p.joueurs[0].main.len(), p.joueurs[1].main.len()), (6, 2));
    assert_eq!((p.courant, p.penalite, p.attente), (1, 0, Attente::Aucune));
    assert_eq!(p.contester("b"), Err(Erreur::RienAContester));
}

#[test]
fn contestation_ratee_le_contestataire_pioche_six() {
    let mains: &[&[&str]] = &[&["joker_plus4-1", "bleu-1-a"], &["jaune-1-a", "jaune-2-a"], &["vert-1-a", "vert-2-a"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.jouer("a", avec_couleur("joker_plus4-1", Couleur::Bleu)).unwrap();
    assert_eq!(p.attente, Attente::Contestation { poseur: 0, avait_couleur: false });
    p.contester("b").unwrap();
    assert_eq!((p.joueurs[1].main.len(), p.joueurs[0].main.len()), (8, 1));
    assert_eq!((p.courant, p.penalite, p.couleur), (2, 0, Couleur::Bleu));
}

#[test]
fn accepter_le_plus_quatre_sans_contester() {
    let mut p = partie(MAINS, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.jouer("a", avec_couleur("joker_plus4-1", Couleur::Bleu)).unwrap();
    assert_eq!(p.jouer("b", pose("jaune-1-a")), Err(Erreur::PenaliteEnCours));
    p.piocher("b").unwrap();
    assert_eq!((p.joueurs[1].main.len(), p.courant, p.attente), (6, 2, Attente::Aucune));
    assert_eq!(p.contester("c"), Err(Erreur::RienAContester));
}

#[test]
fn contestation_desactivee_penalite_immediate() {
    let options = Options { contester_plus4: false, ..Options::default() };
    let mut p = partie(MAINS, "rouge-9-a", Couleur::Rouge, options, &[]);
    assert_eq!(p.jouer("a", avec_couleur("joker_plus4-1", Couleur::Bleu)), Err(Erreur::CarteInjouable));
    p.joueurs[0].main.remove(1);
    p.jouer("a", avec_couleur("joker_plus4-1", Couleur::Bleu)).unwrap();
    assert_eq!((p.attente, p.penalite, p.joueurs[1].main.len(), p.courant), (Attente::Aucune, 0, 6, 2));
    assert_eq!(p.contester("c"), Err(Erreur::RienAContester));
}

#[test]
fn contestation_activee_le_bluff_est_permis() {
    let mut p = partie(MAINS, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    assert_eq!(p.jouer("a", avec_couleur("joker_plus4-1", Couleur::Jaune)), Ok(()));
    assert_eq!(p.couleur, Couleur::Jaune);
}
